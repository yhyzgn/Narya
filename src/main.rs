mod components;
mod theme;

use components::{badge, glass_card, search_input, switch};
use gpui::{prelude::*, *};
use std::borrow::Cow;
use std::time::Duration;
use theme::Theme;

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        // Diagnostic print to see what path is being requested
        // println!("Loading asset: {}", path);
        match std::fs::read(path) {
            Ok(file) => Ok(Some(Cow::Owned(file))),
            Err(_) => {
                // println!("Failed to load asset {}: {}", path, e);
                Ok(None)
            }
        }
    }

    fn list(&self, _path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ActiveView {
    Dashboard,
    Nodes,
    Connections,
    Rules,
    Subscriptions,
    Config,
    Logs,
    Tools,
    Settings,
    About,
}

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
        let entity_id = cx.entity_id();
        cx.spawn(move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut cx_clone = cx.clone();
            async move {
                for i in 0..=100 {
                    cx_clone
                        .background_executor()
                        .timer(Duration::from_millis(20))
                        .await;
                    let _ = this.update(&mut cx_clone, |this, cx| {
                        this.progress = i as f32 / 100.0;
                        cx.notify();
                    });
                }

                // Wait a bit before switching
                cx_clone
                    .background_executor()
                    .timer(Duration::from_millis(300))
                    .await;

                cx_clone.update(|cx| {
                    open_main_window(cx);
                    let _ = cx.with_window(entity_id, |window, _| {
                        window.remove_window();
                    });
                });
            }
        })
        .detach();
    }
}

impl Render for Splash {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let loading_text = if self.progress < 0.2 {
            "Initializing Kernel..."
        } else if self.progress < 0.4 {
            "Loading Core Modules..."
        } else if self.progress < 0.6 {
            "Configuring Network..."
        } else if self.progress < 0.8 {
            "Synchronizing Subscriptions..."
        } else {
            "Starting Application..."
        };

        div()
            .size_full()
            .relative()
            .child(
                // Background
                img("ui/splash-background-2k.png").size_full().absolute(),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child(
                        // Brand Logo
                        div()
                            .size(px(160.0))
                            .mb_12()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(img("ui/icons/narya-logo-transparent-1080.png").size(px(140.0))),
                    )
                    .child(
                        // Progress Bar Container
                        div()
                            .w(px(320.0))
                            .flex_col()
                            .items_center()
                            .child(
                                div()
                                    .w_full()
                                    .h(px(6.0))
                                    .bg(Rgba {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 0.2,
                                    })
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
                                    .flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(loading_text),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(format!("{}%", (self.progress * 100.0) as i32)),
                                    ),
                            ),
                    ),
            )
            .child(
                // Footer Info
                div()
                    .absolute()
                    .bottom_8()
                    .flex()
                    .w_full()
                    .justify_center()
                    .gap_8()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Version: v0.1.0-alpha"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Config: Default"),
                    ),
            )
    }
}

struct AppShell {
    active_view: ActiveView,
    handle: WeakEntity<Self>,
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let view = self.active_view;
        let handle = self.handle.clone();

