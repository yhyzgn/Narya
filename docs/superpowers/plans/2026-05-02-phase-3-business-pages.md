# Phase 3 Implementation Plan: Business Pages & Enum Routing

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement view routing and restore `Nodes` and `Subscriptions` pages in GPUI with high fidelity.

**Architecture:** Introduce an `ActiveView` enum in `AppShell`. Refactor the content area to render views based on the active state. Use mock data for list rendering.

**Tech Stack:** Rust, GPUI.

---

### Task 1: Add `ActiveView` Enum and Refactor Routing

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Define the `ActiveView` enum**

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum ActiveView {
    Dashboard,
    Nodes,
    Subscriptions,
    Rules,
    Settings,
}
```

- [ ] **Step 2: Add `active_view` to `AppShell` and update navigation**

Update `AppShell` struct:
```rust
struct AppShell {
    active_view: ActiveView,
}
```
Update `nav_item` signature to accept a click callback or use a direct update.

- [ ] **Step 3: Implement view switching in the sidebar**

Update `AppShell::render` to handle clicks on `nav_item`.

- [ ] **Step 4: Verify compilation and basic switching**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat(ui): add ActiveView enum and routing state to AppShell"
```

---

### Task 2: Implement Reusable Components (`Badge`, `SearchInput`)

**Files:**
- Modify: `src/components.rs`

- [ ] **Step 1: Implement `Badge` component**

```rust
pub fn badge(label: impl IntoElement, color: Rgba) -> Div {
    div()
        .bg(color)
        .text_color(rgb(0xffffff))
        .rounded_md()
        .px_2()
        .py_0p5()
        .text_xs()
        .child(label)
}
```

- [ ] **Step 2: Implement `SearchInput` component**

```rust
pub fn search_input() -> Div {
    let theme = Theme::default();
    div()
        .flex()
        .items_center()
        .w(px(641.0)) // Based on spec
        .h(px(40.0))
        .border_1()
        .border_color(theme.border)
        .rounded_lg()
        .bg(theme.surface)
        .px_3()
        .child(div().text_sm().text_color(theme.text_muted).child("Search nodes..."))
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 4: Commit**

```bash
git add src/components.rs
git commit -m "feat(ui): implement reusable Badge and SearchInput components"
```

---

### Task 3: Implement `Nodes` View

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Create `render_nodes_view` function**

Implement the layout with a toolbar (SearchInput, filters) and a scrollable list of nodes.

- [ ] **Step 2: Implement `node_card` for the detailed view**

Include Name, Protocol Badge, Latency Badge, and a "Connect" button.

- [ ] **Step 3: Integrate into `AppShell::render`**

Use `match self.active_view` to render the nodes view.

- [ ] **Step 4: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat(ui): implement Nodes view with list and badges"
```

---

### Task 4: Implement `Subscriptions` View

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Create `render_subscriptions_view` function**

Implement a responsive grid of subscription cards.

- [ ] **Step 2: Implement `subscription_card`**

Include Title, Progress Bar (Traffic), Expiry status, and Update button.

- [ ] **Step 3: Integrate into `AppShell::render`**

- [ ] **Step 4: Verify compilation**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: Success.

- [ ] **Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat(ui): implement Subscriptions view with usage cards"
```

---

### Task 5: Final Memory Update & Cleanup

**Files:**
- Modify: `.memory/status.md`
- Modify: `.memory/tasks.md`
- Modify: `.memory/changelog.md`
- Create: `.prompt/005-phase-4-core-logic-integration.md`

- [ ] **Step 1: Update status and tasks**
- [ ] **Step 2: Write next stage prompt**
- [ ] **Step 3: Commit and push**

```bash
git add .
git commit -m "docs: complete Phase 3 implementation and update status"
git push origin main
```
