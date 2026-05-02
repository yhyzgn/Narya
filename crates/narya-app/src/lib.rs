pub mod assets;
pub mod components;
pub mod theme;
pub mod views;

use crate::assets::Assets;
use crate::views::splash::Splash;
use gpui::{prelude::*, *};

pub fn run() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
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
