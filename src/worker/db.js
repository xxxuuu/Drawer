import { openDB } from 'idb';
import event from '@/utils/event-topic';

const { ipcRenderer, remote } = window.require('electron');

const DB_NAME = 'drawer';
const CLIPBOARD_STORE_NAME = 'clipboard';
const TAG_STORE_NAME = 'tag';
const TAG_CLIPBOARD_STORE_NAME = 'tag-clipboard';
const db = openDB(DB_NAME, 2, {
  upgrade(d, oldVersion) {
    if (oldVersion < 1) {
      d.createObjectStore(CLIPBOARD_STORE_NAME, {
        keyPath: 'time',
        autoIncrement: true,
      });
    }
    if (oldVersion < 2) {
      const tagStore = d.createObjectStore(TAG_STORE_NAME, {
        keyPath: 'id',
        autoIncrement: true,
      });
      tagStore.createIndex('uk_name', 'name', { unique: true });

      const tagClipboardStore = d.createObjectStore(TAG_CLIPBOARD_STORE_NAME, {
        keyPath: 'id',
        autoIncrement: true,
      });
      tagClipboardStore.createIndex('idx_tag_id', 'tagId');
    }
  },
});

export default {
  /** 获取所有历史剪贴板数据 */
  async getAllClipboard() {
    return (await db).getAll(CLIPBOARD_STORE_NAME);
  },
  /** 获取最近一个历史剪贴板 */
  async getLastClipboard() {
    const cursor = await (await db).transaction(CLIPBOARD_STORE_NAME).store.openCursor(null, 'prev');

    if (cursor) {
      return cursor.value;
    }
    return null;
  },
  /** 存储剪贴板数据 */
  async storeClipboard(value) {
    // 和最后一个是否重复
    const last = await this.getLastClipboard();

    if (last && last.data === value.data) {
      // 返回插入失败
      return false;
    }
    // 插入成功
    (await db).add(CLIPBOARD_STORE_NAME, value);
    return true;
  },
  /** 清理过期(24h)剪贴板 */
  async clearOutdatedClipboard() {
    const timestamp = Date.parse(new Date());
    // 一天的毫秒数
    const day = 1000 * 60 * 60 * 24;
    // time作为主键，小于这个范围的全部删掉
    const delLen = (await (await db).getAll(CLIPBOARD_STORE_NAME,
      IDBKeyRange.upperBound(timestamp - day))).length;
    await (await db).delete(CLIPBOARD_STORE_NAME, IDBKeyRange.upperBound(timestamp - day));
    // 需要通知前端删除列表UI上的相关部分
    ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.DELETE_OLD, delLen);
    setTimeout(this.clearOutdatedClipboard, 10000);
  },
  /** 获取所有标签 */
  async getAllTag() {
    return (await db).getAll(TAG_STORE_NAME);
  },
  /** 存储标签 */
  async storeTag(tagName) {
    const tagData = {
      name: tagName,
    };
    const key = await (await db).add(TAG_STORE_NAME, tagData);
    tagData.id = key;
    return tagData;
  },
  /** 删除标签 */
  async deleteTag(tagId) {
    (await db).delete(TAG_STORE_NAME, tagId);
    // 删除这个标签的剪贴板数据
    const tx = (await db).transaction(TAG_CLIPBOARD_STORE_NAME, 'readwrite');
    const idx = tx.store.index('idx_tag_id');
    let cursor = await idx.openCursor(IDBKeyRange.only(tagId));
    while (cursor) {
      cursor.delete();
      /* eslint-disable no-await-in-loop */
      cursor = await cursor.continue();
    }
    await tx.done;
  },
  /** 把剪贴板钉到某个标签里 */
  async storeClipboard2Tag(tagId, clipboardVal) {
    const val = clipboardVal;
    val.tagId = tagId;
    delete val.id;
    (await db).add(TAG_CLIPBOARD_STORE_NAME, val);
  },
  /** 获取某个标签下钉起来的剪贴板 */
  async getClipboardByTag(tagId) {
    const result = [];

    const tx = (await db).transaction(TAG_CLIPBOARD_STORE_NAME, 'readonly');
    const idx = tx.store.index('idx_tag_id');
    let cursor = await idx.openCursor(IDBKeyRange.only(tagId));
    while (cursor) {
      result.push(cursor.value);
      /* eslint-disable no-await-in-loop */
      cursor = await cursor.continue();
    }
    await tx.done;
    return result;
  },
};
