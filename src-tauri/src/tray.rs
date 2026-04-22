use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};
use tauri_plugin_updater::UpdaterExt;

pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "显示 TinyBox", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "隐藏 TinyBox", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let check_update = MenuItem::with_id(app, "check_update", "检查更新", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&show, &hide, &separator1, &check_update, &separator2, &quit],
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("TinyBox v1.0.0 - 小巧工具箱")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "check_update" => {
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    match try_check_update(&handle).await {
                        Ok(Some(version)) => {
                            let _ = handle.emit("notification", serde_json::json!({ "title": "TinyBox 更新", "body": format!("发现新版本 {}，请前往 GitHub 下载", version) }));
                        }
                        Ok(None) => {
                            let _ = handle.emit("notification", serde_json::json!({ "title": "TinyBox", "body": "已是最新版本" }));
                        }
                        Err(e) => {
                            let _ = handle.emit("notification", serde_json::json!({ "title": "更新检查失败", "body": e.to_string() }));
                        }
                    }
                });
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

async fn try_check_update(
    app: &tauri::AppHandle,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let updater = app.updater_builder().build()?;
    match updater.check().await {
        Ok(Some(update)) => Ok(Some(update.version)),
        Ok(None) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
