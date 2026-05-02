# Phase 3 Design: Business Pages & Enum Routing

**Goal:** Implement view routing and restore `Nodes` and `Subscriptions` pages in GPUI.

**Architecture:** Use a simple `ActiveView` enum within the `AppShell` state. The content area will dynamically render different page layouts based on this enum.

**Tech Stack:** Rust, GPUI.

---

## 1. Routing State Design

We will add an `active_view` field to the `AppShell` struct.

```rust
#[derive(Clone, Copy, PartialEq)]
enum ActiveView {
    Dashboard,
    Nodes,
    Subscriptions,
    Rules,
    Settings,
}

struct AppShell {
    active_view: ActiveView,
}
```

## 2. Navigation Mechanism

- **SideBar Updates**: Each `nav_item` will become a clickable element.
- **Click Handler**: When a navigation item is clicked, it will update `active_view` and call `cx.notify()`.

```rust
// Inside AppShell::render
.child(nav_item("Dashboard", self.active_view == ActiveView::Dashboard, cx, |_, cx| {
    this.update(cx, |this, cx| {
        this.active_view = ActiveView::Dashboard;
        cx.notify();
    });
}))
```

## 3. View Implementations

### Dashboard View (Existing)
- Refactor existing dashboard content into a `render_dashboard` function or keep it within the `match` block.

### Nodes View
- **Toolbar**: Search input (641px wide candidate) and filter badges.
- **List**: Vertical scrollable list of node cards.
- **Node Card**:
  - Name: "SG-01 (Singapore)"
  - Protocol Badge: "Shadowsocks" (Cold Blue)
  - Latency Badge: "12ms" (Success Green)
  - Action: "Connect" button.

### Subscriptions View
- **Card Grid**: Display subscription cards in a responsive grid.
- **Subscription Card**:
  - Title: "Premium Plan"
  - Progress Bar: Traffic usage (e.g., 45/100 GB).
  - Status: "Expires in 15 days".
  - Actions: "Update", "Copy Link".

## 4. Reusable Components

- `Badge(label, color)`: Small rounded container for protocol/latency.
- `SearchInput()`: Input field with 1px border and search icon placeholder.
- `PageHeader(title)`: Consistent title styling for all pages.

## 5. Data Flow (Mock)

For Phase 3, we will use mock data vectors within the `AppShell` or as constants to drive the list rendering.

---

## 6. Testing & Verification

- **Manual Verification**: Run the app and click through all sidebar items.
- **Layout Check**: Verify `Nodes` and `Subscriptions` margins (302px left start, 120px top start) match specs.
- **Headless Check**: Ensure `cargo clippy` and `cargo fmt` are clean.
