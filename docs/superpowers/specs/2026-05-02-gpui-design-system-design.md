# Phase 1: GPUI Design System Design Spec

## Overview
This document specifies the foundational design system for the Narya GPUI client, including Design Tokens, core UI components, and the overall application shell layout.

## Goals
- Establish a consistent visual language based on the "Glassmorphism" aesthetic.
- Implement a reusable layout structure (AppShell) that follows canonical dimensions.
- Create base UI components (GlassCard) that adhere to the high-fidelity design specs.

## Architecture
- **Theme Module**: A centralized Rust module in the main application to manage Design Tokens.
- **App Context**: Use GPUI's `AppContext` to store and access global theme state.
- **Component Pattern**: Custom UI components implemented as functions or structs that return `impl IntoElement`.

---

## 1. Design Tokens

### 1.1 Colors
| Token | Value | Purpose |
|---|---|---|
| `color_bg` | `#f9fafc` | Main application background (Cold White). |
| `color_surface` | `rgba(255, 255, 255, 0.8)` | High-transparency surface for glass cards. |
| `color_border` | `rgba(225, 230, 236, 0.5)` | Extremely subtle, low-contrast border. |
| `color_primary` | `#4f46e5` | Primary Indigo (Switches, Active States). |
| `color_success` | `#10b981` | Success Green (Status Badges, Toggles). |
| `color_text_primary` | `#1e293b` | Main body/title text (Slate 800). |
| `color_text_secondary` | `#64748b` | Muted/Secondary text (Slate 500). |

### 1.2 Typography
- **Font Family**: `Inter`, `SF Pro Text`, `system-ui`.
- **Body**: 13px/14px, Regular (Weight 400).
- **Subtitle**: 12px/13px, Regular (Weight 400), `color_text_secondary`.
- **Title**: 18px, SemiBold (Weight 600), `color_text_primary`.
- **Emphasized Stats**: 23px, Medium (Weight 500).

### 1.3 Spacing & Radius
- **Base Radius**: 16px for main cards.
- **Inner Radius**: 12px for smaller inner elements (e.g., toggles).
- **Padding**: 24px standard container padding.
- **Blur**: 12px-16px backdrop-filter blur for glass surfaces.

---

## 2. Core Components

### 2.1 GlassCard
A container component that implements the high-fidelity glassy aesthetic.
- **Styles**:
  - Background: `color_surface` (`rgba(255, 255, 255, 0.8)`).
  - Backdrop Filter: `blur(12px)`.
  - Border: 1px `color_border` (`rgba(225, 230, 236, 0.5)`).
  - Shadow: Large-radius ambient shadow (`0 10px 25px -5px rgba(0, 0, 0, 0.05)`).
- **Usage**: Wraps content sections in Dashboard and other pages.

---

## 3. AppShell Layout

### 3.1 Dimensions (Canonical 1536x1024)
- **Sidebar**: Fixed width `270px`. Height `100%`.
- **Header**: Height `118px` (Y: 0 to 118).
- **Main Area**: Responsive width, fills remaining space.
- **Footer**: Fixed height `30px` at the bottom.

### 3.2 Layout Tree
- `div() .flex() .size_full()` (Root)
  - `Sidebar` (Left)
  - `div() .flex_col() .flex_1()` (Main Column)
    - `Header` (Top)
    - `div() .flex_1() .overflow_y_scroll()` (Scrollable Content Area)
    - `Footer` (Bottom)

---

## 4. Testing & Validation
- **Visual Regression**: Compare GPUI implementation against `ui/dashboard.png` at 1536x1024.
- **Clippy**: Ensure zero warnings in the `theme` and layout modules.
- **Performance**: Monitor GPU memory usage for backdrop blur layers.
