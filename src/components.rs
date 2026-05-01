use crate::theme::Theme;
use gpui::{prelude::*, *};

pub fn glass_card() -> Div {
    let theme = Theme::default();

    div()
        .bg(theme.surface)
        .border_1()
        .border_color(theme.border)
        .rounded_xl() // 12px based on spec
        .p_6() // 24px padding
        .shadow_sm()
}

#[allow(dead_code)]
pub fn button(label: impl IntoElement) -> Div {
    let theme = Theme::default();

    div()
        .bg(theme.primary)
        .text_color(rgb(0xffffff))
        .rounded_lg()
        .px_4()
        .py_2()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .child(label)
}

#[allow(dead_code)]
pub fn switch(active: bool) -> Div {
    let theme = Theme::default();

    div()
        .w(px(52.0))
        .h(px(29.0))
        .rounded_full()
        .bg(if active {
            theme.success
        } else {
            theme.text_muted
        })
        .p(px(3.0))
        .flex()
        .child(
            div()
                .size(px(23.0))
                .rounded_full()
                .bg(rgb(0xffffff))
                .shadow_sm()
                .ml(if active { px(23.0) } else { px(0.0) }),
        )
}
