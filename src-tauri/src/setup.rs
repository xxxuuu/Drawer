use std::fs;

use tauri::api::dialog::MessageDialogBuilder;
use tauri::{Manager, Position, LogicalPosition, LogicalSize, Size, App, GlobalShortcutManager, WindowEvent, SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayEvent};
use window_vibrancy::NSVisualEffectMaterial;

use crate::cmd::ClipboardRecordVO;
use crate::storage::{StorageConn, ClipboardRecord};
use crate::{clipboard::{ClipboardManager, ClipboardContent}, storage};
use crate::event::Topic;

pub type AppError = Box<(dyn std::error::Error + 'static)>;
pub type SetupResult = Result<(), AppError>;

/// 设置窗口
fn set_window(app: &mut App) -> SetupResult {
    let win = app.get_window("main").unwrap();
    const HEIGHT: f64 = 350.0;
    // 设置大小和位置
    let monitor = win.current_monitor()
        .expect("failed to get monitor info")
        .expect("failed to get monitor info");
    let screen_size = monitor.size().to_logical::<f64>(monitor.scale_factor());
    win.set_size(Size::Logical(LogicalSize::new(screen_size.width as f64, HEIGHT)))?;
    win.set_position(Position::Logical(LogicalPosition::new(0.0, screen_size.height as f64 - HEIGHT)))?;
    // 设置毛玻璃背景
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(win, NSVisualEffectMaterial::Popover, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    // 将窗口设置成类似 NSPanel 的模式 https://github.com/tauri-apps/tauri/issues/2258
    // FIXME: Dock栏层级比窗口高
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    Ok(())
}

/// 设置菜单托盘
fn set_tray(app: &mut App) -> SetupResult {
    let app_handle = app.handle();
    SystemTray::new()
      .with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("about", "关于"))
          .add_item(CustomMenuItem::new("quit", "退出 Drawer"))
      )
      .on_event(move | event| match event {
        SystemTrayEvent::MenuItemClick {  id  , .. } => {
            match id.as_str() {
                "about" => {
                    app_handle.get_window("main").unwrap().hide().unwrap();
                    MessageDialogBuilder
                        ::new("Drawer", format!("{}\n\n{}\n\nDrawer是一个macOS上的剪贴板应用",
                            app_handle.package_info().version,
                            "https://github.com/xxxuuu/Drawer"
                        ))
                        .show(|_ok| {});
                },
                "quit" => app_handle.exit(0),
                _ => {}
            }
        },
        _ => {}
      }).build(app)?;
    Ok(())
}

/// 注册全局快捷键
fn register_shortcut(app: &mut App) -> SetupResult {
    let mut short_cut = app.global_shortcut_manager();
    let app_handler = app.handle();
    short_cut.register("shift+command+v", move || {
        let window = app_handler.get_window("main").unwrap();
        if window.is_visible().unwrap() {
            // 使用 app.hide() 而不是 window.hide()，才能实现隐藏时焦点恢复到上一个应用
            app_handler.hide().unwrap();
        } else {
            app_handler.show().unwrap();
            window.set_focus().unwrap();
        }
    })?;
    Ok(())
}

/// 注册窗口事件
fn register_window_event(app: &mut App) -> SetupResult {
    let win = app.get_window("main").unwrap();
    let app_handler = app.handle();
    win.on_window_event(move |e: &WindowEvent| {
        match e {
            WindowEvent::Focused(is_focus) => {
                if !is_focus {
                    app_handler.hide().unwrap();
                }
            },
            _ => {}
        }
    });
    Ok(())
}

// 注册剪贴板事件
fn register_clipboard_event(app: &mut App) -> SetupResult {
    let app_handler = app.handle();
    let cm = ClipboardManager::new(move |c: ClipboardContent| {
        // 剪贴板更新：入库，通知前端
        let _record: ClipboardRecord = match c.try_into() {
            Ok(record) => record,
            Err(err) => {
                println!("failed to convert ClipboardContent to ClipboardRecord {:?}", err);
                return;
            }
        };
        let record = app_handler.state::<StorageConn>().insert_record(_record);
        match record {
            Ok(payload) => {
                let vo: ClipboardRecordVO = match payload.try_into() {
                    Ok(_vo) => _vo,
                    Err(err) => {
                        println!("failed to convert record to vo {:?}", err);
                        return;
                    }
                };
                app_handler.emit_all(Topic::CLIPBOARD_UPDATE, vo).unwrap();
            },
            Err(err) => {
                println!("failed to insert record {:?}", err);
                return;
            }
        }
    });
    app.manage(cm);
    Ok(())
}

// 初始化存储服务
fn init_storage(app: &mut App) -> SetupResult {
    let data_dir = app.path_resolver().app_data_dir().expect("failed to fetch data directory");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");
    let conn = storage::StorageConn::new(&data_dir, app.handle()).expect("failed to create database connection");
    app.handle().manage(conn);
    Ok(())
}

pub fn init(app: &mut App) -> SetupResult {
    set_window(app)?;
    set_tray(app)?;
    init_storage(app)?;
    register_shortcut(app)?;
    register_window_event(app)?;
    register_clipboard_event(app)?;
    Ok(())
}