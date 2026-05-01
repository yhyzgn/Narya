# UI Spec — `ui/config/visual-editor.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/config/visual-editor.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `config detail/state` |
| Human title | visual editor |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 17.06% | 0.992 |
| 2 | `#f9fafd` | 12.91% | 0.98 |
| 3 | `#fdfefe` | 11.13% | 0.995 |
| 4 | `#fefefe` | 10.84% | 0.996 |
| 5 | `#f3f5f9` | 9.82% | 0.96 |
| 6 | `#fafbfd` | 8.73% | 0.984 |
| 7 | `#bbc2d7` | 7.1% | 0.761 |
| 8 | `#ffffff` | 6.26% | 1.0 |
| 9 | `#fcfdfe` | 5.87% | 0.992 |
| 10 | `#fbfcfd` | 4.99% | 0.988 |
| 11 | `#fefeff` | 1.74% | 0.996 |
| 12 | `#fdfdfd` | 1.29% | 0.992 |
| 13 | `#fffefe` | 0.97% | 0.997 |
| 14 | `#fefdfe` | 0.56% | 0.993 |
| 15 | `#fdfdff` | 0.41% | 0.993 |
| 16 | `#fefefd` | 0.32% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a3b0c0` |
| top_center | `#a9b5c5` |
| top_right | `#a2adb9` |
| center | `#fdfdfe` |
| bottom_left | `#aab1bf` |
| bottom_center | `#b1bac9` |
| bottom_right | `#a7aebe` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 30.18 |
| x | 61 | 11.67 |
| x | 88 | 10.65 |
| x | 102 | 11.85 |
| x | 116 | 15.41 |
| x | 266 | 9.05 |
| x | 320 | 9.86 |
| x | 341 | 15.2 |
| x | 361 | 18.28 |
| x | 423 | 9.36 |
| x | 587 | 10.7 |
| x | 723 | 9.68 |
| x | 744 | 10.17 |
| x | 1092 | 10.4 |
| x | 1146 | 16.74 |
| x | 1208 | 9.45 |
| y | 34 | 14.56 |
| y | 61 | 19.11 |
| y | 75 | 32.66 |
| y | 88 | 33.81 |
| y | 136 | 19.53 |
| y | 163 | 16.75 |
| y | 177 | 26.64 |
| y | 191 | 17.88 |
| y | 204 | 18.67 |
| y | 259 | 22.49 |
| y | 273 | 20.5 |
| y | 293 | 16.63 |
| y | 607 | 16.95 |
| y | 832 | 23.17 |
| y | 880 | 31.07 |
| y | 983 | 19.0 |


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
