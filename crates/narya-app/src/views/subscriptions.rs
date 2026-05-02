use crate::components::{badge, glass_card};
use crate::theme::Theme;
use gpui::{prelude::*, *};

pub fn render_subscriptions_view() -> impl IntoElement {
    div()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .gap_6()
                .child(subscription_card("Premium Plan", 0.45, "Expires in 15 days"))
                .child(subscription_card("Free Trial", 0.82, "Expires in 2 days")),
        )
}

pub fn subscription_card(title: &'static str, usage: f32, expiry: &'static str) -> impl IntoElement {
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
