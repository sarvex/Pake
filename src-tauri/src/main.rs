use tauri::{generate_context, Menu, MenuItem, Submenu};
use tauri_utils::config::{Config, WindowConfig, WindowUrl};
#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "windows")]
use wry::application::platform::windows::WindowBuilderExtWindows;

fn main() {
    let first_menu = Menu::new()
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::EnterFullScreen)
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::CloseWindow)
        .add_native_item(MenuItem::Quit);

    let menu_bar_menu = Menu::new().add_submenu(Submenu::new("App", first_menu));

    tauri::Builder::default()
        .menu(menu_bar_menu)
        .setup(|app| {
            let WindowConfig {
                url,
                width,
                height,
                resizable,
                transparent,
                fullscreen,
                ..
            } = get_windows_config().unwrap_or_default();

            let window_builder = tauri::WindowBuilder::new(
                app,
                "main",
                WindowUrl::External(url.to_string().parse().unwrap()),
            )
            .inner_size(width, height)
            .resizable(resizable)
            .fullscreen(fullscreen)
            .initialization_script(include_str!("pake.js"));
            #[cfg(target_os = "windows")]
            {
                let icon_path = format!("png/{}_32.ico", package_name);
                let icon = load_icon(std::path::Path::new(&icon_path));

                window_builder = window_builder
                    .transparent(transparent)
                    .decorations(false)
                    .icon(Some(icon));
            }
            let window = window_builder.build().unwrap();
            #[cfg(debug_assertions)]
            window.open_devtools();
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn get_windows_config() -> Option<WindowConfig> {
    let config_file = include_str!("../pake.conf.json");
    let config: Config = serde_json::from_str(config_file).expect("failed to parse windows config");

    config.tauri.windows.first().cloned()
}

#[cfg(target_os = "windows")]
fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
