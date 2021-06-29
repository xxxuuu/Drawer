import {
  app, protocol,
  BrowserWindow, globalShortcut,
  ipcMain, screen, Tray, Menu,
  dialog, nativeImage, shell,
} from 'electron';
import { createProtocol } from 'vue-cli-plugin-electron-builder/lib';
import installExtension, { VUEJS_DEVTOOLS } from 'electron-devtools-installer';
import path from 'path';
import event from './event-topic';

const isDevelopment = process.env.NODE_ENV !== 'production';

// Scheme must be registered before the app is ready
protocol.registerSchemesAsPrivileged([
  { scheme: 'app', privileges: { secure: true, standard: true } },
]);

global.winId = {
  mainWindow: null,
};

// worker进程 通过隐藏窗口实现
async function createWorker() {
  const worker = new BrowserWindow({
    show: isDevelopment,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
      enableRemoteModule: true,
    },
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    await worker.loadURL(`${process.env.WEBPACK_DEV_SERVER_URL}worker.html`);
    // if (!process.env.IS_TEST) worker.webContents.openDevTools();
  } else {
    createProtocol('app');
    worker.loadURL('app://./worker.html');
  }
}

// 主窗口实例
let mainWindow = null;
let tray = null;
async function createWindow() {
  const screenSize = screen.getPrimaryDisplay().size;
  const screenWidth = screenSize.width;
  const screenHeight = screenSize.height;
  const height = 350;
  const windowY = screenHeight - height;

  if (mainWindow) {
    // 令其能显示在当前桌面（工作区）：https://github.com/electron/electron/issues/5362
    mainWindow.setVisibleOnAllWorkspaces(true);
    mainWindow.show();
    mainWindow.setSize(screenWidth, height);
    mainWindow.setPosition(0, windowY, true);
    mainWindow.setVisibleOnAllWorkspaces(false);
    return;
  }

  // Create the browser window.
  const win = new BrowserWindow({
    width: screenWidth,
    height,
    x: 0,
    y: windowY,
    frame: false,
    movable: false,
    show: false,
    resizable: false,
    alwaysOnTop: true,
    webPreferences: {
      nodeIntegration: true,
      enableRemoteModule: true,
      webSecurity: true,
      experimentalFeatures: true,
      contextIsolation: false,
    },
  });

  mainWindow = win;
  global.winId.mainWindow = win.id;
  win.on('blur', () => {
    win.hide();
    app.dock.hide();
  });
  win.setAlwaysOnTop(true, 'pop-up-menu');
  win.setPosition(0, windowY, true);

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    await win.loadURL(process.env.WEBPACK_DEV_SERVER_URL);
    // if (!process.env.IS_TEST) win.webContents.openDevTools();
  } else {
    createProtocol('app');
    win.loadURL('app://./index.html');
  }
}

// 隐藏docker栏图标
app.dock.hide();
// 开机自启
app.setLoginItemSettings({
  openAtLogin: !isDevelopment,
  openAsHidden: true,
});
// 初始化&监听剪贴板
app.whenReady().then(() => {
  ipcMain.on(event.LOG, (e, args) => {
    console.log(args);
  });

  // 创建托盘
  // eslint-disable-next-line no-undef
  tray = new Tray(path.join(__static, 'tray_icon.png'));
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
  tray.setContextMenu(contextMenu);

  createWorker();
  // 全局快捷键 弹出窗口
  globalShortcut.register('Shift+CommandOrControl+V', () => {
    createWindow();
  });
});

app.on('ready', async () => {
  if (isDevelopment && !process.env.IS_TEST) {
    // Install Vue Devtools
    try {
      await installExtension(VUEJS_DEVTOOLS);
    } catch (e) {
      console.error('Vue Devtools failed to install:', e.toString());
    }
  }
  createWindow();
});

// Exit cleanly on request from parent process in development mode.
if (isDevelopment) {
  if (process.platform === 'win32') {
    process.on('message', (data) => {
      if (data === 'graceful-exit') {
        app.quit();
      }
    });
  } else {
    process.on('SIGTERM', () => {
      app.quit();
    });
  }
}
