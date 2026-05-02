use crate::components::{icon, IconName};
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
        AppState::start_traffic_monitor(state.clone(), cx);

        // Default and Min size: 1536 x 980
        let size = size(px(1536.0), px(980.0));
        let bounds = Bounds::centered(None, size, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size),
                ..Default::default()
            },
            move |_, cx| {
                cx.new(|_| AppShell {
                    active_view: ActiveView::Subscriptions,
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

        let active_node = state_ref
            .active_node_id
            .as_ref()
            .and_then(|id| state_ref.nodes.iter().find(|n| n.id == *id));

        let active_node_name = active_node
            .map(|n| n.name.clone())
            .unwrap_or_else(|| "Not Connected".to_string());

        let download_speed = active_node.map(|n| n.download_speed).unwrap_or(0.0);
        let upload_speed = active_node.map(|n| n.upload_speed).unwrap_or(0.0);
        let is_running = state_ref.kernel_running;

        // --- SPEC CONSTANTS ---
        let sidebar_width = px(220.0);
        let header_height = px(64.0);
        let footer_height = px(36.0);

        let color_bg = rgb(0xF5F7FB);
        let color_surface = rgb(0xFFFFFF);
        let color_border = rgb(0xE5E7EB);
        let color_text_primary = rgb(0x111827);
        let color_text_secondary = rgb(0x6B7280);
        let color_brand = rgb(0x3B82F6);
        let color_brand_soft = rgb(0xEEF2FF);

        div()
            .size_full()
            .flex()
            .flex_row() // Sidebar on the left, everything else on the right
            .bg(color_surface)
            .text_color(color_text_primary)
            .child(
                // --- 1. Sidebar (220px fixed) ---
                div()
                    .w(sidebar_width)
                    .flex_shrink_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .border_r_1()
                    .border_color(color_border)
                    .bg(color_surface)
                    .child(
                        div()
                            .flex_col()
                            .child(
                                // Sidebar Brand Area
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
                                                    .text_color(color_text_secondary)
                                                    .child("v1.0.0"),
                                            ),
                                    ),
                            )
                            .child(
                                // Menu List
                                div()
                                    .flex_col()
                                    .p_2()
                                    .gap_1()
                                    .child(nav_item(
                                        "仪表盘",
                                        IconName::Dashboard,
                                        view == ActiveView::Dashboard,
                                        cx,
                                        ActiveView::Dashboard,
                                    ))
                                    .child(nav_item(
                                        "节点",
                                        IconName::Nodes,
                                        view == ActiveView::Nodes,
                                        cx,
                                        ActiveView::Nodes,
                                    ))
                                    .child(nav_item(
                                        "配置",
                                        IconName::Config,
                                        view == ActiveView::Config,
                                        cx,
                                        ActiveView::Config,
                                    ))
                                    .child(nav_item(
                                        "订阅",
                                        IconName::Subscriptions,
                                        view == ActiveView::Subscriptions,
                                        cx,
                                        ActiveView::Subscriptions,
                                    ))
                                    .child(nav_item(
                                        "连接",
                                        IconName::Connections,
                                        view == ActiveView::Connections,
                                        cx,
                                        ActiveView::Connections,
                                    ))
                                    .child(nav_item(
                                        "规则",
                                        IconName::Rules,
                                        view == ActiveView::Rules,
                                        cx,
                                        ActiveView::Rules,
                                    ))
                                    .child(nav_item(
                                        "日志",
                                        IconName::Logs,
                                        view == ActiveView::Logs,
                                        cx,
                                        ActiveView::Logs,
                                    ))
                                    .child(nav_item(
                                        "工具箱",
                                        IconName::Tools,
                                        view == ActiveView::Tools,
                                        cx,
                                        ActiveView::Tools,
                                    ))
                                    .child(nav_item(
                                        "设置",
                                        IconName::Settings,
                                        view == ActiveView::Settings,
                                        cx,
                                        ActiveView::Settings,
                                    )),
                            ),
                    )
                    .child(
                        // Sidebar Bottom Area
                        div()
                            .flex_col()
                            .p_3()
                            .gap_2()
                            .child(
                                // Status Card
                                div()
                                    .p_3()
                                    .rounded_lg()
                                    .bg(rgb(0xF9FAFB))
                                    .border_1()
                                    .border_color(color_border)
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
                                                    .bg(if is_running { rgb(0x10B981) } else { color_text_secondary })
                                                    .rounded_full(),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(if is_running { "已连接" } else { "未连接" }),
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
                                            .text_xs()
                                            .bg(color_brand_soft)
                                            .text_color(color_brand)
                                            .px_2()
                                            .py_0p5()
                                            .rounded_sm()
                                            .child("48ms"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .text_xs()
                                            .text_color(color_text_secondary)
                                            .child(div().child(format!("↓ {:.1} MB/s", download_speed)))
                                            .child(div().child(format!("↑ {:.1} MB/s", upload_speed))),
                                    )
                                    .child(
                                        // Chart Placeholder
                                        div().h(px(40.0)).w_full().bg(color_border).rounded_sm(),
                                    ),
                            )
                            .child(
                                // Bottom Action Icons
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(action_icon(IconName::Github))
                                    .child(action_icon(IconName::Moon))
                                    .child(action_icon(IconName::Bell)),
                            ),
                    ),
            )
            .child(
                // --- 2. Main Right Side (Flex Column) ---
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .bg(color_bg)
                    .overflow_hidden()
                    .child(
                        // Main Header (64px)
                        div()
                            .h(header_height)
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_5()
                            .bg(color_surface)
                            .border_b_1()
                            .border_color(color_border)
                            .child(
                                div()
                                    .flex_col()
                                    .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).child(
                                        match view {
                                            ActiveView::Dashboard => "仪表盘",
                                            ActiveView::Nodes => "节点列表",
                                            ActiveView::Config => "配置编辑",
                                            ActiveView::Subscriptions => "订阅管理",
                                            ActiveView::Connections => "连接详情",
                                            ActiveView::Rules => "规则管理",
                                            ActiveView::Logs => "实时日志",
                                            ActiveView::Tools => "工具箱",
                                            ActiveView::Settings => "系统设置",
                                            _ => "Narya",
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(color_text_secondary)
                                            .child("管理您的网络连接与订阅"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(header_button("添加", true))
                                    .child(header_button("刷新全部", false))
                                    .child(header_button("导入", false))
                                    .child(header_button("导出", false))
                                    .child(action_icon(IconName::Settings))
                                    .child(action_icon(IconName::ExternalLink)),
                            ),
                    )
                    .child(
                        // Scrollable Content
                        div().flex_1().p_5().overflow_hidden().child(match view {
                            ActiveView::Dashboard => render_dashboard_view(&self.state, cx).into_any_element(),
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
                        // --- 3. Footer (36px) - Now on the right side ---
                        div()
                            .h(footer_height)
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_4()
                            .bg(color_surface)
                            .border_t_1()
                            .border_color(color_border)
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
                                            .text_color(color_brand)
                                            .cursor_pointer()
                                            .child("检查更新"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(color_text_secondary)
                                            .child("1.0.0"),
                                    ),
                            ),
                    ),
            )
    }
}

fn nav_item(
    label: &'static str,
    icon_name: IconName,
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
                .gap_3()
                .child(
                    icon(
                        icon_name,
                        18.0,
                        if active { rgb(0x3B82F6).into() } else { rgb(0x6B7280).into() },
                    )
                )
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

fn action_icon(icon_name: IconName) -> impl IntoElement {
    div()
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(0xF3F4F6)))
        .child(icon(icon_name, 18.0, rgb(0x6B7280).into()))
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
