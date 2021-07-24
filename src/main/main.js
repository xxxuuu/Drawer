import {
  app, protocol, BrowserWindow, globalShortcut, screen,
} from 'electron';
import { createProtocol } from 'vue-cli-plugin-electron-builder/lib';
import installExtension, { VUEJS_DEVTOOLS } from 'electron-devtools-installer';
import tray from './tray';

const isDevelopment = process.env.NODE_ENV !== 'production';

// Scheme must be registered before the app is ready
protocol.registerSchemesAsPrivileged([
  { scheme: 'app', privileges: { secure: true, standard: true } },
]);

global.winId = {
  mainWindow: null,
  worker: null,
};
// 标志值 防止从历史剪贴板中复制时又被当做新的数据添加进来
global.sync = {
  flag: 0,
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

  global.winId.worker = worker.id;

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    await worker.loadURL(`${process.env.WEBPACK_DEV_SERVER_URL}worker.html`);
  } else {
    createProtocol('app');
    worker.loadURL('app://./worker.html');
  }
}

// 主窗口实例
let mainWindow = null;
async function createWindow() {
  const screenSize = screen.getPrimaryDisplay().size;
  const screenWidth = screenSize.width;
  const screenHeight = screenSize.height;
  const height = 350;
  const windowY = screenHeight - height;

  if (mainWindow) {
    mainWindow.show();
    mainWindow.setSize(screenWidth, height);
    mainWindow.setPosition(0, windowY, true);
    mainWindow.focus();
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
    visualEffectState: 'active',
    vibrancy: 'light',
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
  win.on('blur', win.hide);
  win.setAlwaysOnTop(true, 'pop-up-menu');
  win.setPosition(0, windowY, true);
  win.setVisibleOnAllWorkspaces(true);

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    await win.loadURL(process.env.WEBPACK_DEV_SERVER_URL);
  } else {
    createProtocol('app');
    win.loadURL('app://./index.html');
  }
}

// 开机自启
app.setLoginItemSettings({
  openAtLogin: !isDevelopment,
  openAsHidden: true,
});
// 初始化&监听剪贴板
app.whenReady().then(async () => {
  if (isDevelopment && !process.env.IS_TEST) {
    try {
      await installExtension(VUEJS_DEVTOOLS);
    } catch (e) {
      console.error('Vue Devtools failed to install:', e.toString());
    }
  }

  await createWorker();
  await createWindow();
  // 创建顶栏菜单 隐藏dock栏图标
  tray.createTray();
  app.dock.hide();

  // 全局快捷键 弹出窗口
  globalShortcut.register('Shift+CommandOrControl+V', () => {
    if (mainWindow.isVisible()) {
      mainWindow.hide();
    } else {
      createWindow();
    }
  });
});

// Exit cleanly on request from parent process in development mode.
if (isDevelopment) {
  process.on('SIGTERM', app.quit);
}
