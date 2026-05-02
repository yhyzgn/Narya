#![recursion_limit = "512"]
pub mod assets;
pub mod components;
pub mod ipc;
pub mod state;
pub mod theme;
pub mod views;

use crate::assets::Assets;
use crate::views::splash::Splash;
use gpui::{prelude::*, *};

pub fn run() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            // Initialize System Tray (Skeleton)
            #[cfg(not(target_os = "linux"))] // Tray icon can be tricky on Linux in some environments
            let _tray = init_tray();

            let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: None,
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Transparent,
                    kind: WindowKind::PopUp,
                    ..Default::default()
                },
                |_, cx| cx.new(Splash::new),
            )
            .expect("failed to open splash window");
            cx.activate(true);
        });
}

#[cfg(not(target_os = "linux"))]
fn init_tray() -> Option<tray_icon::TrayIcon> {
    use tray_icon::{menu::Menu, TrayIconBuilder};
    let menu = Menu::new();
    TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Narya")
        .build()
        .ok()
}
