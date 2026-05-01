# UI Spec — `ui/nodes/latency-test.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/nodes/latency-test.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `nodes detail/state` |
| Human title | latency test |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 29.92% | 0.996 |
| 2 | `#f0f2f7` | 7.32% | 0.949 |
| 3 | `#b8bcd3` | 6.87% | 0.74 |
| 4 | `#fafbfe` | 6.53% | 0.984 |
| 5 | `#ffffff` | 6.49% | 1.0 |
| 6 | `#fbfbfd` | 6.3% | 0.985 |
| 7 | `#fdfdfe` | 6.18% | 0.992 |
| 8 | `#f8fafc` | 5.9% | 0.979 |
| 9 | `#fbfcfe` | 5.8% | 0.988 |
| 10 | `#fefdfe` | 4.11% | 0.993 |
| 11 | `#fdfdfd` | 3.88% | 0.992 |
| 12 | `#fcfdfd` | 3.43% | 0.991 |
| 13 | `#fdfefe` | 2.98% | 0.995 |
| 14 | `#fefefd` | 1.95% | 0.996 |
| 15 | `#fffefe` | 1.5% | 0.997 |
| 16 | `#fefeff` | 0.83% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#b1bdcd` |
| top_center | `#dae0e6` |
| top_right | `#b6bfca` |
| center | `#f2fdfa` |
| bottom_left | `#bbc2d1` |
| bottom_center | `#e6e7ee` |
| bottom_right | `#b8becd` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 24.95 |
| x | 54 | 14.0 |
| x | 95 | 10.85 |
| x | 109 | 10.15 |
| x | 293 | 12.47 |
| x | 307 | 9.65 |
| x | 498 | 12.82 |
| x | 525 | 22.87 |
| x | 539 | 20.97 |
| x | 559 | 13.3 |
| x | 580 | 11.78 |
| x | 614 | 13.72 |
| x | 634 | 10.87 |
| x | 887 | 12.25 |
| x | 1201 | 14.67 |
| x | 1488 | 12.07 |
| y | 191 | 19.3 |
| y | 307 | 13.95 |
| y | 320 | 33.98 |
| y | 334 | 16.48 |
| y | 361 | 16.65 |
| y | 375 | 15.71 |
| y | 409 | 19.76 |
| y | 512 | 13.71 |
| y | 552 | 18.39 |
| y | 587 | 14.76 |
| y | 675 | 18.65 |
| y | 785 | 13.6 |
| y | 826 | 37.21 |
| y | 887 | 35.07 |
| y | 983 | 13.7 |
| y | 996 | 21.69 |


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
