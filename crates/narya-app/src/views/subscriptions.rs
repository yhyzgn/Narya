use crate::components::{badge, glass_card, icon, IconName};
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
    let theme = Theme::default();
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    div()
        .flex_col()
        .size_full()
        .gap_6()
        .child(
            // 1. Metrics Row
            div()
                .flex()
                .w_full()
                .gap_4()
                .justify_between()
                .child(metric_card(
                    "当前订阅",
                    "机场 A",
                    "运行中",
                    "类型: 远程订阅",
                    IconName::Dashboard,
                    rgb(0x4F46E5),
                ))
                .child(metric_card_stats(
                    "节点总数",
                    "128",
                    "38 可用 / 3 失效",
                    IconName::Nodes,
                    rgb(0x3B82F6),
                ))
                .child(metric_card_stats(
                    "剩余流量",
                    "842 GB",
                    "已用 436 GB / 总量 1.28 TB",
                    IconName::Config,
                    rgb(0x6366F1),
                ))
                .child(metric_card_stats(
                    "到期时间",
                    "42 天",
                    "2026-06-10 到期",
                    IconName::Subscriptions,
                    rgb(0x8B5CF6),
                ))
                .child(
                    div()
                        .flex_col()
                        .gap_2()
                        .w(px(140.0))
                        .child(action_button("添加订阅", true, theme.primary))
                        .child(action_button("手动刷新", false, theme.border)),
                ),
        )
        .child(
            // 2. Main Three Columns
            div()
                .flex()
                .w_full()
                .gap_6()
                .child(
                    // Column 1: List
                    div()
                        .flex_col()
                        .w(relative(0.3))
                        .gap_4()
                        .child(div().text_sm().font_weight(FontWeight::BOLD).child("订阅源列表"))
                        .child(
                            div()
                                .flex_col()
                                .gap_3()
                                .children(state.subscriptions.iter().map(subscription_list_item)),
                        ),
                )
                .child(
                    // Column 2: Details
                    glass_card()
                        .w(relative(0.4))
                        .p_5()
                        .flex_col()
                        .gap_5()
                        .child(div().text_sm().font_weight(FontWeight::BOLD).child("订阅详情"))
                        .child(
                            div()
                                .flex()
                                .gap_4()
                                .border_b_1()
                                .border_color(theme.border)
                                .pb_2()
                                .child(tab_item("概览", true))
                                .child(tab_item("节点", false))
                                .child(tab_item("规则", false))
                                .child(tab_item("转换", false)),
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_3()
                                .child(form_row("名称", "机场 A"))
                                .child(form_row("订阅 URL", "https://*****************/sub"))
                                .child(form_row("User-Agent", "Narya/1.0.0 (Windows; sing-box)"))
                                .child(form_row("更新间隔", "30 分钟")),
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_3()
                                .child(div().text_xs().font_weight(FontWeight::MEDIUM).child("流量配额"))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_4()
                                        .child(div().size(px(48.0)).bg(theme.success).rounded_full()) // Placeholder for circular progress
                                        .child(
                                            div()
                                                .flex_1()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .justify_between()
                                                        .child(div().text_xs().text_color(theme.text_secondary).child("已用 436 GB"))
                                                        .child(div().text_xs().text_color(theme.text_secondary).child("总量 1.28 TB")),
                                                )
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .h(px(6.0))
                                                        .bg(theme.border)
                                                        .rounded_full()
                                                        .child(div().w(relative(0.34)).h_full().bg(theme.success).rounded_full()),
                                                ),
                                        ),
                                ),
                        ),
                )
                .child(
                    // Column 3: Status & Settings
                    div()
                        .flex_col()
                        .w(relative(0.3))
                        .gap_4()
                        .child(
                            glass_card()
                                .p_4()
                                .flex_col()
                                .gap_3()
                                .child(div().text_xs().font_weight(FontWeight::MEDIUM).child("更新状态"))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().size(px(8.0)).bg(theme.success).rounded_full())
                                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(theme.success).child("更新成功")),
                                )
                                .child(status_row("延迟", "128 ms"))
                                .child(status_row("下载时间", "1.82 s")),
                        )
                        .child(
                            glass_card()
                                .p_4()
                                .flex_col()
                                .gap_3()
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child(div().text_xs().font_weight(FontWeight::MEDIUM).child("自动更新"))
                                        .child(div().size(px(24.0)).bg(theme.primary).rounded_full()), // Placeholder for toggle
                                )
                                .child(status_row("更新间隔", "每 30 分钟"))
                                .child(status_row("启动时更新", "已开启")),
                        ),
                ),
        )
}

