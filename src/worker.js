import event from './event-topic';
import db from './db';

const { clipboard, ipcRenderer, remote } = window.require('electron');

async function listen() {
  const formats = clipboard.availableFormats();
  // ipcRenderer.send(event.LOG, formats);

  const timestamp = Date.parse(new Date());
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
    info.data = clipboard.readRTF();
    info.type = 'rtf';
    info.description = `${clipboard.readText().length} 个字符`;
  } else if (formats.indexOf('image/png') >= 0 && !clipboard.readImage().isEmpty()) {
    // 图片
    const img = clipboard.readImage();
    const size = img.getSize();
    info.data = img.toDataURL();
    info.type = 'image';
    info.description = `${size.width} × ${size.height} 像素`;
  } else {
    const colorPattern = /^#[0-9A-Fa-f]{6}$/;
    const urlPattern = /^(http|https):\/\/.+/;
    const text = clipboard.readText();

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
  db.store(info).then((res) => {
    if (res) {
      ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.APPEND, info);
    }
  });

  setTimeout(listen, 1000);
}

// 第一次首先获取数据库所有数据 然后开始监听
db.getAll().then((res) => {
  ipcRenderer.sendTo(remote.getGlobal('winId').mainWindow, event.INIT, res);
  setTimeout(listen, 1000);
});
