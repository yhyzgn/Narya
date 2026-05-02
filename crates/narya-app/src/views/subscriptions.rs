use crate::components::{icon, IconName};
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
    let _theme = Theme::default();

    // --- Definitive Colors from PNG ---
    let color_bg = rgb(0xF8FAFC);
    let color_card = rgb(0xFFFFFF);
    let color_border = rgb(0xE2E8F0);
    let color_brand = rgb(0x4F46E5);
    let color_success = rgb(0x10B981);
    let color_text_primary = rgb(0x0F172A);
    let color_text_secondary = rgb(0x64748B);
    let color_text_muted = rgb(0x94A3B8);

    div()
        .flex_col()
        .size_full()
        .bg(color_bg)
        .gap_6()
        .child(
            // 1. Top Metrics Row
            div()
                .flex()
                .w_full()
                .gap_4()
                .child(metric_card(
                    "当前订阅",
                    "机场 A",
                    Some("运行中"),
                    "类型: 远程订阅",
                    IconName::Dashboard,
                    color_brand,
                ))
                .child(metric_card(
                    "节点总数",
                    "128",
                    None,
                    "38 可用 / 3 失败",
                    IconName::Nodes,
                    rgb(0x3B82F6),
                ))
                .child(metric_card(
                    "剩余流量",
                    "842 GB",
                    None,
                    "已用 436 GB / 总量 1.28 TB",
                    IconName::Config,
                    rgb(0x6366F1),
                ))
                .child(metric_card(
                    "到期时间",
                    "42 天",
                    None,
                    "2026-06-10 到期",
                    IconName::Subscriptions,
                    rgb(0x8B5CF6),
                ))
                .child(
                    div()
                        .flex_col()
                        .gap_3()
                        .w(px(140.0))
                        .child(
                            div()
                                .h(px(42.0))
                                .bg(color_brand)
                                .text_color(white())
                                .rounded_lg()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .cursor_pointer()
                                .child(icon(IconName::Github, 14.0, white().into()))
                                .child(div().text_sm().font_weight(FontWeight::BOLD).child("添加订阅")),
                        )
                        .child(
                            div()
                                .h(px(42.0))
                                .bg(color_card)
                                .border_1()
                                .border_color(color_border)
                                .rounded_lg()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .cursor_pointer()
                                .child(icon(IconName::Github, 14.0, color_text_primary.into()))
                                .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("手动刷新")),
                        ),
                ),
        )
        .child(
            // 2. Main Middle Section
            div()
                .flex()
                .w_full()
                .gap_6()
                .flex_1()
                .child(
                    // Column 1: List
                    div()
                        .flex_col()
                        .w(relative(0.3))
                        .gap_4()
                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("订阅源列表"))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .h(px(38.0))
                                .bg(color_card)
                                .border_1()
                                .border_color(color_border)
                                .rounded_lg()
                                .px_3()
                                .gap_2()
                                .child(icon(IconName::Github, 16.0, color_text_muted.into()))
                                .child(div().text_sm().text_color(color_text_muted).child("搜索订阅名称或 URL"))
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_3()
                                .children(state.subscriptions.iter().map(|sub| subscription_card(sub, sub.name == "机场 A")))
                        )
                )
                .child(
                    // Column 2: Details Panel
                    div()
                        .w(relative(0.4))
                        .flex_col()
                        .bg(color_card)
                        .border_1()
                        .border_color(color_border)
                        .rounded_2xl()
                        .p_6()
                        .gap_6()
                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("订阅详情"))
                        .child(
                            div()
                                .flex()
                                .gap_8()
                                .border_b_1()
                                .border_color(color_border)
                                .child(tab_item("概览", true))
                                .child(tab_item("节点", false))
                                .child(tab_item("规则", false))
                                .child(tab_item("转换", false))
                                .child(tab_item("高级", false))
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_4()
                                .child(form_row("名称", "机场 A", false))
                                .child(form_row("订阅 URL", "https://*****************/sub", true))
                                .child(form_row("User-Agent", "Narya/1.0.0 (Windows; sing-box)", true))
                                .child(form_row("更新间隔", "30 分钟", true))
                                .child(form_row("目标内核", "sing-box", true))
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_4()
                                .mt_2()
                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("流量配额"))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_8()
                                        .child(
                                            // Realistic Circular Progress Simulation
                                            div()
                                                .size(px(68.0))
                                                .rounded_full()
                                                .border_4()
                                                .border_color(color_success)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(div().text_sm().font_weight(FontWeight::BOLD).child("34%"))
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .flex_col()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .justify_between()
                                                        .items_baseline()
                                                        .child(div().flex_col().child(div().text_xs().text_color(color_text_secondary).child("已用流量")).child(div().text_base().font_weight(FontWeight::BOLD).text_color(color_success).child("↓ 436 GB")))
                                                        .child(div().flex_col().items_end().child(div().text_xs().text_color(color_text_secondary).child("总量")).child(div().text_base().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("1.28 TB")))
                                                )
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .h(px(6.0))
                                                        .bg(rgb(0xF1F5F9))
                                                        .rounded_full()
                                                        .child(div().w(relative(0.34)).h_full().bg(color_success).rounded_full())
                                                )
                                        )
                                )
                        )
                        .child(
                            div()
                                .flex_col()
                                .gap_3()
                                .child(info_row("到期时间", "2026-06-10 (42 天后)", false))
                                .child(info_row("上次更新", "2024-04-29 17:28:42", true))
                        )
                )
                .child(
                    // Column 3: Status Cards
                    div()
                        .w(relative(0.3))
                        .flex_col()
                        .gap_6()
                        .child(
                            div()
                                .flex_col()
                                .bg(color_card)
                                .border_1()
                                .border_color(color_border)
                                .rounded_2xl()
                                .p_6()
                                .gap_5()
                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("更新状态"))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().size(px(14.0)).bg(color_success).rounded_full())
                                        .child(div().text_base().font_weight(FontWeight::BOLD).text_color(color_success).child("更新成功"))
                                )
                                .child(div().flex().justify_between().items_baseline().child(div().text_xs().text_color(color_text_secondary).child("延迟")).child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("128 ms")))
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(36.0))
                                        .bg(rgb(0xF1F5F9))
                                        .rounded_lg()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap_2()
                                        .child(icon(IconName::Github, 14.0, color_text_secondary.into()))
                                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(color_text_secondary).child("查看更新日志"))
                                )
                        )
                        .child(
                            div()
                                .flex_col()
                                .bg(color_card)
                                .border_1()
                                .border_color(color_border)
                                .rounded_2xl()
                                .p_6()
                                .gap_5()
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("自动更新"))
                                        .child(div().w(px(34.0)).h(px(18.0)).bg(color_brand).rounded_full())
                                )
                                .child(div().flex().justify_between().items_baseline().child(div().text_xs().text_color(color_text_secondary).child("更新间隔")).child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("每 30 分钟")))
                        )
                )
        )
        .child(
            // 3. Bottom Sections
            div()
                .flex()
                .w_full()
                .gap_6()
                .child(
                    // Traffic Analytics
                    div()
                        .w(relative(0.55))
                        .flex_col()
                        .bg(color_card)
                        .border_1()
                        .border_color(color_border)
                        .rounded_2xl()
                        .p_6()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .items_end()
                                .child(
                                    div()
                                        .flex()
                                        .items_baseline()
                                        .gap_3()
                                        .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("流量趋势"))
                                        .child(div().text_xs().text_color(color_text_secondary).child("(最近 30 天)"))
                                )
                                .child(div().text_xs().text_color(color_text_primary).bg(rgb(0xF1F5F9)).px_2().py_1().rounded_md().child("30 天 ▾"))
                        )
                        .child(
                            div()
                                .h(px(150.0))
                                .w_full()
                                .bg(rgb(0xFAFBFE))
                                .rounded_xl()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_xs().text_color(color_text_muted).child("Enhanced Analytics Plot"))
                        )
                )
                .child(
                    // Subscription Priority
                    div()
                        .w(relative(0.45))
                        .flex_col()
                        .gap_4()
                        .child(
                            div()
                                .flex_col()
                                .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("订阅优先级"))
                                .child(div().text_xs().text_color(color_text_secondary).child("按优先级从高到低使用，拖拽可调整顺序"))
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(priority_item("1", "远程订阅", "机场 A", true))
                                .child(div().text_xl().text_color(color_border).child("→"))
                                .child(priority_item("2", "本地覆写", "Narya Default", false))
                                .child(div().text_xl().text_color(color_border).child("→"))
                                .child(priority_item("3", "UI 临时", "活动中", false))
                        )
                )
        )
}

