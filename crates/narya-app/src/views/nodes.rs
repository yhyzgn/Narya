use crate::components::{badge, glass_card, search_input};
use crate::state::AppState;
use crate::theme::Theme;
use crate::views::app_shell::AppShell;
use gpui::{prelude::*, *};
use narya_core::Node;

pub fn render_nodes_view(model: &Entity<AppState>, cx: &mut Context<AppShell>) -> impl IntoElement {
    let theme = Theme::default();
    let state = model.read(cx);

    let color_brand = rgb(0x3B82F6);

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
                        .gap_3()
                        .child(
                            div()
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, {
                                    let model = model.clone();
                                    move |_, _, cx| {
                                        AppState::test_all_latency(model.clone(), cx);
                                    }
                                })
                                .child(badge("一键测速", color_brand)),
                        )
                        .child(badge("All Protocols", theme.primary_light))
                        .child(badge("Fastest", theme.success)),
                ),
        )
        .child(
            // Node List (Middle)
            div()
                .flex_1()
                .overflow_hidden()
                .flex_col()
                .gap_3()
                .mb_6()
                .children(state.nodes.iter().map(|n| {
                    let is_selected = state.active_node_id.as_deref() == Some(&n.id);
                    node_card(n, is_selected, {
                        let model = model.clone();
                        let node_id = n.id.clone();
                        move |_, _, cx| {
                            model.update(cx, |state, cx| {
                                state.active_node_id = Some(node_id.clone());
                                cx.notify();
                            });
                        }
                    })
                })),
        )
        .child(
            // Bottom Area (Chart + Details)
            div()
                .flex()
                .flex_row()
                .gap_6()
                .h(px(240.0))
                .child(
                    // Latency Trend Card
                    glass_card()
                        .flex_1()
                        .child(
                            div()
                                .flex_col()
                                .child(div().text_sm().font_weight(FontWeight::BOLD).child("延迟趋势"))
                                .child(div().flex_1().flex().items_center().justify_center().text_xs().text_color(theme.text_muted).child("Chart Placeholder"))
                        )
                )
                .child(
                    // Node Details Card (Strictly follow design)
                    render_node_details_card(state.active_node_id.as_deref(), &state.nodes)
                )
        )
}

fn render_node_details_card(active_id: Option<&str>, nodes: &[Node]) -> impl IntoElement {
    let theme = Theme::default();
    let node = active_id.and_then(|id| nodes.iter().find(|n| n.id == id));
    
    glass_card()
        .w(px(320.0))
        .child(
            div()
                .flex_col()
                .gap_2()
                .child(div().text_sm().font_weight(FontWeight::BOLD).child(format!("节点详情 ({})", node.map(|n| n.name.as_str()).unwrap_or("未选择"))))
                .child(detail_row("地址", node.map(|n| n.details.address.as_str()).unwrap_or("-")))
                .child(detail_row("协议", node.map(|n| n.protocol.as_str()).unwrap_or("-")))
                .child(detail_row("加密", node.map(|n| n.details.encryption.as_str()).unwrap_or("-")))
                .child(detail_row("UDP", if node.map(|n| n.details.udp).unwrap_or(false) { "已启用" } else { "否" }))
                .child(detail_row("TLS", if node.map(|n| n.details.tls).unwrap_or(false) { "是" } else { "否" }))
                .child(detail_row("传输", node.map(|n| n.details.transport.as_str()).unwrap_or("-")))
                .child(detail_row("上次测试", node.map(|n| n.details.last_test.as_str()).unwrap_or("-")))
        )
}

fn detail_row(label: &'static str, value: &str) -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex()
        .justify_between()
        .text_xs()
        .child(div().text_color(theme.text_secondary).child(label))
        .child(div().text_color(theme.text_primary).font_weight(FontWeight::MEDIUM).child(value.to_string()))
}

pub fn node_card(
    node: &Node,
    selected: bool,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let theme = Theme::default();
    let name = node.name.clone();
    let protocol = node.protocol.clone();
    let latency_str = node
        .latency
        .map(|l| format!("{}ms", l))
        .unwrap_or_else(|| "--".to_string());

    glass_card()
        .p_4()
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, on_click)
        .border_color(if selected { theme.primary } else { theme.border })
        .child(
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
                            latency_str,
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
