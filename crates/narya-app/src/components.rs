use gpui::{prelude::*, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    Dashboard,
    Nodes,
    Config,
    Subscriptions,
    Connections,
    Rules,
    Logs,
    Tools,
    Settings,
    About,
    Github,
    Moon,
    Bell,
    ExternalLink,
    Terminal,
}

pub fn icon(name: IconName, size: f32, color: Hsla) -> impl IntoElement {
    // High-fidelity placeholder: stylized rounded box
    // In real GPUI app with assets: svg().path(name.path())
    div()
        .size(px(size))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .size(px(size * 0.7))
                .bg(color)
                .rounded_sm()
        )
}

pub fn glass_card() -> Div {
    div()
        .bg(rgb(0xFFFFFF))
        .border_1()
        .border_color(rgb(0xE5E7EB))
        .rounded_xl()
        .shadow_sm()
}

pub fn badge(text: impl Into<String>, color: Hsla) -> impl IntoElement {
    div()
        .bg(color)
        .text_color(rgb(0xffffff))
        .text_xs()
        .px_2()
        .py_0p5()
        .rounded_md()
        .child(text.into())
}

pub fn search_input() -> impl IntoElement {
    div()
        .w(px(320.0))
        .h(px(36.0))
        .bg(rgb(0xffffff))
        .border_1()
        .border_color(rgb(0xe5e7eb))
        .rounded_md()
        .flex()
        .items_center()
        .px_3()
        .gap_2()
        .child("🔍")
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x9ca3af))
                .child("搜索节点..."),
        )
}

pub fn toast(message: impl Into<String>, _kind: ToastKind) -> impl IntoElement {
    div()
        .absolute()
        .bottom_10()
        .right_10()
        .child(
            glass_card()
                .p_4()
                .shadow_lg()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child("🔔")
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .child(message.into()),
                        ),
                ),
        )
}

pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}