fn metric_card(title: &'static str, val: &'static str, badge_text: Option<&'static str>, sub: &'static str, icon_name: IconName, color: Rgba) -> impl IntoElement {
    let mut icon_bg: Hsla = color.into();
    icon_bg.a = 0.1;

    div()
        .flex_1()
        .bg(rgb(0xFFFFFF))
        .border_1()
        .border_color(rgb(0xE2E8F0))
        .rounded_2xl()
        .p_5()
        .flex()
        .items_center()
        .gap_4()
        .child(
            div()
                .size(px(48.0))
                .bg(icon_bg)
                .rounded_2xl()
                .flex()
                .items_center()
                .justify_center()
                .child(icon(icon_name, 26.0, color.into()))
        )
        .child(
            div()
                .flex_col()
                .gap_1()
                .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(0x64748B)).child(title))
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .gap_2()
                        .child(div().text_2xl().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(val))
                        .child(if let Some(t) = badge_text {
                            let mut bg: Hsla = rgb(0xDCFCE7).into();
                            bg.a = 1.0;
                            div().bg(bg).px_2().py_0p5().rounded_full().child(div().text_color(rgb(0x10B981)).text_size(px(10.0)).font_weight(FontWeight::BOLD).child(t)).into_any_element()
                        } else {
                            div().into_any_element()
                        })
                )
                .child(div().text_xs().text_color(rgb(0x94A3B8)).child(sub))
        )
}

