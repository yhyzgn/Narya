# UI Spec — `ui/dashboard/system-proxy-rule-mode.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/dashboard/system-proxy-rule-mode.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `dashboard detail/state` |
| Human title | system proxy rule mode |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 17.7% | 0.996 |
| 2 | `#f9fafc` | 13.05% | 0.98 |
| 3 | `#fdfdfd` | 9.59% | 0.992 |
| 4 | `#eff3f7` | 9.38% | 0.951 |
| 5 | `#fefdfd` | 7.4% | 0.993 |
| 6 | `#b9c2d4` | 6.81% | 0.758 |
| 7 | `#fbfcfc` | 6.8% | 0.987 |
| 8 | `#fefefd` | 5.72% | 0.996 |
| 9 | `#fafafc` | 5.13% | 0.981 |
| 10 | `#f6f9fb` | 4.92% | 0.975 |
| 11 | `#fafbfd` | 4.29% | 0.984 |
| 12 | `#ffffff` | 3.4% | 1.0 |
| 13 | `#fdfefe` | 2.77% | 0.995 |
| 14 | `#fdfcfd` | 1.69% | 0.989 |
| 15 | `#fffefe` | 1.04% | 0.997 |
| 16 | `#fefeff` | 0.32% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#9eabbb` |
| top_center | `#d3d9e0` |
| top_right | `#a2acb8` |
| center | `#fefefe` |
| bottom_left | `#a2a9b7` |
| bottom_center | `#d1d4e1` |
| bottom_right | `#a2aabc` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 26.43 |
| x | 61 | 12.72 |
| x | 81 | 12.8 |
| x | 116 | 13.38 |
| x | 320 | 13.0 |
| x | 334 | 11.87 |
| x | 348 | 11.31 |
| x | 361 | 13.02 |
| x | 382 | 26.13 |
| x | 593 | 10.15 |
| x | 812 | 10.33 |
| x | 839 | 10.65 |
| x | 894 | 15.32 |
| x | 955 | 12.94 |
| x | 1201 | 11.6 |
| x | 1358 | 14.32 |
| y | 109 | 15.18 |
| y | 143 | 32.39 |
| y | 157 | 16.92 |
| y | 341 | 11.9 |
| y | 477 | 12.18 |
| y | 539 | 12.3 |
| y | 552 | 13.95 |
| y | 662 | 24.46 |
| y | 675 | 25.0 |
| y | 696 | 12.85 |
| y | 709 | 12.73 |
| y | 723 | 16.01 |
| y | 791 | 12.61 |
| y | 832 | 17.05 |
| y | 969 | 16.22 |
| y | 983 | 24.61 |


### Shared app-shell geometry for 1536×1024 full-page references

These measurements apply to the main application screenshots unless the source image is a modal/detail crop or icon asset.

| Region | Target measurement | Implementation note |
|---|---:|---|
| Canvas | 1536×1024 | Treat as the canonical reference viewport for page screenshots. |
| Sidebar | x=0, y=0, w≈270, h=1024 | Fixed-width visual rail; do not make it fluid. |
| Sidebar separator | x≈270 | Thin 1px blue-gray border. |
| Main area | x≈270, y=0, w≈1266, h=1024 | Right side owns responsive growth when window widens. |
| Header/title band | y≈0–118 | Native Tauri titlebar may add OS chrome; web content should preserve the visual spacing inside the WebView. |
| Content start | x≈302, y≈120 | Cards begin after the header breathing space. |
| Footer/status band | y≈955–1024 | Thin top border; status text is compact and muted. |
| Card radius | 10–12px | Most main cards use small rounded corners, not large pill radii. |
| Border | rgba(126,143,170,0.18–0.26) | Borders are soft and low-contrast. |
| Shadow | subtle blue-gray ambient shadow | Avoid heavy Material shadows. |


### Global Narya visual rules

- Visual truth source: the referenced PNG itself; do not reinterpret proportions from memory.
- Preferred font stack: `SF Pro Text`, `Inter`, `Segoe UI`, `Noto Sans SC`, system-ui, sans-serif.
- Overall type density: compact desktop UI, closer to 12–14px body text and 16–21px card titles.
- Emphasized metric numerals: avoid oversized dashboard numerals; current dashboard density uses around 23px for major stats.
- Icon style: Lucide-like 2–2.35 stroke, compact sizes; avoid oversized decorative icons except explicit gradient card icons.
- Surface style: glassy white/translucent cards over pale blue background; keep borders and shadows restrained.
- Element Plus strategy: business pages may use Element Plus, but theme variables must be overridden to Narya tokens before usage.
- Do not use default Element Plus blue/radius/row height directly.
- Dashboard/Splash/AppShell remain custom high-fidelity implementations unless a component is purely behavioral (tooltip/popover/dialog).


### Asset-specific notes

- Use this PNG as the precise visual source for dimensions, color, and composition.
- If implemented in code, first classify whether it is a full page, modal state, panel state, or standalone asset.

## 5. Gemini implementation checklist

- [ ] Open the source PNG and this spec side by side.
- [ ] Lock the canvas/window assumptions before coding.
- [ ] Extract layout first: shell, header, sidebar, card grid, dialogs/panels.
- [ ] Extract design tokens second: background, surface, border, radius, shadow, text colors, icon sizes.
- [ ] Implement static layout before adding state or IPC.
- [ ] For Element Plus pages, theme controls before using them in screenshots.
- [ ] For Dashboard/Splash/AppShell, do not replace custom visual components with default Element Plus components.
- [ ] Produce a screenshot at the same viewport size and compare against the PNG.
- [ ] Record any intentional deviation explicitly in the relevant memory/handoff file.

## 6. Notes for future maintainers

- This spec is generated from raster evidence and path semantics; it is intentionally verbose so another AI agent can avoid guessing.
- If source design files become available, replace generated boundary hints with exact Figma/Sketch measurements.
- Do not delete the original PNG; it remains the visual truth source.
