use crate::components::{badge, glass_card, search_input};
use crate::theme::Theme;
use gpui::{prelude::*, *};

pub fn render_nodes_view() -> impl IntoElement {
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

pub fn node_card(
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