fn subscription_card(sub: &NaryaSubscription, active: bool) -> impl IntoElement {
    let border_color = if active { rgb(0x4F46E5) } else { rgb(0xE2E8F0) };
    let border_width = if active { px(2.0) } else { px(1.0) };

    let mut icon_bg: Hsla = rgb(0x4F46E5).into();
    icon_bg.a = 0.1;

    div()
        .bg(rgb(0xFFFFFF))
        .border(border_width)
        .border_color(border_color)
        .rounded_xl()
        .p_5()
        .flex()
        .gap_4()
        .child(
            div()
                .size(px(44.0))
                .bg(icon_bg)
                .rounded_xl()
                .flex()
                .items_center()
                .justify_center()
                .child(icon(IconName::Dashboard, 22.0, rgb(0x4F46E5).into()))
        )
        .child(
            div()
                .flex_1()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .items_baseline()
                        .child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap_2()
                                .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(sub.name.clone()))
                                .child(if active {
                                    let mut bg: Hsla = rgb(0xEEF2FF).into();
                                    bg.a = 1.0;
                                    div().bg(bg).px_2().py_0p5().rounded_md().child(div().text_color(rgb(0x4F46E5)).text_size(px(11.0)).font_weight(FontWeight::BOLD).child("当前使用")).into_any_element()
                                } else {
                                    div().into_any_element()
                                })
                        )
                )
                .child(div().text_xs().text_color(rgb(0x94A3B8)).child("https://***********/sub"))
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .mt_2()
                        .child(div().text_color(rgb(0x64748B)).text_size(px(11.0)).child(format!("{} 节点   更新: {}", sub.node_count, sub.update_time)))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(div().text_color(rgb(0x64748B)).text_size(px(10.0)).font_weight(FontWeight::BOLD).child("34%"))
                                .child(div().w(px(40.0)).h(px(4.0)).bg(rgb(0xE5E7EB)).rounded_full().child(div().w(relative(0.34)).h_full().bg(rgb(0x10B981)).rounded_full()))
                        )
                )
        )
}

fn tab_item(label: &'static str, active: bool) -> impl IntoElement {
    div()
        .pb_3()
        .border_b(if active { px(2.5) } else { px(0.0) })
        .border_color(rgb(0x4F46E5))
        .child(
            div()
                .text_sm()
                .font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                .text_color(if active { rgb(0x4F46E5) } else { rgb(0x64748B) })
                .child(label)
        )
}

fn form_row(label: &'static str, val: &'static str, has_icon: bool) -> impl IntoElement {
    div()
        .flex()
        .justify_between()
        .items_baseline()
        .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(rgb(0x64748B)).child(label))
        .child(
            div()
                .flex()
                .items_baseline()
                .gap_3()
                .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(val))
                .child(if has_icon { icon(IconName::ExternalLink, 14.0, rgb(0x94A3B8).into()).into_any_element() } else { div().into_any_element() })
        )
}

fn info_row(label: &'static str, val: &'static str, success: bool) -> impl IntoElement {
    div()
        .flex()
        .items_baseline()
        .gap_4()
        .child(div().w(px(100.0)).text_xs().text_color(rgb(0x64748B)).child(label))
        .child(
            div()
                .flex()
                .items_baseline()
                .gap_3()
                .child(div().text_sm().font_weight(FontWeight::MEDIUM).text_color(rgb(0x0F172A)).child(val))
                .child(if success {
                    div().bg(rgb(0xDCFCE7)).px_2().py_0p5().rounded_full().child(div().text_color(rgb(0x10B981)).text_size(px(10.0)).font_weight(FontWeight::BOLD).child("成功")).into_any_element()
                } else {
                    div().into_any_element()
                })
        )
}

fn priority_item(index: &'static str, title: &'static str, sub: &'static str, active: bool) -> impl IntoElement {
    let bg = if active { rgb(0xEEF2FF) } else { rgb(0xFFFFFF) };
    let border = if active { rgb(0x4F46E5) } else { rgb(0xE2E8F0) };

    div()
        .flex_1()
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded_2xl()
        .p_4()
        .flex()
        .items_center()
        .gap_3()
        .child(
            div()
                .size(px(24.0))
                .bg(border)
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_color(white()).text_size(px(10.0)).font_weight(FontWeight::BOLD).child(index))
        )
        .child(
            div()
                .flex_col()
                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(title))
                .child(div().text_xs().text_color(rgb(0x64748B)).child(sub))
        )
}
