import { openDB } from 'idb';
import event from '@/utils/event-topic';

const { ipcRenderer, remote } = window.require('electron');

// const path = require('path');
// const os = require('os');
// const fs = require('fs');

// 数据库和缓存文件存放目录
// const dbPath = path.join(os.homedir(), '.drawer-clipboard', 'db');
// const imgCachePath = path.join(dbPath, 'img-cache');
// if (!fs.existsSync(imgCachePath)) {
//   fs.mkdirSync(imgCachePath, { recursive: true });
// }

const DB_NAME = 'drawer';
const STORE_NAME = 'clipboard';
const db = openDB(DB_NAME, 1, {
  upgrade(d) {
    d.createObjectStore(STORE_NAME, {
      keyPath: 'time',
      autoIncrement: true,
    });
  },
});

async function getAll() {
  return (await db).getAll(STORE_NAME);
}

async function getLast() {
  const cursor = await (await db).transaction(STORE_NAME).store.openCursor(null, 'prev');

  if (cursor) {
    return cursor.value;
  }
  return null;
}

async function store(value) {
  // 和最后一个是否重复
  const last = await getLast();

  if (last && last.data === value.data) {
    // 返回插入失败
    return false;
  }
  // 插入成功
  (await db).add(STORE_NAME, value);
  return true;
}

// 清理过期
async function clearOutdated() {
  const timestamp = Date.parse(new Date());
  // 一天的毫秒数
  const day = 1000 * 60 * 60 * 24;
  // time作为主键，小于这个范围的全部删掉
  const delLen = (await (await db).getAll(STORE_NAME,
    IDBKeyRange.upperBound(timestamp - day))).length;
  await (await db).delete(STORE_NAME, IDBKeyRange.upperBound(timestamp - day));
  // 需要通知前端删除列表UI上的相关部分
  ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.DELETE_OLD, delLen);
  setTimeout(clearOutdated, 10000);
}
clearOutdated();

export default { getAll, getLast, store };
