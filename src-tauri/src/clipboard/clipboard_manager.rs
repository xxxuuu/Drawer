use std::{thread, sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}}};

use clipboard_master::{Master, ClipboardHandler};
use serde::{Serialize, Deserialize};
use urlencoding::decode_binary;
use crate::{clipboard::clipboard::ClipboardFormat};

use super::clipboard::{Clipboard, backend};

type ClipboardCallback = Box<dyn Fn(ClipboardContent) + Send + 'static>;

// 剪贴板类型
cfg_if::cfg_if!{
    // some discussion: https://github.com/electron/electron/issues/9035
    // macOS: https://developer.apple.com/documentation/appkit/nspasteboardtype
    if #[cfg(target_os = "macos")] {
        const UTF8: &'static str = "public.utf8-plain-text";
        const UTF16: &'static str = "public.utf16-plain-text";
        const HTML: &'static str = "public.html";
        const WEB_ARCHIVE: &'static str = "com.apple.webarchive";
        const RTF: &'static str = "public.rtf";
        const JPG: &'static str = "public.jpeg";
        const PNG: &'static str = "public.png";
        const TIFF: &'static str = "public.tiff";
        const FILE: &'static str = "public.file-url";
        const MULTI_FILE: &'static str = "NSFilenamesPboardType";
        const PDF: &'static str = "com.adobe.pdf";
        const COLOR: &'static str = "com.apple.cocoa.pasteboard.color";
    } else if #[cfg(target_os = "windows")] {
        // Windows: https://www.codeproject.com/Reference/1091137/Windows-Clipboard-Formats
    } else {
        // Unix(X11)
    }
}
/// 防止重复记录的专属格式
const PREVENT_RECOPY: &'static str = "com.xxxuuu.drawer.prevent_recopy";
const PREVENT_FORMAT: ClipboardFormat = ClipboardFormat {
    identifier: PREVENT_RECOPY,
    data: vec![],
};

/// 不同格式的剪贴板数据
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClipboardFormatContent {
    Text(String),
    RTF(Vec<u8>),
    Image(Vec<u8>),
    Files(Vec<String>),   // ([file_url...])
}

/// 剪贴板内容
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardContent {
    pub main_format: ClipboardFormatContent,
    pub data: Vec<ClipboardFormatContent>
}

/// 剪贴板管理
pub struct ClipboardManager (Arc<Mutex<InnerManger>>);

struct InnerManger {
    callback: ClipboardCallback,
    clipboard: Clipboard,
}

struct Handler {
    tx: Sender<()>
}

impl ClipboardManager {
    pub fn new<F>(callback: F) -> ClipboardManager where F: Fn(ClipboardContent) + Send + 'static {
        let (tx, rx) = mpsc::channel();
        let cm = ClipboardManager ( Arc::new(Mutex::new(InnerManger {
            callback: Box::new(callback),
            clipboard: Clipboard(backend::Clipboard)
        })));
        cm.listener_launch(rx, tx);
        cm
    }

    /// 启动剪贴板监听器
    fn listener_launch(&self, rx: Receiver<()>, tx: Sender<()>) {
        thread::spawn(move || {
            Master::new(Handler{tx}).run().unwrap();
        });

        // 收到更新事件后，获取剪贴板内容，调用callback
        let _inner = self.0.clone();
        thread::spawn(move || for _ in rx {
            let inner = _inner.lock().unwrap();
            if let Some(content) = inner.get_content() {
                (*inner.callback)(content);
            }
        });
    }

    /// 粘贴一条内容到剪贴板上
    pub fn paste(&self, content: ClipboardContent) -> Result<(), String> {
        self.0.clone().lock().unwrap().paste(content.into())
    }
}

impl InnerManger {
    /// 获取剪贴板内容
    fn get_content(&self) -> Option<ClipboardContent> {
        let types = self.clipboard.available_type_names();
        println!("avalibe format type: {:?}", &types);

        // 本来想尝试存储所有类型的数据以真正存储剪贴板所有内容，但 get_format 参数需要一个 'static str
        // 看起来是因为通过 FFI 申请的内存没有释放所以才必须传入'static来限制，之后有空也许可以优化一下它的实现
        // for i in types {
        //     if let Some(data) = c.get_format(i.as_str()) {
        //         println!("{} => {} byte", i, data.len());
        //     }
        // }

        // 自己粘贴的 就忽略
        if let Some(_) = self.clipboard.get_format(PREVENT_RECOPY) {
            return None
        }

        let mut clipboard_data: Vec<ClipboardFormatContent> = vec![];
        let mut main_format = None;

        if let Some(text) = self.clipboard.get_string() {
            main_format = Some(ClipboardFormatContent::Text(text.clone()));
            clipboard_data.push(ClipboardFormatContent::Text(text));
        }
        if let Some(png) = self.clipboard.get_format(PNG) {
            main_format = Some(ClipboardFormatContent::Image(png.clone()));
            clipboard_data.push(ClipboardFormatContent::Image(png));
        }
        if let Some(rtf) = self.clipboard.get_format(RTF) {
            let _rtf = ClipboardFormatContent::RTF(rtf);
            clipboard_data.push(_rtf.clone());
            main_format = Some(_rtf);
        }
        if let Some(file_url) = self.clipboard.get_format(FILE) {
            let _binary = decode_binary(file_url.as_slice());
            let _file = ClipboardFormatContent::Files(
                vec![String::from_utf8_lossy(&_binary)
                    .to_string()
                    .replace("file://", "")]
            );
            main_format = Some(_file.clone());
            clipboard_data.push(_file);
        }

        if let None = main_format {
            return None
        }
        if clipboard_data.len() == 0 {
            return None
        }

        Some(ClipboardContent {
            main_format: main_format.unwrap(),
            data: clipboard_data
        })
    }

    /// 粘贴剪贴板内容
    fn paste(&mut self, content: ClipboardContent) -> Result<(), String> {
        let mut formats = vec![PREVENT_FORMAT];
        for data in content.data {
            match data {
                ClipboardFormatContent::Text(text) => {
                    formats.push(ClipboardFormat {
                        identifier: ClipboardFormat::TEXT,
                        data: text.as_bytes().to_vec()
                    });
                },
                ClipboardFormatContent::RTF(rtf) => {
                    formats.push(ClipboardFormat {
                        identifier: RTF,
                        data: rtf,
                    });
                },
                ClipboardFormatContent::Image(img) => {
                    formats.push(ClipboardFormat {
                        identifier: PNG,
                        data: img,
                    });
                },
                ClipboardFormatContent::Files(files) => {
                    // TODO: 多文件&多文件夹
                    formats.push(ClipboardFormat {
                        identifier: MULTI_FILE,
                        data: format!(r#"
                            <?xml version="1.0" encoding="UTF-8"?>
                            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
                            <plist version="1.0">
                            <array>
                                <string>{}</string>
                            </array>
                            </plist>
                        "#, files[0]).into(),
                    });
                }
            }
        }
        self.clipboard.put_formats(&formats);
        Ok(())
    }
}

/// 剪贴板监听处理
impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> clipboard_master::CallbackResult {
        if let Err(err) = self.tx.send(()) {
            println!("failed to send clipboard change event {}", err);
        }
        clipboard_master::CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, _: std::io::Error) -> clipboard_master::CallbackResult {
        clipboard_master::CallbackResult::Next
    }
}