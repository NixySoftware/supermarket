import { BrowserWindow, app } from 'electron';
import started from 'electron-squirrel-startup';
import path from 'node:path';

import { getCodeFromRedirectUrl } from './constants/supermarkets';

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (started) {
    app.quit();
}

const createWindow = async () => {
    const mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
        },
    });

    mainWindow.webContents.addListener('will-navigate', (event) => {
        intercept(mainWindow, event);
    });

    mainWindow.webContents.addListener('will-redirect', (event) => {
        intercept(mainWindow, event);
    });

    await loadRenderer(mainWindow);
};

const loadRenderer = async (window: BrowserWindow, query: Record<string, string> = {}) => {
    if (MAIN_WINDOW_VITE_DEV_SERVER_URL) {
        const url = new URL(MAIN_WINDOW_VITE_DEV_SERVER_URL);
        for (const [key, value] of Object.entries(query)) {
            url.searchParams.set(key, value);
        }
        await window.loadURL(url.toString());
    } else {
        await window.loadFile(path.join(__dirname, `../renderer/${MAIN_WINDOW_VITE_NAME}/index.html`), {
            query,
        });
    }
};

const intercept = (
    window: BrowserWindow,
    event:
        | Electron.Event<Electron.WebContentsWillNavigateEventParams>
        | Electron.Event<Electron.WebContentsWillRedirectEventParams>,
) => {
    console.log(event);

    const result = getCodeFromRedirectUrl(event.url);
    if (result) {
        const [supermarket, code] = result;
        console.log(supermarket, code);

        event.preventDefault();
        void loadRenderer(window, {
            'supermarket-id': supermarket.id,
            'supermarket-code': code,
        });
    }
};

app.on('ready', () => {
    void createWindow();
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
});

app.on('activate', () => {
    // On OS X it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) {
        void createWindow();
    }
});
