# UI Spec — `ui/tools/speed-test.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/speed-test.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | speed test |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 20.44% | 0.992 |
| 2 | `#fafbfe` | 18.21% | 0.984 |
| 3 | `#f0f3f7` | 10.46% | 0.952 |
| 4 | `#fefefe` | 9.83% | 0.996 |
| 5 | `#b9c1d8` | 7.47% | 0.757 |
| 6 | `#fcfcfe` | 6.29% | 0.989 |
| 7 | `#f8fafc` | 6.08% | 0.979 |
| 8 | `#fefeff` | 5.47% | 0.996 |
| 9 | `#ffffff` | 5.0% | 1.0 |
| 10 | `#fdfefe` | 3.38% | 0.995 |
| 11 | `#fefdfe` | 2.49% | 0.993 |
| 12 | `#fcfdfe` | 2.16% | 0.992 |
| 13 | `#fdfdff` | 1.09% | 0.993 |
| 14 | `#fdfdfd` | 0.88% | 0.992 |
| 15 | `#fffeff` | 0.68% | 0.997 |
| 16 | `#fefefd` | 0.07% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#aab5c5` |
| top_center | `#a2aec2` |
| top_right | `#a5b0bd` |
| center | `#fefefe` |
| bottom_left | `#aab0bf` |
| bottom_center | `#a3aebe` |
| bottom_right | `#aab0c0` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 25.29 |
| x | 54 | 16.45 |
| x | 68 | 7.46 |
| x | 81 | 13.61 |
| x | 95 | 7.82 |
| x | 109 | 9.95 |
| x | 307 | 14.53 |
| x | 327 | 14.91 |
| x | 348 | 10.76 |
| x | 532 | 8.49 |
| x | 1140 | 26.51 |
| x | 1153 | 12.54 |
| x | 1181 | 11.91 |
| x | 1194 | 13.23 |
| x | 1256 | 10.93 |
| x | 1488 | 7.34 |
| y | 61 | 21.55 |
| y | 75 | 18.14 |
| y | 88 | 22.11 |
| y | 177 | 17.81 |
| y | 252 | 20.58 |
| y | 266 | 21.71 |
| y | 314 | 23.91 |
| y | 327 | 29.35 |
| y | 361 | 38.68 |
| y | 395 | 49.18 |
| y | 457 | 22.93 |
| y | 532 | 23.0 |
| y | 546 | 18.74 |
| y | 607 | 21.19 |
| y | 976 | 23.35 |
| y | 989 | 21.18 |


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


### tools secondary-state notes

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
