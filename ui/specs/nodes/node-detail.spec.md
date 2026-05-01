# UI Spec — `ui/nodes/node-detail.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/nodes/node-detail.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `nodes detail/state` |
| Human title | node detail |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 29.84% | 0.992 |
| 2 | `#f9fafd` | 10.47% | 0.98 |
| 3 | `#fafbfd` | 9.3% | 0.984 |
| 4 | `#fefefe` | 7.21% | 0.996 |
| 5 | `#fcfcfd` | 6.87% | 0.989 |
| 6 | `#eaf2f3` | 6.63% | 0.943 |
| 7 | `#bdc6d5` | 6.47% | 0.773 |
| 8 | `#f7f8fb` | 5.15% | 0.973 |
| 9 | `#fcfdfe` | 4.42% | 0.992 |
| 10 | `#fefdfe` | 3.34% | 0.993 |
| 11 | `#ffffff` | 3.19% | 1.0 |
| 12 | `#fdfefe` | 2.88% | 0.995 |
| 13 | `#fdfdfd` | 2.26% | 0.992 |
| 14 | `#f6faf9` | 1.08% | 0.977 |
| 15 | `#f9fbfe` | 0.56% | 0.983 |
| 16 | `#fdfdff` | 0.35% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#dce5f2` |
| top_center | `#e2e5ec` |
| top_right | `#e1e9f0` |
| center | `#fdfdfe` |
| bottom_left | `#e3e9f0` |
| bottom_center | `#e1e5ed` |
| bottom_right | `#dbe1ec` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 22.52 |
| x | 54 | 21.07 |
| x | 81 | 15.64 |
| x | 109 | 12.61 |
| x | 300 | 26.06 |
| x | 348 | 8.66 |
| x | 416 | 7.55 |
| x | 593 | 8.75 |
| x | 655 | 10.85 |
| x | 778 | 11.23 |
| x | 832 | 8.41 |
| x | 1037 | 9.82 |
| x | 1153 | 8.71 |
| x | 1167 | 8.27 |
| x | 1324 | 10.04 |
| x | 1454 | 9.2 |
| y | 81 | 25.47 |
| y | 177 | 24.27 |
| y | 204 | 15.38 |
| y | 232 | 23.42 |
| y | 286 | 17.58 |
| y | 300 | 18.11 |
| y | 327 | 19.6 |
| y | 368 | 11.03 |
| y | 634 | 11.16 |
| y | 757 | 12.46 |
| y | 778 | 10.15 |
| y | 832 | 18.45 |
| y | 866 | 11.32 |
| y | 880 | 15.11 |
| y | 983 | 21.56 |
| y | 996 | 19.67 |


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


### nodes secondary-state notes

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