        div()
            .size_full()
            .relative()
            .child(
                // Background
                img("ui/splash-background-2k.png").size_full().absolute(),
            )
            .child(
                div()
                    .flex()
                    .size_full()
                    .text_color(theme.text_primary)
                    .child(
                        // Sidebar
                        div()
                            .w(px(270.0))
                            .h_full()
                            .border_r_1()
                            .border_color(theme.border)
                            .bg(Rgba {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 0.4,
                            }) // Semi-transparent sidebar
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
                                    .overflow_hidden()
                                    .child(nav_item("Dashboard", view == ActiveView::Dashboard, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Dashboard;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item("Nodes", view == ActiveView::Nodes, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Nodes;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item(
                                        "Connections",
                                        view == ActiveView::Connections,
                                        {
                                            let handle = handle.clone();
                                            move |_, _, cx| {
                                                let _ = handle.update(cx, |this, cx| {
                                                    this.active_view = ActiveView::Connections;
                                                    cx.notify();
                                                });
                                            }
                                        },
                                    ))
                                    .child(nav_item("Rules", view == ActiveView::Rules, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Rules;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item(
                                        "Subscriptions",
                                        view == ActiveView::Subscriptions,
                                        {
                                            let handle = handle.clone();
                                            move |_, _, cx| {
                                                let _ = handle.update(cx, |this, cx| {
                                                    this.active_view = ActiveView::Subscriptions;
                                                    cx.notify();
                                                });
                                            }
                                        },
                                    ))
                                    .child(nav_item("Config", view == ActiveView::Config, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Config;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item("Logs", view == ActiveView::Logs, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Logs;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item("Tools", view == ActiveView::Tools, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Tools;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item("Settings", view == ActiveView::Settings, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::Settings;
                                                cx.notify();
                                            });
                                        }
                                    }))
                                    .child(nav_item("About", view == ActiveView::About, {
                                        let handle = handle.clone();
                                        move |_, _, cx| {
                                            let _ = handle.update(cx, |this, cx| {
                                                this.active_view = ActiveView::About;
                                                cx.notify();
                                            });
                                        }
                                    })),
                            )
                            .child(
                                // Sidebar Footer
                                div().p_6().child(
                                    glass_card()
                                        .p_3()
                                        .flex()
                                        .items_center()
                                        .child(
                                            div().size(px(10.0)).bg(theme.success).rounded_full(),
                                        )
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
                                        div().text_2xl().font_weight(FontWeight::SEMIBOLD).child(
                                            match view {
                                                ActiveView::Dashboard => "Dashboard",
                                                ActiveView::Nodes => "Nodes",
                                                ActiveView::Connections => "Connections",
                                                ActiveView::Rules => "Rules",
                                                ActiveView::Subscriptions => "Subscriptions",
                                                ActiveView::Config => "Config",
                                                ActiveView::Logs => "Logs",
                                                ActiveView::Tools => "Tools",
                                                ActiveView::Settings => "Settings",
                                                ActiveView::About => "About",
                                            },
                                        ),
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
                                div().flex_1().overflow_hidden().px_8().child(match view {
                                    ActiveView::Dashboard => {
                                        render_dashboard_view().into_any_element()
                                    }
                                    ActiveView::Nodes => render_nodes_view().into_any_element(),
                                    ActiveView::Subscriptions => {
                                        render_subscriptions_view().into_any_element()
                                    }
                                    _ => div()
                                        .child(format!("{:?} View Placeholder", view))
                                        .into_any_element(),
                                }),
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
                                        div().text_xs().text_color(theme.text_secondary).child(
                                            "Narya v1.0.0-alpha | Kernels: sing-box (Active)",
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}

fn nav_item(
    label: &'static str,
    active: bool,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
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
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, on_click)
        .child(label)
}

fn render_dashboard_view() -> impl IntoElement {
    div()
        .flex_col()
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
                            .child(node_item("US-West (Oregon)", "156ms", false))
                            .child(node_item("HK-Premium (Hong Kong)", "45ms", false)),
                    ),
            ),
        )
}

fn render_nodes_view() -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex_col()
        .size_full()
        .child(
            // Toolbar
            div()
                .flex()
                .items_center()
                .justify_between()
                .mb_6()
                .child(search_input())
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .child(badge("All Protocols", theme.primary_light))
                        .child(badge("Fastest", theme.success)),
                ),
        )
        .child(
            // Node List
            div()
                .flex_1()
                .overflow_hidden()
                .flex_col()
                .gap_3()
                .child(node_card("SG-01 (Singapore)", "Shadowsocks", "12ms", true))
                .child(node_card("US-West (Oregon)", "VLESS", "156ms", false))
                .child(node_card("HK-Premium (Hong Kong)", "Trojan", "45ms", false))
                .child(node_card("JP-Tokyo", "Shadowsocks", "68ms", false))
                .child(node_card("DE-Frankfurt", "VLESS", "180ms", false)),
        )
}

fn render_subscriptions_view() -> impl IntoElement {
    div().flex_col().size_full().child(
        div()
            .flex()
            .gap_6()
            .child(subscription_card(
                "Premium Plan",
                0.45,
                "Expires in 15 days",
            ))
            .child(subscription_card("Free Trial", 0.82, "Expires in 2 days")),
    )
}

fn subscription_card(title: &'static str, usage: f32, expiry: &'static str) -> impl IntoElement {
    let theme = Theme::default();
    glass_card().w(px(380.0)).child(
        div()
            .flex_col()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_4()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title),
                    )
                    .child(badge("Active", theme.success)),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_secondary)
                    .mb_1()
                    .child("Traffic Usage")
                    .child(format!("{} / 100 GB", (usage * 100.0) as i32)),
            )
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .bg(theme.border)
                    .rounded_full()
                    .mb_4()
                    .child(
                        div()
                            .h_full()
                            .w(relative(usage))
                            .bg(if usage > 0.8 {
                                theme.danger
                            } else {
                                theme.primary
                            })
                            .rounded_full(),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(div().text_xs().text_color(theme.text_muted).child(expiry))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .border_1()
                                    .border_color(theme.border)
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .text_xs()
                                    .child("Update"),
                            )
                            .child(
                                div()
                                    .bg(theme.surface)
                                    .border_1()
                                    .border_color(theme.border)
                                    .rounded_md()
                                    .px_3()
                                    .py_1()
                                    .text_xs()
                                    .child("Copy Link"),
                            ),
                    ),
            ),
    )
}

fn node_card(
    name: &'static str,
    protocol: &'static str,
    latency: &'static str,
    selected: bool,
) -> impl IntoElement {
    let theme = Theme::default();
    glass_card().p_4().child(
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .size(px(10.0))
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
                            .flex_col()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(name))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child(protocol),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(badge(
                        latency,
                        if selected {
                            theme.success
                        } else {
                            theme.text_muted
                        },
                    ))
                    .child(
                        div()
                            .bg(if selected {
                                theme.primary
                            } else {
                                theme.surface
                            })
                            .border_1()
                            .border_color(theme.border)
                            .rounded_md()
                            .px_3()
                            .py_1()
                            .text_xs()
                            .text_color(if selected {
                                rgb(0xffffff)
                            } else {
                                theme.text_primary
                            })
                            .child("Connect"),
                    ),
            ),
    )
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

fn open_main_window(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |_, cx| {
            cx.new(|cx| AppShell {
                active_view: ActiveView::Dashboard,
                handle: cx.entity().downgrade(),
            })
        },
    )
    .expect("failed to open main window");
}

fn main() {
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
