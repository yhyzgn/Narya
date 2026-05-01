# Establish Theme Module and Design Tokens Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a centralized `theme` module in the Narya GPUI client to manage Design Tokens and integrate it into the main application.

**Architecture:**
- Create `src/theme.rs` to hold the `Theme` struct with color tokens.
- Implement `Default` for `Theme` using values from the design spec.
- Modify `src/main.rs` to expose the `theme` module and make the `Theme` struct available.

**Tech Stack:**
- Rust
- GPUI

---

### Task 1: Create `src/theme.rs`

**Files:**
- Create: `src/theme.rs`

- [x] **Step 1: Write the implementation of `src/theme.rs`**

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
            surface: Rgba { r: 1.0, g: 1.0, b: 1.0, a: 0.8 },
            border: Rgba { r: 0.88, g: 0.90, b: 0.92, a: 0.5 },
            primary: rgb(0x4f46e5),
            success: rgb(0x10b981),
            text_primary: rgb(0x1e293b),
            text_secondary: rgb(0x64748b),
        }
    }
}
```

- [x] **Step 2: Save the file**
Save the content above to `src/theme.rs`.

---

### Task 2: Integrate Theme into `src/main.rs`

**Files:**
- Modify: `src/main.rs`

- [x] **Step 1: Add module declaration and use statement**

```rust
use gpui::{prelude::*, *};

mod theme;
use theme::Theme;

struct HelloWorld {
// ...
```

- [x] **Step 2: Apply the change using `replace`**

---

### Task 3: Verification

**Files:**
- N/A

- [x] **Step 1: Run `cargo check`**

Run: `cargo check`
Expected: Successful compilation without errors related to the new module.

---

### Task 4: Cleanup (Optional but good practice)

- [x] **Step 1: Commit the changes (if requested by user, but user said "Do not commit yet")**
Wait for further instructions.
