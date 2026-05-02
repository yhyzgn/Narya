use crate::components::glass_card;
use crate::components::switch;
use crate::theme::Theme;
use gpui::{prelude::*, *};

pub fn render_dashboard_view() -> impl IntoElement {
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

pub fn proxy_card(
    title: &'static str,
    description: &'static str,
    active: bool,
) -> impl IntoElement {
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

pub fn node_item(name: &'static str, latency: &'static str, selected: bool) -> impl IntoElement {
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
