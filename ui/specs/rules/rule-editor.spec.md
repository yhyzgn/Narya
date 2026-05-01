# UI Spec — `ui/rules/rule-editor.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/rules/rule-editor.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `rules detail/state` |
| Human title | rule editor |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 17.07% | 0.996 |
| 2 | `#f9fafd` | 13.85% | 0.98 |
| 3 | `#fdfdfe` | 13.18% | 0.992 |
| 4 | `#fbfcfd` | 11.21% | 0.988 |
| 5 | `#f0f3f8` | 10.06% | 0.952 |
| 6 | `#bec5db` | 7.63% | 0.773 |
| 7 | `#ffffff` | 6.34% | 1.0 |
| 8 | `#fdfefe` | 4.14% | 0.995 |
| 9 | `#f7fafb` | 4.07% | 0.978 |
| 10 | `#fcfdfe` | 3.19% | 0.992 |
| 11 | `#fdfdfd` | 2.72% | 0.992 |
| 12 | `#fefdfe` | 2.62% | 0.993 |
| 13 | `#fefeff` | 1.65% | 0.996 |
| 14 | `#fcfbfb` | 0.97% | 0.985 |
| 15 | `#fffefe` | 0.88% | 0.997 |
| 16 | `#fefefd` | 0.42% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#f6f8fb` |
| top_center | `#a1abbb` |
| top_right | `#b7c0c9` |
| center | `#fdfcfd` |
| bottom_left | `#cad0d9` |
| bottom_center | `#8d9ab0` |
| bottom_right | `#999faf` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 22.85 |
| x | 54 | 19.21 |
| x | 68 | 8.21 |
| x | 81 | 11.99 |
| x | 102 | 11.27 |
| x | 259 | 9.79 |
| x | 320 | 16.46 |
| x | 341 | 16.17 |
| x | 361 | 8.67 |
| x | 628 | 7.95 |
| x | 894 | 17.17 |
| x | 907 | 16.44 |
| x | 921 | 12.9 |
| x | 1003 | 9.86 |
| x | 1078 | 8.51 |
| x | 1160 | 13.45 |
| y | 163 | 28.87 |
| y | 177 | 15.91 |
| y | 232 | 27.42 |
| y | 245 | 16.36 |
| y | 286 | 25.74 |
| y | 300 | 29.69 |
| y | 327 | 20.56 |
| y | 375 | 22.97 |
| y | 464 | 18.58 |
| y | 484 | 33.25 |
| y | 518 | 28.31 |
| y | 593 | 16.01 |
| y | 641 | 15.91 |
| y | 791 | 16.38 |
| y | 969 | 19.37 |
| y | 983 | 18.49 |


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


### rules secondary-state notes

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
