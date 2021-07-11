import event from '@/utils/event-topic';
import db from './db';
import service from './service';

const path = window.require('path');
const { exec } = window.require('child_process');
const fs = window.require('fs');

const { clipboard, ipcRenderer, remote } = window.require('electron');

/** 剪贴板更新 提取类型 插入数据库 */
async function updateClipboard() {
  const formats = clipboard.availableFormats();

  const timestamp = Date.now();
  const info = {};

  let filePath = clipboard.read('public.file-url').replace('file://', '');
  if (filePath) {
    filePath = decodeURI(filePath);
    // 文件
    info.data = filePath;
    info.type = 'file';
    const previewImg = clipboard.readImage('png');
    if (previewImg.isEmpty()) {
      // TODO: 获取预览图在某些类型的文件上不生效 只能先获取图标
      info.preview = (await remote.app.getFileIcon(filePath)).toDataURL();
    } else {
      info.preview = previewImg.toDataURL();
    }
    info.description = filePath;
  } else if (formats.indexOf('text/rtf') >= 0) {
    // RTF 富文本
    info.data = {
      rtf: clipboard.readRTF(),
      text: clipboard.readText(),
    };
    info.type = 'rtf';
    info.description = `${info.data.text.length} 个字符`;
  } else if (formats.indexOf('image/png') >= 0 && !clipboard.readImage().isEmpty()) {
    // 图片
    const img = clipboard.readImage();
    const size = img.getSize();
    info.data = img.toDataURL();
    info.type = 'image';
    info.description = `${size.width} × ${size.height} 像素`;
  } else {
    const text = clipboard.readText();
    if (text === '') {
      return;
    }

    const colorPattern = /^#[0-9A-Fa-f]{6}$/;
    const urlPattern = /^(http|https):\/\/.+/;
    if (text.search(colorPattern) >= 0) {
      // 颜色
      info.data = text;
      info.type = 'color';
      info.description = '';
    } else if (text.search(urlPattern) >= 0) {
      // 链接
      info.data = text;
      info.type = 'url';
      info.description = '';
    } else {
      // 普通文本
      info.data = text;
      info.type = 'text';
      info.description = `${text.length} 个字符`;
    }
  }

  info.time = timestamp;

  // 插入成功就通知前端更新
  const success = await db.storeClipboard(info);
  if (success) {
    ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.APPEND, info);
  }
}

/** 开启子进程监听剪贴板 */
function nativeListen() {
  // eslint-disable-next-line no-undef
  const listenPath = path.join(__static, 'clipboard-listen');
  try {
    fs.accessSync(listenPath, fs.constants.X_OK);
  } catch {
    fs.chmodSync(listenPath, 0o775);
  }
  const childProcess = exec(listenPath);
  console.log(listenPath);
  childProcess.stdout.on('data', (data) => {
    console.log(data);
    updateClipboard();
  });
  remote.app.on('before-quit', () => {
    childProcess.kill();
  });
}

// 第一次首先获取数据库所有数据 然后开始监听
db.getAllClipboard().then((res) => {
  ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.INIT, res);
  nativeListen();
});
db.clearOutdatedClipboard();
service.registerService();
