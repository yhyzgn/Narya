use crate::components::{badge, glass_card};
use crate::state::AppState;
use crate::theme::Theme;
use crate::views::app_shell::AppShell;
use gpui::{prelude::*, *};
use narya_core::Subscription as NaryaSubscription;

pub fn render_subscriptions_view(
    model: &Entity<AppState>,
    cx: &mut Context<AppShell>,
) -> impl IntoElement {
    let state = model.read(cx);
    div().flex_col().size_full().child(
        div()
            .flex()
            .gap_6()
            .children(state.subscriptions.iter().map(subscription_card)),
    )
}

pub fn subscription_card(sub: &NaryaSubscription) -> impl IntoElement {
    let theme = Theme::default();
    let usage_ratio = (sub.traffic_used / sub.traffic_total) as f32;
    let title = sub.name.clone();
    let expiry = sub.expiration.clone();
    let status = sub.status.clone();
    let usage_str = format!("{:.1} / {:.1} GB", sub.traffic_used, sub.traffic_total);

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
                    .child(badge(status, theme.success.into())),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_secondary)
                    .mb_1()
                    .child("Traffic Usage")
                    .child(usage_str),
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
                            .w(relative(usage_ratio))
                            .bg(if usage_ratio > 0.8 {
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
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("Expires: {}", expiry)),
                    )
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
