use serde_json::json;
use tauri::{AppHandle, Manager};

use crate::{storage::{StorageConn, ClipboardRecord, TagRecord}, clipboard::{ClipboardManager, ClipboardContent, ClipboardFormatContent}, event::Topic, util::{self, Data}};

pub type ClipboardRecordVO = serde_json::Value;
pub type CmdResult<T> = Result<T, String>;

impl TryFrom<ClipboardRecord> for ClipboardRecordVO {
    type Error = String;

    fn try_from(record: ClipboardRecord) -> Result<Self, Self::Error> {
        let id = record.id;
        let create_at = record.time;
        let content: ClipboardFormatContent = serde_json::from_str(record.main_data.as_str())
            .map_err(|err| err.to_string())?;
        match content {
            ClipboardFormatContent::Text(text) => Ok(json!({
                "id": id,
                "type": "text",
                "time": create_at,
                "description": format!("{} 个字符", text.chars().count()),
                "data": text
            })),
            ClipboardFormatContent::RTF(rtf) => {
                let rtf_str = String::from_utf8_lossy(rtf.as_slice()).to_string();
                let rtf_char_count = rtf_str.chars().count();
                Ok(json!({
                    "id": id,
                    "type": "rtf",
                    "time": create_at,
                    "data": json!({
                        "rtf": base64::encode(rtf),
                        "text": rtf_char_count,
                    }),
                    "description": format!("{} 个字符", rtf_char_count),
                }))
            },
            ClipboardFormatContent::Image(data) => {
                let size = util::ImageUtil::img_size(data.as_slice())
                    .map_err(|err| err.to_string())?;
                Ok(json!({
                    "id": id,
                    "type": "image",
                    "time": create_at,
                    "data": base64::encode(data),
                    "description": format!("{} × {}", size.width, size.height)
                }))
            },
            ClipboardFormatContent::Files(file_urls) => Ok(json!({
                "id": id,
                "type": "file",
                "time": create_at,
                "data": file_urls[0],
                "description": file_urls[0],
                "thumbnail": match util::QuickLook::thumbnail(&(file_urls[0])) {
                    Ok(data) => match data {
                        Data::Raw(bytes) => base64::encode(bytes),
                        Data::Base64(base64_str) => base64_str
                    },
                    Err(_) => "".to_string()
                }
            })),
        }      
    }
}

/// 获取所有标签
#[tauri::command]
pub fn get_all_tags(app: AppHandle) -> CmdResult<Vec<TagRecord>> {
    let db = app.state::<StorageConn>();
    db.inner().get_tags().map_err(|err| err.to_string())
}

/// 获取指定标签下所有剪贴板记录
#[tauri::command]
pub fn get_all_record(app: AppHandle, tag_id: i64) -> CmdResult<Vec<ClipboardRecordVO>> {
    let db = app.state::<StorageConn>();
    match db.inner().get_records_with_tag(tag_id) {
        Ok(records) => {
            let mut ret = Vec::with_capacity(records.len());
            for record in records {
                match TryInto::<ClipboardRecordVO>::try_into(record) {
                    Ok(vo) => ret.push(vo),
                    Err(err) => return Err(err.to_string())
                }
            }
            Ok(ret)
        }
        Err(err) => Err(err.to_string())
    }
}

/// 删除剪贴板记录
#[tauri::command]
pub fn delete_record(app: AppHandle, id: i64) -> Result<(), String> {
    let db = app.state::<StorageConn>();
    match db.inner().delete_record(id) {
        Ok(()) => {
            app.emit_all(Topic::CLIPBOARD_DELETE, vec![id]).unwrap();
            Ok(())
        }
        Err(err) => Err(err.to_string())
    }
}

/// 将一条剪贴板记录钉到标签上
#[tauri::command]
pub fn pin_record(app: AppHandle, record_id: i64, tag_id: i64) -> CmdResult<()> {
    let db = app.state::<StorageConn>();
    println!("pin {} to {}", record_id, tag_id);
    db.inner().copy_record_to_tag(record_id, tag_id).map_err(|err| err.to_string())
}

/// 创建标签
#[tauri::command]
pub fn create_tag(app: AppHandle, name: &str) -> CmdResult<TagRecord> {
    let db = app.state::<StorageConn>();
    db.inner().insert_tag(name.to_string()).map_err(|err| err.to_string())
}

/// 删除标签
#[tauri::command]
pub fn delete_tag(app: AppHandle, id: i64) -> CmdResult<()> {
    if id == 0 {
        return Err("剪贴板历史不能删除".to_string());
    }
    let db = app.state::<StorageConn>();
    db.inner().delete_tag(id).map_err(|err| err.to_string())
}

/// 粘贴一条剪贴板记录的内容
#[tauri::command]
pub fn paste(app: AppHandle, id: i64) -> CmdResult<()> {
    let db = app.state::<StorageConn>();
    let clipboard = app.state::<ClipboardManager>();
    match db.inner().get_record(id) {
        Ok(record) => {
            let content: ClipboardContent = match record.try_into() {
                Ok(_content) => _content,
                Err(err) => return Err(err.to_string())
            };
            clipboard.inner().paste(content)?;
        }
        Err(err) => return Err(err.to_string())
    }
    Ok(())
}