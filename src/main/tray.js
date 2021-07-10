import {
  app, Tray, Menu, dialog, nativeImage, shell,
} from 'electron';
import path from 'path';

let trayInstance = null;

export default {
  // 创建托盘
  createTray() {
    if (trayInstance) {
      return;
    }
    // eslint-disable-next-line no-undef
    trayInstance = new Tray(path.join(__static, 'tray_icon.png'));
    const contextMenu = Menu.buildFromTemplate([
      {
        label: '关于 Drawer',
        type: 'normal',
        click: () => {
          dialog.showMessageBox({
            title: 'Drawer',
            message: 'Drawer',
            detail: `${app.getVersion()}\n\nDrawer是一个macOS上的剪贴板应用`,
            // eslint-disable-next-line no-undef
            icon: nativeImage.createFromPath(path.join(__static, 'icon_512x512.png')),
            buttons: ['Github', '好'],
          }).then((clickIdx) => {
            if (clickIdx.response === 0) {
              shell.openExternal('https://github.com/xxxuuu/Drawer');
            }
          });
        },
      },
      { type: 'separator' },
      { label: '退出 Drawer', type: 'normal', click: app.quit },
    ]);
    trayInstance.setContextMenu(contextMenu);
  },
};
