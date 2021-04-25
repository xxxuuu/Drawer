import {
  app, protocol, BrowserWindow, globalShortcut, ipcMain, screen,
} from 'electron';
import { createProtocol } from 'vue-cli-plugin-electron-builder/lib';
import installExtension, { VUEJS_DEVTOOLS } from 'electron-devtools-installer';
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
    show: false,
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
async function createWindow() {
  const screenSize = screen.getPrimaryDisplay().size;
  const screenWidth = screenSize.width;
  const screenHeight = screenSize.height;
  const height = 350;
  const windowY = screenHeight - height;

  if (mainWindow) {
    mainWindow.show();
    mainWindow.setPosition(0, windowY, true);
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
  openAtLogin: true,
  openAsHidden: true,
});
// 初始化&监听剪贴板
app.whenReady().then(() => {
  ipcMain.on(event.LOG, (e, args) => {
    console.log(args);
  });

  createWorker();
  // 全局快捷键 弹出窗口
  globalShortcut.register('Shift+CommandOrControl+V', () => {
    createWindow();
  });
});

// Quit when all windows are closed.
app.on('window-all-closed', () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  // if (BrowserWindow.getAllWindows().length === 0) createWindow();
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
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
