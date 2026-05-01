mod components;
mod theme;

use components::{glass_card, switch};
use gpui::{prelude::*, *};
use std::time::Duration;
use theme::Theme;

struct Splash {
    progress: f32,
}

impl Splash {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut splash = Self { progress: 0.0 };
        splash.simulate_loading(cx);
        splash
    }

    fn simulate_loading(&mut self, cx: &mut Context<Self>) {
        cx.spawn(|this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut cx_clone = cx.clone();
            async move {
                for i in 0..=100 {
                    cx_clone
                        .background_executor()
                        .timer(Duration::from_millis(30))
                        .await;
                    let _ = this.update(&mut cx_clone, |this, cx| {
                        this.progress = i as f32 / 100.0;
                        cx.notify();
                    });
                }

                // Wait a bit before switching
                cx_clone
                    .background_executor()
                    .timer(Duration::from_millis(500))
                    .await;

                cx_clone.update(|cx| {
                    let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
                    cx.open_window(
                        WindowOptions {
                            window_bounds: Some(WindowBounds::Windowed(bounds)),
                            ..Default::default()
                        },
                        |_, cx| cx.new(|_| AppShell),
                    )
                    .expect("failed to open main window");
                });
            }
        })
        .detach();
    }
}

impl Render for Splash {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(theme.bg)
            .child(
                // Logo placeholder
                div()
                    .size(px(120.0))
                    .bg(theme.primary_light)
                    .rounded_xl()
                    .mb_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("N"),
                    ),
            )
            .child(
                div()
                    .w(px(300.0))
                    .h(px(6.0))
                    .bg(theme.border)
                    .rounded_full()
                    .child(
                        div()
                            .h_full()
                            .w(relative(self.progress))
                            .bg(theme.primary)
                            .rounded_full(),
                    ),
            )
            .child(
                div()
                    .mt_4()
                    .text_sm()
                    .text_color(theme.text_secondary)
                    .child(format!(
                        "Loading Resources... {}%",
                        (self.progress * 100.0) as i32
                    )),
            )
    }
}

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
                    .flex_col()
                    .child(
                        // Sidebar Logo
                        div()
                            .h(px(118.0))
                            .flex()
                            .items_center()
                            .px_6()
                            .child(
                                div()
                                    .size(px(32.0))
                                    .bg(theme.primary)
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("N"),
                                    ),
                            )
                            .child(
                                div()
                                    .ml_3()
                                    .text_xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Narya"),
                            ),
                    )
                    .child(
                        // Nav items
                        div()
                            .flex_1()
                            .px_3()
                            .child(nav_item("Dashboard", true))
                            .child(nav_item("Nodes", false))
                            .child(nav_item("Subscriptions", false))
                            .child(nav_item("Rules", false))
                            .child(nav_item("Settings", false)),
                    )
                    .child(
                        // Sidebar Footer
                        div().p_6().child(
                            glass_card()
                                .p_3()
                                .flex()
                                .items_center()
                                .child(div().size(px(10.0)).bg(theme.success).rounded_full())
                                .child(
                                    div()
                                        .ml_3()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child("Connected to SG-01"),
                                ),
                        ),
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
                            .justify_between()
                            .px_8()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Dashboard"),
                            )
                            .child(
                                div().flex().items_center().child(
                                    div()
                                        .size(px(40.0))
                                        .bg(theme.surface)
                                        .border_1()
                                        .border_color(theme.border)
                                        .rounded_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child("🔔"), // placeholder icon
                                ),
                            ),
                    )
                    .child(
                        // Content
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .px_8()
                            .child(
                                div()
                                    .flex()
                                    .gap_6()
                                    .child(proxy_card(
                                        "System Proxy",
                                        "Redirect system traffic to Narya",
                                        true,
                                    ))
                                    .child(proxy_card(
                                        "TUN Mode",
                                        "Virtual network interface for all apps",
                                        false,
                                    )),
                            )
                            .child(
                                div().mt_6().child(
                                    glass_card()
                                        .child(
                                            div()
                                                .text_lg()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .mb_4()
                                                .child("Quick Connect"),
                                        )
                                        .child(
                                            div()
                                                .flex_col()
                                                .gap_2()
                                                .child(node_item("SG-01 (Singapore)", "12ms", true))
                                                .child(node_item(
                                                    "US-West (Oregon)",
                                                    "156ms",
                                                    false,
                                                ))
                                                .child(node_item(
                                                    "HK-Premium (Hong Kong)",
                                                    "45ms",
                                                    false,
                                                )),
                                        ),
                                ),
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
                            .px_8()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("Narya v1.0.0-alpha | Kernels: sing-box (Active)"),
                            ),
                    ),
            )
    }
}

fn nav_item(label: &'static str, active: bool) -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex()
        .items_center()
        .px_4()
        .py_3()
        .rounded_lg()
        .bg(if active {
            theme.surface
        } else {
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        })
        .text_color(if active {
            theme.primary
        } else {
            theme.text_secondary
        })
        .font_weight(if active {
            FontWeight::SEMIBOLD
        } else {
            FontWeight::NORMAL
        })
        .child(label)
}

fn proxy_card(title: &'static str, description: &'static str, active: bool) -> impl IntoElement {
    let theme = Theme::default();
    glass_card().flex_1().child(
        div()
            .flex()
            .justify_between()
            .items_start()
            .child(
                div()
                    .flex_col()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .mt_1()
                            .child(description),
                    ),
            )
            .child(switch(active)),
    )
}

fn node_item(name: &'static str, latency: &'static str, selected: bool) -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex()
        .items_center()
        .justify_between()
        .p_3()
        .rounded_md()
        .bg(if selected {
            theme.bg
        } else {
            Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        })
        .child(
            div()
                .flex()
                .items_center()
                .child(
                    div()
                        .size(px(8.0))
                        .bg(if selected {
                            theme.success
                        } else {
                            theme.text_muted
                        })
                        .rounded_full(),
                )
                .child(
                    div()
                        .ml_3()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .child(name),
                ),
        )
        .child(div().text_xs().text_color(theme.text_muted).child(latency))
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(Splash::new),
        )
        .expect("failed to open splash window");
        cx.activate(true);
    });
}