fn metric_card(
    label: &'static str,
    value: &'static str,
    status: &'static str,
    subtext: &'static str,
    icon_name: IconName,
    icon_color: Rgba,
) -> impl IntoElement {
    let theme = Theme::default();
    let mut hsla_color: Hsla = icon_color.into();
    hsla_color.a = 0.1;

    glass_card().flex_1().p_4().child(
        div()
            .flex()
            .items_center()
            .gap_4()
            .child(
                div()
                    .size(px(40.0))
                    .bg(hsla_color)
                    .rounded_lg()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon(icon_name, 24.0, icon_color.into())),
            )
            .child(
                div()
                    .flex_col()
                    .child(div().text_xs().text_color(theme.text_secondary).child(label))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_base().font_weight(FontWeight::BOLD).child(value))
                            .child(badge(status, theme.success.into())),
                    )
                    .child(div().text_xs().text_color(theme.text_muted).child(subtext)),
            ),
    )
}

fn metric_card_stats(
    label: &'static str,
    value: &'static str,
    subtext: &'static str,
    icon_name: IconName,
    icon_color: Rgba,
) -> impl IntoElement {
    let theme = Theme::default();
    let mut hsla_color: Hsla = icon_color.into();
    hsla_color.a = 0.1;

    glass_card().flex_1().p_4().child(
        div()
            .flex()
            .items_center()
            .gap_4()
            .child(
                div()
                    .size(px(40.0))
                    .bg(hsla_color)
                    .rounded_lg()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon(icon_name, 24.0, icon_color.into())),
            )
            .child(
                div()
                    .flex_col()
                    .child(div().text_xs().text_color(theme.text_secondary).child(label))
                    .child(div().text_xl().font_weight(FontWeight::BOLD).child(value))
                    .child(div().text_xs().text_color(theme.text_muted).child(subtext)),
            ),
    )
}

fn action_button(label: &'static str, primary: bool, color: Rgba) -> impl IntoElement {
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    div()
        .h(px(36.0))
        .w_full()
        .bg(if primary { color } else { transparent })
        .border_1()
        .border_color(if primary { color } else { rgb(0xE5E7EB) })
        .text_color(if primary { rgb(0xffffff) } else { rgb(0x111827) })
        .text_xs()
        .rounded_md()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .child(label)
}

fn subscription_list_item(sub: &NaryaSubscription) -> impl IntoElement {
    let theme = Theme::default();
    let is_active = sub.name == "机场 A"; // Hardcoded for fidelity to spec
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    glass_card()
        .p_4()
        .border_color(if is_active { theme.primary } else { theme.border })
        .border_2()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(icon(IconName::Dashboard, 24.0, theme.primary.into()))
                .child(
                    div()
                        .flex_1()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(sub.name.clone()))
                                        .child(if is_active { badge("当前使用", theme.primary.into()) } else { badge("", transparent.into()) }),
                                )
                                .child(icon(IconName::ExternalLink, 16.0, theme.text_muted.into())),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(sub.url.clone()))
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .child(div().text_xs().text_color(theme.text_secondary).child(format!("{} 节点  更新: {}", sub.node_count, sub.update_time))),
                        ),
                ),
        )
}

fn tab_item(label: &'static str, active: bool) -> impl IntoElement {
    let theme = Theme::default();
    let transparent = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    div()
        .pb_1()
        .border_b_2()
        .border_color(if active { theme.primary } else { transparent })
        .text_color(if active { theme.text_primary } else { theme.text_secondary })
        .text_xs()
        .cursor_pointer()
        .child(label)
}

fn form_row(label: &'static str, value: &str) -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex()
        .justify_between()
        .items_center()
        .child(div().text_xs().text_color(theme.text_secondary).child(label))
        .child(
            div()
                .flex()
                .gap_2()
                .items_center()
                .child(div().text_xs().font_weight(FontWeight::MEDIUM).child(value.to_string()))
                .child(icon(IconName::ExternalLink, 14.0, theme.text_muted.into())),
        )
}

fn status_row(label: &'static str, value: &str) -> impl IntoElement {
    let theme = Theme::default();
    div()
        .flex()
        .justify_between()
        .text_xs()
        .child(div().text_color(theme.text_secondary).child(label))
        .child(div().font_weight(FontWeight::MEDIUM).child(value.to_string()))
}
