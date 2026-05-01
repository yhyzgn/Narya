# UI Spec — `ui/config/yaml-editor.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/config/yaml-editor.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `config detail/state` |
| Human title | yaml editor |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 20.53% | 0.996 |
| 2 | `#fdfdfe` | 13.86% | 0.992 |
| 3 | `#f9fafc` | 9.4% | 0.98 |
| 4 | `#fafbfd` | 8.99% | 0.984 |
| 5 | `#f4f5fa` | 8.9% | 0.961 |
| 6 | `#c0c3de` | 7.69% | 0.77 |
| 7 | `#fcfcfd` | 6.02% | 0.989 |
| 8 | `#fafafd` | 5.49% | 0.981 |
| 9 | `#fbfbfd` | 4.83% | 0.985 |
| 10 | `#ffffff` | 4.42% | 1.0 |
| 11 | `#fefdfe` | 4.17% | 0.993 |
| 12 | `#fdfefe` | 2.02% | 0.995 |
| 13 | `#fefeff` | 1.21% | 0.996 |
| 14 | `#fffefe` | 0.95% | 0.997 |
| 15 | `#fefefd` | 0.8% | 0.996 |
| 16 | `#fdfcfd` | 0.72% | 0.989 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#9facbd` |
| top_center | `#d7dde5` |
| top_right | `#9faab6` |
| center | `#ffffff` |
| bottom_left | `#9ca4b4` |
| bottom_center | `#e1e3ed` |
| bottom_right | `#9299aa` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 18.04 |
| x | 54 | 12.96 |
| x | 81 | 12.16 |
| x | 109 | 13.5 |
| x | 286 | 23.6 |
| x | 307 | 12.74 |
| x | 327 | 16.73 |
| x | 546 | 8.52 |
| x | 559 | 12.21 |
| x | 607 | 8.11 |
| x | 621 | 7.82 |
| x | 655 | 8.42 |
| x | 1167 | 7.96 |
| x | 1181 | 10.71 |
| x | 1235 | 8.92 |
| x | 1276 | 6.67 |
| y | 34 | 25.72 |
| y | 54 | 18.39 |
| y | 75 | 22.48 |
| y | 122 | 23.64 |
| y | 211 | 13.99 |
| y | 225 | 17.5 |
| y | 375 | 13.74 |
| y | 423 | 18.36 |
| y | 436 | 20.84 |
| y | 505 | 17.81 |
| y | 532 | 15.73 |
| y | 723 | 16.97 |
| y | 853 | 23.19 |
| y | 921 | 21.8 |
| y | 962 | 38.31 |
| y | 989 | 16.19 |


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


### config secondary-state notes

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
