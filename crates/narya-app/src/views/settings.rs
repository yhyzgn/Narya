use crate::components::{icon, IconName};
use crate::state::AppState;
use crate::views::app_shell::AppShell;
use gpui::{prelude::*, *};

pub fn render_settings_view(_model: &Entity<AppState>, _cx: &mut Context<AppShell>) -> impl IntoElement {
    let color_text_primary: Hsla = rgb(0x0F172A).into();
    let color_text_secondary: Hsla = rgb(0x64748B).into();
    let color_border: Hsla = rgb(0xE2E8F0).into();
    let color_brand: Hsla = rgb(0x4F46E5).into();

    div()
        .flex()
        .size_full()
        .bg(rgb(0xF8FAFC))
        .gap_8()
        .child(
            // Sub-sidebar for settings categories
            div()
                .flex()
                .flex_col()
                .w(px(240.0))
                .flex_shrink_0()
                .gap_2()
                .child(div().flex().mb_4().text_sm().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("设置中心"))
                .child(settings_nav_item("基础设置", IconName::Settings, true))
                .child(settings_nav_item("网络代理", IconName::Dashboard, false))
                .child(settings_nav_item("内核核心", IconName::Terminal, false))
                .child(settings_nav_item("DNS 解析", IconName::Nodes, false))
                .child(settings_nav_item("外观主题", IconName::Moon, false))
                .child(settings_nav_item("关于 Narya", IconName::About, false))
        )
        .child(
            // Settings Detail Area
            div()
                .flex()
                .flex_col()
                .flex_1()
                .bg(white())
                .border_1()
                .border_color(color_border)
                .rounded_2xl()
                .p_8()
                .gap_8()
                .overflow_hidden()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().flex().text_xl().font_weight(FontWeight::BOLD).text_color(color_text_primary).child("基础设置"))
                        .child(div().flex().text_sm().text_color(color_text_secondary).child("管理应用的启动行为、系统托盘以及全局快捷键。"))
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_6()
                        .child(setting_toggle("开机自动启动", "当系统启动时，自动运行 Narya 代理服务。", true))
                        .child(setting_toggle("关闭时最小化", "点击窗口关闭按钮时，将应用隐藏到系统托盘。", true))
                        .child(setting_toggle("自动更新内核", "每天凌晨自动检测并下载最新的内核二进制文件。", false))
                        .child(setting_input("Socks5 端口", "1080"))
                        .child(setting_input("HTTP 端口", "1081"))
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .px_6()
                                .py_2()
                                .bg(rgb(0xF1F5F9))
                                .text_color(color_text_secondary)
                                .rounded_lg()
                                .child("还原默认")
                        )
                        .child(
                            div()
                                .flex()
                                .px_6()
                                .py_2()
                                .bg(color_brand)
                                .text_color(white())
                                .rounded_lg()
                                .font_weight(FontWeight::BOLD)
                                .child("保存修改")
                        )
                )
        )
}

fn settings_nav_item(label: &'static str, icon_name: IconName, active: bool) -> impl IntoElement {
    let bg_color: Hsla = if active { rgb(0xEEF2FF).into() } else { white().into() };
    let text_color: Hsla = if active { rgb(0x4F46E5).into() } else { rgb(0x0F172A).into() };
    div()
        .flex()
        .items_center()
        .gap_3()
        .px_4()
        .py_3()
        .rounded_xl()
        .bg(bg_color)
        .cursor_pointer()
        .child(icon(icon_name, 18.0, if active { rgb(0x4F46E5).into() } else { rgb(0x64748B).into() }))
        .child(div().flex().text_sm().font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM }).text_color(text_color).child(label))
}

fn setting_toggle(title: &'static str, desc: &'static str, active: bool) -> impl IntoElement {
    let bg_color: Hsla = if active { rgb(0x4F46E5).into() } else { rgb(0xE2E8F0).into() };
    div()
        .flex()
        .justify_between()
        .items_center()
        .child(
            div()
                .flex()
                .flex_col()
                .child(div().flex().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(title))
                .child(div().flex().text_xs().text_color(rgb(0x64748B)).child(desc))
        )
        .child(
            div()
                .flex()
                .items_center()
                .w(px(40.0))
                .h(px(22.0))
                .bg(bg_color)
                .rounded_full()
                .px_1()
                .child({
                    let mut dot = div().flex().size(px(16.0)).bg(white()).rounded_full();
                    if active { dot = dot.ml_auto(); }
                    dot
                })
        )
}

fn setting_input(label: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .justify_between()
        .items_center()
        .child(div().flex().text_sm().font_weight(FontWeight::BOLD).text_color(rgb(0x0F172A)).child(label))
        .child(
            div()
                .flex()
                .w(px(200.0))
                .px_3()
                .py_2()
                .bg(rgb(0xF8FAFC))
                .border_1()
                .border_color(rgb(0xE2E8F0))
                .rounded_lg()
                .text_sm()
                .text_color(rgb(0x0F172A))
                .child(value)
        )
}
