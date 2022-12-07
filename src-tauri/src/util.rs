use swift_rs::*;
use tauri::Window;
use std::ffi::c_void;

pub type NSObject = *mut c_void;
swift_fn!(get_file_icon_base64(path: &str) -> String);
swift_fn!(get_file_thumbnail_base64(path: &str) -> String);
swift_fn!(set_window_top_level(window: NSObject));
swift_fn!(paste() -> String);

pub enum Data {
    Raw(Vec<u8>),
    Base64(String),
}

pub struct QuickLook;

impl QuickLook {
    /// TODO: 调用QuickLook显示文件预览
    #[cfg(target_os = "macos")]
    pub fn preview() -> Result<Data, String> {
        // qlmanage -p file
        Err("unimplement".to_string())
    }

    /// 生成缩略图
    #[cfg(target_os = "macos")]
    pub fn thumbnail(path: &str) -> Result<Data, String> {
        let mut thumbnail = get_file_thumbnail_base64(path.into());
        if thumbnail.is_empty() {
            thumbnail = get_file_icon_base64(path.into());
        }
        Ok(Data::Base64(thumbnail.to_string()))
    }
}

pub struct ClipboardUtil;

impl ClipboardUtil {
    /// 向系统发送粘贴事件
    #[cfg(target_os = "macos")]
    pub fn paste_event() -> String {
        paste().to_string()
    }
}

pub struct ImageUtil;

impl ImageUtil {
    pub fn img_size(data: &[u8]) -> Result<imagesize::ImageSize, imagesize::ImageError> {
        imagesize::blob_size(data)
    }
}

pub struct WindowUtil;

impl WindowUtil {
    pub fn set_window_top_level(window: &Window) {
        set_window_top_level(window.ns_window().unwrap())
    }
}