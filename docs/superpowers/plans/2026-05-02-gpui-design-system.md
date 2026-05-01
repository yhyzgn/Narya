# GPUI Design System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the foundational GPUI Design Tokens, GlassCard component, and AppShell layout for Narya.

**Architecture:** Create a `theme` module to encapsulate Design Tokens (colors, spacing, shadows). Implement the `GlassCard` as a reusable styling function or component. Update the `main.rs` to render the canonical AppShell layout using these tokens.

**Tech Stack:** Rust, GPUI (from zed-industries/zed git).

---

### Task 1: Establish Theme Module and Tokens

**Files:**
- Modify: `src/main.rs` (to include theme module)
- Create: `src/theme.rs`

- [ ] **Step 1: Create `theme.rs` with Design Tokens**

Create `src/theme.rs` and define the core colors and dimensions based on the design spec.

```rust
use gpui::{rgb, Rgba};

pub struct Theme {
    pub bg: Rgba,
    pub surface: Rgba,
    pub border: Rgba,
    pub primary: Rgba,
    pub success: Rgba,
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: rgb(0xf9fafc),
            surface: Rgba { r: 1.0, g: 1.0, b: 1.0, a: 0.8 }, // rgba(255, 255, 255, 0.8)
            border: Rgba { r: 0.88, g: 0.90, b: 0.92, a: 0.5 }, // rgba(225, 230, 236, 0.5) approx
            primary: rgb(0x4f46e5),
            success: rgb(0x10b981),
            text_primary: rgb(0x1e293b),
            text_secondary: rgb(0x64748b),
        }
    }
}
```

- [ ] **Step 2: Integrate `theme.rs` into `main.rs`**

Add `mod theme;` to the top of `src/main.rs`.

```rust
mod theme;
use gpui::{prelude::*, *};
use theme::Theme;
// ... rest of code
```

- [ ] **Step 3: Run cargo check**

Run: `cargo check`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/theme.rs
git commit -m "feat(ui): implement base design tokens in theme module"
```

---

### Task 2: Implement GlassCard Component

**Files:**
- Create: `src/components.rs`
- Modify: `src/main.rs` (to include components module)

- [ ] **Step 1: Create `components.rs` and `GlassCard`**

Create a reusable `GlassCard` wrapper that applies the blur, border, and shadow. GPUI allows styling `div()` directly, so we can create a helper function that returns a pre-styled `div`.

```rust
use gpui::{prelude::*, *};
use crate::theme::Theme;

pub fn glass_card() -> Div {
    let theme = Theme::default();
    
    div()
        .bg(theme.surface)
        // GPUI styling for blur and border
        // Note: GPUI currently might not expose all CSS-like backdrop-filters directly
        // We will approximate the glass look with background opacity and border
        .border_1()
        .border_color(theme.border)
        .rounded_xl() // 16px
        .p_6() // 24px padding
        .shadow_sm() // Approximation of ambient shadow
}
```

- [ ] **Step 2: Integrate `components.rs` into `main.rs`**

Add `mod components;` to `src/main.rs`.

```rust
mod components;
mod theme;
// ...
```

- [ ] **Step 3: Run cargo check**

Run: `cargo check`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/components.rs
git commit -m "feat(ui): implement reusable glass_card component styling"
```

---

### Task 3: Build AppShell Layout

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace HelloWorld with AppShell structure**

Replace the `HelloWorld` struct and its `Render` implementation with a new `AppShell` view that implements the 1536x1024 canonical layout.

```rust
use crate::theme::Theme;
use crate::components::glass_card;

struct AppShell;

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .flex()
            .size_full()
            .bg(theme.bg)
            .text_color(theme.text_primary)
            .child(
                // Sidebar
                div()
                    .w(px(270.0))
                    .h_full()
                    .border_r_1()
                    .border_color(theme.border)
                    .p_4()
                    .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).child("Narya")),
            )
            .child(
                // Main Area
                div()
                    .flex_1()
                    .flex_col()
                    .child(
                        // Header
                        div()
                            .h(px(118.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .px_6()
                            .child(div().text_xl().font_weight(FontWeight::SEMIBOLD).child("Dashboard")),
                    )
                    .child(
                        // Content
                        div()
                            .flex_1()
                            .px_6()
                            .child(
                                glass_card()
                                    .child("System Proxy Settings")
                                    .text_color(theme.text_secondary)
                            )
                    )
                    .child(
                        // Footer
                        div()
                            .h(px(30.0))
                            .w_full()
                            .border_t_1()
                            .border_color(theme.border)
                            .flex()
                            .items_center()
                            .px_6()
                            .child(div().text_xs().text_color(theme.text_secondary).child("Status: Online")),
                    )
            )
    }
}
```

- [ ] **Step 2: Update App initialization**

Update `main()` to load `AppShell` and set the canonical window bounds.

```rust
fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1536.0), px(1024.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AppShell),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
```

- [ ] **Step 3: Run cargo check and formatting**

Run: `cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat(ui): implement AppShell canonical layout and replace hello world"
```
