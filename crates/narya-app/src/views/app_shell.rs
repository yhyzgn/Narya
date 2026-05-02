use crate::components::glass_card;
use crate::state::AppState;
use crate::theme::Theme;
use crate::views::dashboard::render_dashboard_view;
use crate::views::nodes::render_nodes_view;
use crate::views::subscriptions::render_subscriptions_view;
use crate::views::ActiveView;
use gpui::{prelude::*, *};

pub struct AppShell {
    pub(super) active_view: ActiveView,
    pub(super) state: Entity<AppState>,
    pub(super) handle: WeakEntity<Self>,
}

impl AppShell {
    pub fn open(cx: &mut App) {
        let state = cx.new(|_| AppState::mock_data());
        let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_, cx| {
                cx.new(|cx| AppShell {
                    active_view: ActiveView::Dashboard,
                    state,
                    handle: cx.entity().downgrade(),
                })
            },
        )
        .expect("failed to open main window");
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let view = self.active_view;
        let state_ref = self.state.read(cx);
        let handle = self.handle.clone();

        let active_node_name = state_ref
            .active_node_id
            .as_ref()
            .and_then(|id| state_ref.nodes.iter().find(|n| n.id == *id))
            .map(|n| n.name.clone())
            .unwrap_or_else(|| "Not Connected".to_string());

        div()
            .size_full()
            .bg(theme.bg) // Use theme background color instead of image
            .flex()
            .text_color(theme.text_primary)
            .child(
                // Sidebar
                div()
                    .w(px(270.0))
                    .h_full()
                    .border_r_1()
                    .border_color(theme.border)
                    .bg(theme.surface) // Solid surface for sidebar
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
                            .child(nav_item("Connections", view == ActiveView::Connections, {
                                let handle = handle.clone();
                                move |_, _, cx| {
                                    let _ = handle.update(cx, |this, cx| {
                                        this.active_view = ActiveView::Connections;
                                        cx.notify();
                                    });
                                }
                            }))
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
                                .child(div().size(px(10.0)).bg(theme.success).rounded_full())
                                .child(
                                    div()
                                        .ml_3()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(format!("Connected to {}", active_node_name)),
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
                            .child(div().text_2xl().font_weight(FontWeight::SEMIBOLD).child(
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
                            ))
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
                            ActiveView::Dashboard => render_dashboard_view().into_any_element(),
                            ActiveView::Nodes => {
                                render_nodes_view(&self.state, cx).into_any_element()
                            }
                            ActiveView::Subscriptions => {
                                render_subscriptions_view(&self.state, cx).into_any_element()
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
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("Narya v1.0.0-alpha | Kernels: sing-box (Active)"),
                            ),
                    ),
            )
    }
}

pub fn nav_item(
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
