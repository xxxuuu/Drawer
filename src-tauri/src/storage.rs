use std::{path::{Path, PathBuf}, sync::{Mutex, mpsc::{self, Receiver, Sender}, Arc}, thread, time, fmt, result, rc::Rc};

use chrono::Local;
use rusqlite::{Connection, params, Row, types::Value};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::{clipboard::{ClipboardContent, ClipboardFormatContent}, event::Topic};

/// 包装来自上游的错误
#[derive(Debug)]
pub enum StorageError {
    Serde(serde_json::Error),
    Sqlite(rusqlite::Error)
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StorageError {}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        StorageError::Serde(value)
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(value: rusqlite::Error) -> Self {
        StorageError::Sqlite(value)
    }
}

pub type Result<T> = result::Result<T, StorageError>;

/// 剪贴板记录
#[derive(Serialize, Clone, Debug)]
pub struct ClipboardRecord {
    pub id: i64,
    pub main_data: String,
    pub data: String,
    pub content_type: String,
    pub time: i64,
}

/// ClipboardContent 转 ClipboardRecord
impl TryFrom<ClipboardContent> for ClipboardRecord {
    type Error = serde_json::Error;

    fn try_from(value: ClipboardContent) -> result::Result<Self, Self::Error> {
        Ok(ClipboardRecord {
            id: 0,
            content_type: match value.main_format {
                ClipboardFormatContent::Text(_) => "text",
                ClipboardFormatContent::RTF(_) => "rtf",
                ClipboardFormatContent::Image(_) => "image",
                ClipboardFormatContent::Files(_) => "files",
            }.to_string(),
            main_data: serde_json::to_string(&(value.main_format.clone()))?,
            data: serde_json::to_string(&value)?,
            time: Local::now().timestamp_millis(),
        })
    }
}

impl TryFrom<ClipboardRecord> for ClipboardContent {
    type Error = serde_json::Error;

    fn try_from(record: ClipboardRecord) -> result::Result<Self, Self::Error> {
        Ok(serde_json::from_str(record.data.as_str())?)
    }
}

impl ClipboardRecord {
    fn parse(row: &Row) -> result::Result<ClipboardRecord, rusqlite::Error> {
        Ok(ClipboardRecord {
            id: row.get(0)?,
            main_data: row.get(1)?,
            data: row.get(2)?,
            content_type: row.get(3)?,
            time: row.get(4)?,
        })
    }
}

/// 标签记录
#[derive(Serialize, Clone, Debug)]
pub struct TagRecord {
    id: i64,
    name: String
}

