mod components;
mod theme;

use components::glass_card;
use gpui::{prelude::*, *};
use theme::Theme;

struct AppShell;

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .flex()
            .size_full()
            .bg(theme.bg)
            .text_color(theme.text_primary)
            .child(
                // Sidebar
                div()
                    .w(px(270.0))
                    .h_full()
                    .border_r_1()
                    .border_color(theme.border)
                    .p_4()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Narya"),
                    ),
            )
            .child(
                // Main Area
                div()
                    .flex_1()
                    .flex_col()
                    .child(
                        // Header
                        div()
                            .h(px(118.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .px_6()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Dashboard"),
                            ),
                    )
                    .child(
                        // Content
                        div().flex_1().px_6().child(
                            glass_card()
                                .child("System Proxy Settings")
                                .text_color(theme.text_secondary),
                        ),
                    )
                    .child(
                        // Footer
                        div()
                            .h(px(30.0))
                            .w_full()
                            .border_t_1()
                            .border_color(theme.border)
                            .flex()
                            .items_center()
                            .px_6()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("Status: Online"),
                            ),
                    ),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AppShell),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
