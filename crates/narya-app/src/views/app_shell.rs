use crate::state::AppState;
use crate::views::dashboard::render_dashboard_view;
use crate::views::nodes::render_nodes_view;
use crate::views::subscriptions::render_subscriptions_view;
use crate::views::ActiveView;
use gpui::{prelude::*, *};

pub struct AppShell {
    pub(super) active_view: ActiveView,
    pub(super) state: Entity<AppState>,
}

impl AppShell {
    pub fn open(cx: &mut App) {
        let state = cx.new(|_| AppState::mock_data());
        let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            move |_, cx| {
                cx.new(|_| AppShell {
                    active_view: ActiveView::Dashboard,
                    state,
                })
            },
        )
        .expect("failed to open main window");
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = self.active_view;
        let state_ref = self.state.read(cx);

        let active_node_name = state_ref
            .active_node_id
            .as_ref()
            .and_then(|id| state_ref.nodes.iter().find(|n| n.id == *id))
            .map(|n| n.name.clone())
            .unwrap_or_else(|| "Not Connected".to_string());

        // --- SPEC CONSTANTS ---
        let sidebar_width = px(220.0);
        let titlebar_height = px(48.0);
        let header_height = px(64.0);
        let footer_height = px(36.0);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0xF5F7FB)) // Background color from spec
            .text_color(rgb(0x111827)) // Text primary
            .child(
                // 1. TitleBar (48px)
                div()
                    .h(titlebar_height)
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .border_b_1()
                    .border_color(rgb(0xE5E7EB))
                    .bg(rgb(0xFFFFFF))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(img("resources/assets/logo.png").size(px(24.0)))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Narya"),
                            )
                            .child(div().text_xs().text_color(rgb(0x6B7280)).child("v1.0.0")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(window_control_button("−"))
                            .child(window_control_button("□"))
                            .child(window_control_button("×")),
                    ),
            )
            .child(
                // 2. Body (Flex Row)
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .child(
                        // 2.1 Sidebar (220px fixed)
                        div()
                            .w(sidebar_width)
                            .h_full()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .bg(rgb(0xFFFFFF))
                            .border_r_1()
                            .border_color(rgb(0xE5E7EB))
                            .child(
                                // Sidebar Brand Header
                                div()
                                    .h(px(60.0))
                                    .flex()
                                    .items_center()
                                    .px_5()
                                    .gap_2()
                                    .child(img("resources/assets/logo.png").size(px(32.0)))
                                    .child(
                                        div()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(FontWeight::BOLD)
                                                    .child("Narya"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Secure Proxy"),
                                            ),
                                    ),
                            )
                            .child(
                                // Menu List
                                div()
                                    .flex_1()
                                    .flex_col()
                                    .px_2()
                                    .py_2()
                                    .gap_1()
                                    .child(nav_item(
                                        "Dashboard",
                                        view == ActiveView::Dashboard,
                                        cx,
                                        ActiveView::Dashboard,
                                    ))
                                    .child(nav_item(
                                        "Nodes",
                                        view == ActiveView::Nodes,
                                        cx,
                                        ActiveView::Nodes,
                                    ))
                                    .child(nav_item(
                                        "Connections",
                                        view == ActiveView::Connections,
                                        cx,
                                        ActiveView::Connections,
                                    ))
                                    .child(nav_item(
                                        "Rules",
                                        view == ActiveView::Rules,
                                        cx,
                                        ActiveView::Rules,
                                    ))
                                    .child(nav_item(
                                        "Subscriptions",
                                        view == ActiveView::Subscriptions,
                                        cx,
                                        ActiveView::Subscriptions,
                                    ))
                                    .child(nav_item(
                                        "Config",
                                        view == ActiveView::Config,
                                        cx,
                                        ActiveView::Config,
                                    ))
                                    .child(nav_item(
                                        "Logs",
                                        view == ActiveView::Logs,
                                        cx,
                                        ActiveView::Logs,
                                    ))
                                    .child(nav_item(
                                        "Tools",
                                        view == ActiveView::Tools,
                                        cx,
                                        ActiveView::Tools,
                                    ))
                                    .child(nav_item(
                                        "Settings",
                                        view == ActiveView::Settings,
                                        cx,
                                        ActiveView::Settings,
                                    ))
                                    .child(nav_item(
                                        "About",
                                        view == ActiveView::About,
                                        cx,
                                        ActiveView::About,
                                    )),
                            )
                            .child(
                                // Sidebar Footer Area
                                div()
                                    .flex_col()
                                    .child(
                                        // Status Card
                                        div()
                                            .m_3()
                                            .p_3()
                                            .rounded_lg()
                                            .bg(rgb(0xF9FAFB))
                                            .border_1()
                                            .border_color(rgb(0xE5E7EB))
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .size(px(8.0))
                                                            .bg(rgb(0x10B981))
                                                            .rounded_full(),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .child("已连接"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(active_node_name),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child(div().child("↓ 12.4 MB/s"))
                                                    .child(div().child("↑ 4.6 MB/s")),
                                            ),
                                    )
                                    .child(
                                        // Bottom Actions
                                        div()
                                            .px_3()
                                            .py_2()
                                            .flex()
                                            .gap_3()
                                            .child(action_icon("GH"))
                                            .child(action_icon("🌙"))
                                            .child(action_icon("🔔")),
                                    ),
                            ),
                    )
                    .child(
                        // 2.2 Main (Flex Column)
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                // Header (64px)
                                div()
                                    .h(header_height)
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px_5()
                                    .bg(rgb(0xFFFFFF))
                                    .border_b_1()
                                    .border_color(rgb(0xE5E7EB))
                                    .child(
                                        div()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(match view {
                                                        ActiveView::Dashboard => "仪表盘",
                                                        ActiveView::Nodes => "节点列表",
                                                        ActiveView::Connections => "活动连接",
                                                        ActiveView::Rules => "分流规则",
                                                        ActiveView::Subscriptions => "订阅管理",
                                                        ActiveView::Config => "配置编辑",
                                                        ActiveView::Logs => "实时日志",
                                                        ActiveView::Tools => "工具箱",
                                                        ActiveView::Settings => "系统设置",
                                                        ActiveView::About => "关于 Narya",
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("管理您的网络连接与订阅"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_3()
                                            .child(header_button("添加", true))
                                            .child(header_button("刷新全部", false)),
                                    ),
                            )
                            .child(
                                // Content (Flex 1)
                                div().flex_1().p_5().overflow_hidden().child(match view {
                                    ActiveView::Dashboard => {
                                        render_dashboard_view().into_any_element()
                                    }
                                    ActiveView::Nodes => {
                                        render_nodes_view(&self.state, cx).into_any_element()
                                    }
                                    ActiveView::Subscriptions => {
                                        render_subscriptions_view(&self.state, cx)
                                            .into_any_element()
                                    }
                                    _ => div()
                                        .child(format!("{:?} View Placeholder", view))
                                        .into_any_element(),
                                }),
                            ),
                    ),
            )
            .child(
                // 3. Footer (36px)
                div()
                    .h(footer_height)
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .bg(rgb(0xFFFFFF))
                    .border_t_1()
                    .border_color(rgb(0xE5E7EB))
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(footer_info_item("内核", "sing-box"))
                            .child(footer_info_item("配置", "Narya Default"))
                            .child(footer_info_item("订阅", "机场 A · 128 节点")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x3B82F6))
                                    .cursor_pointer()
                                    .child("检查更新"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x6B7280))
                                    .child("Version: 1.0.0"),
                            ),
                    ),
            )
    }
}

