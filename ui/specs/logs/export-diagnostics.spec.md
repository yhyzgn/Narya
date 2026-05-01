# UI Spec — `ui/logs/export-diagnostics.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/logs/export-diagnostics.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `logs detail/state` |
| Human title | export diagnostics |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 20.58% | 0.992 |
| 2 | `#f9fafd` | 13.43% | 0.98 |
| 3 | `#fefefe` | 13.13% | 0.996 |
| 4 | `#fbfbfd` | 10.48% | 0.985 |
| 5 | `#f4f5fa` | 8.0% | 0.961 |
| 6 | `#b9bfd9` | 7.77% | 0.751 |
| 7 | `#fefdfe` | 7.72% | 0.993 |
| 8 | `#ffffff` | 4.36% | 1.0 |
| 9 | `#fafbfc` | 3.81% | 0.984 |
| 10 | `#fdfdfd` | 2.72% | 0.992 |
| 11 | `#fdfefe` | 2.64% | 0.995 |
| 12 | `#fcfdfe` | 2.42% | 0.992 |
| 13 | `#fefeff` | 1.89% | 0.996 |
| 14 | `#fffeff` | 0.58% | 0.997 |
| 15 | `#fefefd` | 0.25% | 0.996 |
| 16 | `#fdfdff` | 0.22% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#d3dbe5` |
| top_center | `#e4e9ee` |
| top_right | `#dde3e6` |
| center | `#fdfefe` |
| bottom_left | `#d6dbe1` |
| bottom_center | `#e4e6ec` |
| bottom_right | `#dde1e7` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 20.88 |
| x | 54 | 19.45 |
| x | 68 | 6.03 |
| x | 81 | 9.91 |
| x | 102 | 13.92 |
| x | 259 | 6.3 |
| x | 300 | 14.85 |
| x | 587 | 8.74 |
| x | 634 | 7.57 |
| x | 703 | 7.96 |
| x | 737 | 6.16 |
| x | 805 | 6.14 |
| x | 901 | 8.34 |
| x | 1187 | 21.43 |
| x | 1317 | 6.82 |
| x | 1495 | 14.36 |
| y | 34 | 17.43 |
| y | 61 | 19.18 |
| y | 75 | 28.68 |
| y | 88 | 30.99 |
| y | 197 | 17.58 |
| y | 252 | 16.96 |
| y | 266 | 18.29 |
| y | 368 | 19.74 |
| y | 402 | 21.81 |
| y | 430 | 21.19 |
| y | 491 | 28.89 |
| y | 539 | 37.04 |
| y | 566 | 18.74 |
| y | 634 | 20.0 |
| y | 976 | 23.05 |
| y | 989 | 19.25 |


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


### logs secondary-state notes

- This screenshot describes a specific interaction/detail state, not just a static page.
- Preserve modal/detail panel geometry, table/form density, selected row state, and action hierarchy.
- Element Plus can supply the controls, but the layout, colors, and spacing must be driven by this PNG.

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