impl TagRecord {
    fn parse(row: &Row) -> result::Result<TagRecord, rusqlite::Error> {
        Ok(TagRecord {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}

/// 数据库连接
#[derive(Debug)]
pub struct StorageConn {
    app_handler: AppHandle,
    db: Arc<Mutex<Connection>>,
    gc_close_handle: Mutex<Sender<()>>,
}

impl Drop for StorageConn {
    fn drop(&mut self) {
        self.gc_close_handle.lock().unwrap().send(()).unwrap();
    }
}

impl StorageConn {
    pub fn new(data_dir: &PathBuf, app: AppHandle) -> Result<StorageConn> {
        let db_dir = Path::join(data_dir.as_path(), "drawer.db");
        let db = Connection::open(db_dir)?; 
        let (tx, rx) = mpsc::channel();
        let conn = StorageConn { 
            app_handler: app,
            db: Arc::new(Mutex::new(db)),
            gc_close_handle: Mutex::new(tx),
        };
        {
            rusqlite::vtab::array::load_module(&conn.db.clone().lock().unwrap())?;
        }
        conn.init_table().expect("failed to initialization database table");
        conn.gc_launch(rx);
        Ok(conn)
    }

    /// 初始化表结构
    fn init_table(&self) -> Result<()> {
        let db = self.db.lock().unwrap();
        db.execute("
            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT
            );
        ", ())?;
        db.execute("INSERT OR IGNORE INTO tags (id, name) VALUES (0, '📝 剪贴板历史');", ())?;
        db.execute("
            CREATE TABLE IF NOT EXISTS clipboard (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                main_data TEXT,
                data TEXT,
                content_type TEXT,
                create_at INTERGER
            );
            CREATE INDEX IF NOT EXISTS idx_create_at ON clipboard (create_at DESC);
        ", ())?;
        db.execute("
            CREATE TABLE IF NOT EXISTS clipboard_tags (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                clipboard_id INTERGER,
                tag_id INTERGER
            );
            CREATE INDEX IF NOT EXISTS idx_cid ON clipboard_tags (clipboard_id);
            CREATE INDEX IF NOT EXISTS idx_tid ON clipboard_tags (tag_id);
        ", ())?;
        Ok(())
    }

    /// 启动GC，定期删除过期数据
    fn gc_launch(&self, close_handle: Receiver<()>) { 
        let db = self.db.clone();
        let app_handler = self.app_handler.app_handle();
        thread::spawn(move || loop {
            // drop 时退出线程
            match close_handle.try_recv() {
                Ok(()) | Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                },
                _ => {}
            }

            // 剪贴板历史只保留100条
            match Self::delete_outdated_record(db.clone(), 100) {
                Ok(ids) => {
                    // TODO: 这里通知应该用回调抽出去，依赖进来个app handler不太好
                    app_handler.emit_all(Topic::CLIPBOARD_DELETE, ids).unwrap();
                },
                Err(err) => println!("failed to delete outdated record: {}", err.to_string())
            }

            thread::sleep(time::Duration::from_secs(10));
        });
    }

    /// 只保留n条数据，删除剩余的
    fn delete_outdated_record(_db: Arc<Mutex<Connection>>, n: usize) -> Result<Vec<i64>> {
        // 先获取要被删除的记录 返回用
        let delete_ids = {
            let db = _db.lock().unwrap();
            let mut stmt = db.prepare("
                    SELECT c.id FROM clipboard c INNER JOIN clipboard_tags ct
                    ON c.id = ct.clipboard_id AND ct.tag_id = 0 
                    LIMIT MAX(0, ( 
                        SELECT COUNT(c.id) FROM clipboard c INNER JOIN clipboard_tags ct 
                        ON c.id = ct.clipboard_id AND ct.tag_id = 0
                    ) - ?1)
                ")?;
            let rows = stmt.query_map(params![n], |row| row.get(0))?;
            let mut delete_ids: Vec<i64> = Vec::new();
            for id in rows {
                delete_ids.push(id?);
                if delete_ids.len() >= n {
                    break;
                }
            }
            delete_ids
        };

        let mut db = _db.lock().unwrap();
        let tx = db.transaction()?;
        // 只删除剪贴板历史的记录
        let ids = Rc::new(delete_ids.iter().copied().map(Value::from).collect::<Vec<Value>>());
        tx.execute("DELETE FROM clipboard_tags WHERE clipboard_id IN rarray(?1)", &[&ids])?;
        tx.execute("DELETE FROM clipboard WHERE id IN rarray(?1)", &[&ids])?;
        tx.commit()?;
        Ok(delete_ids)
    }

    /// 获取所有标签
    pub fn get_tags(&self) -> Result<Vec<TagRecord>> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT * FROM tags;")?;
        let rows = stmt.query_map(params![], TagRecord::parse)?;
        let mut ret = Vec::new();
        for r in rows {
            ret.push(r?);
        }
        Ok(ret)
    }

    /// 创建一个标签
    pub fn insert_tag(&self, name: String) -> Result<TagRecord> {
        let db = self.db.lock().unwrap();
        db.execute("INSERT INTO tags (name) VALUES (?1);", params![name])?;
        
        let ret = db.prepare("SELECT * FROM tags WHERE id = ?1")?.
                    query_row(params![db.last_insert_rowid()], TagRecord::parse)?;
        Ok(ret)
    }

    /// 删除标签
    pub fn delete_tag(&self, id: i64) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        // 所属该标签下的记录也要删除
        let tx = db.transaction()?;
        tx.execute("DELETE FROM tags WHERE id = ?1;", params![id])?;
        tx.execute("DELETE FROM clipboard WHERE id IN (SELECT clipboard_id FROM clipboard_tags WHERE tag_id = ?1);", params![id])?;
        tx.execute("DELETE FROM clipboard_tags WHERE tag_id = ?1;", params![id])?;
        tx.commit()?;
        Ok(())
    }

    /// 获取某个标签下的所有记录
    pub fn get_records_with_tag(&self, tag_id: i64) -> Result<Vec<ClipboardRecord>> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("
            SELECT c.* FROM clipboard c
            INNER JOIN clipboard_tags ct 
            ON ct.tag_id = ?1 AND ct.clipboard_id = c.id;"
        )?;
        let rows = stmt.query_map(params![tag_id], ClipboardRecord::parse)?;
        let mut ret = Vec::new();
        for r in rows {
            ret.push(r.unwrap());
        }
        Ok(ret)
    }

    /// 获取一条记录
    pub fn get_record(&self, id: i64) -> Result<ClipboardRecord> {
        let db = self.db.lock().unwrap();
        let ret = db.prepare("SELECT * FROM clipboard WHERE id = ?1")?.
                    query_row(params![id], ClipboardRecord::parse)?;
        Ok(ret)
    }

    /// 插入一条新纪录
    pub fn insert_record(&self, c: ClipboardRecord) -> Result<ClipboardRecord> {
        self.insert_record_with_tag(c, 0)
    }

    /// 在某个标签下插入一条新纪录
    pub fn insert_record_with_tag(&self, record: ClipboardRecord, tag_id: i64) -> Result<ClipboardRecord> {
        let last_insert_id = {
            let mut db = self.db.lock().unwrap();

            let tx = db.transaction()?;
            tx.execute("INSERT INTO clipboard (main_data, data, content_type, create_at) VALUES (?1, ?2, ?3, ?4);", 
                params![record.main_data, record.data, record.content_type, record.time])?;
            tx.execute("INSERT INTO clipboard_tags (clipboard_id, tag_id) VALUES (?1, ?2);", 
                params![tx.last_insert_rowid(), tag_id])?;
            tx.commit()?;

            db.last_insert_rowid()
        };
        self.get_record(last_insert_id)
    }

    /// 复制一条记录到另一个标签下
    pub fn copy_record_to_tag(&self, record_id: i64, tag_id: i64) -> Result<()> {
        self.insert_record_with_tag(self.get_record(record_id)?, tag_id)?;
        Ok(())
    }

    /// 删除一条记录
    pub fn delete_record(&self, id: i64) -> Result<()> {
        let mut db = self.db.lock().unwrap();

        let tx = db.transaction()?;
        tx.execute("DELETE FROM clipboard WHERE id=?1;", params![id])?;
        tx.execute("DELETE FROM clipboard_tags WHERE clipboard_id=?1;", params![id])?;
        tx.commit()?;

        Ok(())
    }
}
