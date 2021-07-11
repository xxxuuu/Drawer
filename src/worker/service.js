import event from '@/utils/event-topic';
import db from './db';

const { ipcRenderer, remote } = window.require('electron');

const tagService = {
  /** 新增标签 返回插入到数据库中的标签数据  */
  addTag(tagName) {
    return db.storeTag(tagName);
  },
  /** 获取所有标签 */
  getAllTag() {
    return db.getAllTag();
  },
  /** 获取指定标签下的所有剪贴板 */
  getClipboardByTag(tagId) {
    return db.getClipboardByTag(tagId);
  },
  /** 将剪贴板钉到指定标签中 */
  storeClipboard2Tag(cardData, tagId) {
    return db.storeClipboard2Tag(tagId, cardData);
  },
};

const eventServiceMap = {
  [event.ADD_TAG]: tagService.addTag,
  [event.GET_ALL_TAG]: tagService.getAllTag,
  [event.GET_CLIPBOARD_BY_TAG]: tagService.getClipboardByTag,
  [event.STORE_CLIPBOARD_TO_TAG]: tagService.storeClipboard2Tag,
};

export default {
  registerService() {
    // 注册监听事件
    // eslint-disable-next-line guard-for-in,no-restricted-syntax
    for (const eventName in eventServiceMap) {
      console.log(`register service ${eventName}...`);
      ipcRenderer.on(eventName, async (...args) => {
        // 调用事件对应方法并返回
        const resp = await eventServiceMap[eventName](...(args.slice(1)));
        ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, `${eventName}-resp`, resp);
      });
    }
  },
};
