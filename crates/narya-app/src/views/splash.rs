use crate::theme::Theme;
use crate::views::app_shell::AppShell;
use gpui::{prelude::*, *};
use std::time::Duration;

pub struct Splash {
    pub(crate) progress: f32,
}

impl Splash {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut splash = Self { progress: 0.0 };
        splash.simulate_loading(cx);
        splash
    }

    fn simulate_loading(&mut self, cx: &mut Context<Self>) {
        let entity_id = cx.entity_id();
        cx.spawn(move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut cx_clone = cx.clone();
            async move {
                for i in 0..=100 {
                    cx_clone
                        .background_executor()
                        .timer(Duration::from_millis(20))
                        .await;
                    let _ = this.update(&mut cx_clone, |this, cx| {
                        this.progress = i as f32 / 100.0;
                        cx.notify();
                    });
                }

                // Wait a bit before switching
                cx_clone
                    .background_executor()
                    .timer(Duration::from_millis(300))
                    .await;

                cx_clone.update(|cx| {
                    AppShell::open(cx);
                    let _ = cx.with_window(entity_id, |window, _| {
                        window.remove_window();
                    });
                });
            }
        })
        .detach();
    }
}

impl Render for Splash {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let loading_text = if self.progress < 0.2 {
            "Initializing Kernel..."
        } else if self.progress < 0.4 {
            "Loading Core Modules..."
        } else if self.progress < 0.6 {
            "Configuring Network..."
        } else if self.progress < 0.8 {
            "Synchronizing Subscriptions..."
        } else {
            "Starting Application..."
        };

        div()
            .size_full()
            .relative()
            .child(
                // Background
                img("resources/assets/bg_splash.png")
                    .size_full()
                    .absolute(),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child(
                        // Brand Logo
                        div()
                            .size(px(160.0))
                            .mb_12()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(img("resources/assets/logo.png").size(px(140.0))),
                    )
                    .child(
                        // Progress Bar Container
                        div()
                            .w(px(320.0))
                            .flex_col()
                            .items_center()
                            .child(
                                div()
                                    .w_full()
                                    .h(px(6.0))
                                    .bg(Rgba {
                                        r: 1.0,
                                        g: 1.0,
                                        b: 1.0,
                                        a: 0.2,
                                    })
                                    .rounded_full()
                                    .child(
                                        div()
                                            .h_full()
                                            .w(relative(self.progress))
                                            .bg(theme.primary)
                                            .rounded_full(),
                                    ),
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(loading_text),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(format!("{}%", (self.progress * 100.0) as i32)),
                                    ),
                            ),
                    )
            )
            .child(
                // Footer Info
                div()
                    .absolute()
                    .bottom_8()
                    .flex()
                    .w_full()
                    .justify_center()
                    .gap_8()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Version: v0.1.0-alpha"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Config: Default"),
                    ),
            )
    }
}
