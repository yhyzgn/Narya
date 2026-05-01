# UI Spec — `ui/tools/mtr-trace.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/mtr-trace.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | mtr trace |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfefe` | 16.56% | 0.995 |
| 2 | `#fdfdfe` | 15.08% | 0.992 |
| 3 | `#f7f9fd` | 13.8% | 0.976 |
| 4 | `#fbfbfd` | 11.76% | 0.985 |
| 5 | `#fefeff` | 10.41% | 0.996 |
| 6 | `#edf1f6` | 8.55% | 0.943 |
| 7 | `#fcfdfe` | 8.06% | 0.992 |
| 8 | `#bcc2d8` | 6.4% | 0.762 |
| 9 | `#f9fafd` | 4.05% | 0.98 |
| 10 | `#fdfeff` | 2.4% | 0.996 |
| 11 | `#fafafc` | 1.22% | 0.981 |
| 12 | `#fdfdfd` | 0.68% | 0.992 |
| 13 | `#fdfdff` | 0.46% | 0.993 |
| 14 | `#fefdfe` | 0.28% | 0.993 |
| 15 | `#fdfefd` | 0.23% | 0.995 |
| 16 | `#fdffff` | 0.08% | 0.998 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#aeb6c1` |
| top_center | `#7b8aa2` |
| top_right | `#aab2b6` |
| center | `#fefeff` |
| bottom_left | `#b6b7bb` |
| bottom_center | `#727f93` |
| bottom_right | `#abadb0` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 18.69 |
| x | 54 | 15.37 |
| x | 81 | 9.4 |
| x | 109 | 14.42 |
| x | 327 | 13.18 |
| x | 341 | 14.92 |
| x | 361 | 10.35 |
| x | 423 | 7.41 |
| x | 552 | 8.29 |
| x | 791 | 6.95 |
| x | 812 | 7.89 |
| x | 826 | 7.4 |
| x | 887 | 8.67 |
| x | 1010 | 13.36 |
| x | 1064 | 9.76 |
| x | 1146 | 15.35 |
| y | 34 | 15.7 |
| y | 61 | 20.69 |
| y | 81 | 19.88 |
| y | 122 | 11.62 |
| y | 143 | 18.58 |
| y | 163 | 18.18 |
| y | 184 | 13.53 |
| y | 252 | 13.65 |
| y | 300 | 12.8 |
| y | 334 | 35.58 |
| y | 368 | 37.63 |
| y | 628 | 13.62 |
| y | 662 | 14.24 |
| y | 764 | 12.24 |
| y | 969 | 23.07 |
| y | 983 | 27.44 |


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
