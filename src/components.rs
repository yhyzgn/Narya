use gpui::{prelude::*, *};
use crate::theme::Theme;

pub fn glass_card() -> Div {
    let theme = Theme::default();
    
    div()
        .bg(theme.surface)
        .border_1()
        .border_color(theme.border)
        .rounded_xl() // 16px
        .p_6() // 24px padding
        .shadow_sm()
}