fn nav_item(
    label: &'static str,
    active: bool,
    cx: &mut Context<AppShell>,
    target_view: ActiveView,
) -> impl IntoElement {
    let handle = cx.entity().downgrade();
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    div()
        .flex()
        .items_center()
        .h(px(44.0))
        .px_3()
        .rounded_md()
        .cursor_pointer()
        .bg(if active { rgb(0xEEF2FF) } else { transparent })
        .hover(|s| s.bg(rgb(0xF3F4F6)))
        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
            let _ = handle.update(cx, |this, cx| {
                this.active_view = target_view;
                cx.notify();
            });
        })
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().size(px(18.0)).bg(rgb(0x6B7280)).rounded_sm()) // Icon placeholder
                .child(
                    div()
                        .text_sm()
                        .font_weight(if active {
                            FontWeight::MEDIUM
                        } else {
                            FontWeight::NORMAL
                        })
                        .text_color(if active { rgb(0x3B82F6) } else { rgb(0x111827) })
                        .child(label),
                ),
        )
}

fn window_control_button(label: &'static str) -> impl IntoElement {
    div()
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(0xF3F4F6)))
        .child(label)
}

fn action_icon(label: &'static str) -> impl IntoElement {
    div()
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(0xF3F4F6)))
        .child(label)
}

fn header_button(label: &'static str, primary: bool) -> impl IntoElement {
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    div()
        .h(px(32.0))
        .px_3()
        .flex()
        .items_center()
        .rounded_md()
        .cursor_pointer()
        .bg(if primary { rgb(0x3B82F6) } else { transparent })
        .border_1()
        .border_color(rgb(0xE5E7EB))
        .text_color(if primary {
            rgb(0xFFFFFF)
        } else {
            rgb(0x111827)
        })
        .text_xs()
        .hover(move |s| if !primary { s.bg(rgb(0xF3F4F6)) } else { s })
        .child(label)
}

fn footer_info_item(label: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .gap_1()
        .text_xs()
        .child(div().text_color(rgb(0x6B7280)).child(format!("{}:", label)))
        .child(div().text_color(rgb(0x111827)).child(value))
}
