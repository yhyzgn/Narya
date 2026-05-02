use crate::components::glass_card;
use crate::state::AppState;
use crate::theme::Theme;
use crate::views::app_shell::AppShell;
use gpui::{prelude::*, *};

pub fn render_dashboard_view(model: &Entity<AppState>, cx: &mut Context<AppShell>) -> impl IntoElement {
    let theme = Theme::default();
    let state = model.read(cx);
    let is_running = state.kernel_running;

    div()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .gap_6()
                .child(
                    // Quick Connect Card - Fixed Min-Width
                    glass_card()
                        .w(px(420.0))
                        .flex_shrink_0()
                        .child(
                            div()
                                .flex_col()
                                .items_center()
                                .child(
                                    div()
                                        .size(px(80.0))
                                        .bg(if is_running { theme.success } else { theme.border })
                                        .rounded_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            div()
                                                .text_3xl()
                                                .child(if is_running { "⚡" } else { "🔌" }),
                                        ),
                                )
                                .child(
                                    div()
                                        .mt_4()
                                        .text_xl()
                                        .font_weight(FontWeight::BOLD)
                                        .child(if is_running { "已连接" } else { "未连接" }),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .text_sm()
                                        .text_color(theme.text_secondary)
                                        .child("系统代理已启动"),
                                )
                                .child(
                                    div()
                                        .mt_6()
                                        .w_full()
                                        .child(
                                            div()
                                                .bg(if is_running { theme.danger } else { theme.primary })
                                                .text_color(rgb(0xffffff))
                                                .py_3()
                                                .rounded_lg()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor_pointer()
                                                .on_mouse_down(MouseButton::Left, {
                                                    let model = model.clone();
                                                    move |_, _, cx| {
                                                        AppState::toggle_proxy(model.clone(), cx);
                                                    }
                                                })
                                                .child(if is_running { "断开连接" } else { "一键开启" }),
                                        ),
                                ),
                        ),
                )
                .child(
                    // Stats Grid - Proportional growth
                    div()
                        .flex_grow()
                        .flex_col()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .gap_4()
                                .child(stat_card("今日流量", "1.24 GB", theme.primary.into()))
                                .child(stat_card("本月流量", "42.8 GB", theme.success.into())),
                        )
                        .child(
                            div()
                                .flex()
                                .gap_4()
                                .child(stat_card("活动连接", "12", theme.warning.into()))
                                .child(stat_card("当前延迟", "48 ms", theme.primary_light.into())),
                        ),
                ),
        )
}

fn stat_card(label: &'static str, value: &'static str, color: Hsla) -> impl IntoElement {
    let theme = Theme::default();
    glass_card()
        .flex_grow()
        .min_w(px(200.0)) // Ensure a reasonable display even when window is at min-size
        .child(
            div()
                .flex_col()
                .child(div().text_xs().text_color(theme.text_secondary).child(label))
                .child(
                    div()
                        .mt_1()
                        .text_2xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(color)
                        .child(value),
                ),
        )
}
