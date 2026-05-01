# UI Spec — `ui/tools/diagnostic-report.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/diagnostic-report.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | diagnostic report |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fcfdfe` | 16.62% | 0.992 |
| 2 | `#f9fafd` | 15.88% | 0.98 |
| 3 | `#fdfdfe` | 14.12% | 0.992 |
| 4 | `#fafbfd` | 13.29% | 0.984 |
| 5 | `#fefefe` | 10.26% | 0.996 |
| 6 | `#f3f5f9` | 8.61% | 0.96 |
| 7 | `#c1c7d9` | 8.39% | 0.78 |
| 8 | `#fbfcfe` | 6.37% | 0.988 |
| 9 | `#fcfdfd` | 2.59% | 0.991 |
| 10 | `#fdfdff` | 1.25% | 0.993 |
| 11 | `#fdfdfd` | 0.73% | 0.992 |
| 12 | `#fefdfe` | 0.61% | 0.993 |
| 13 | `#fcfdff` | 0.41% | 0.992 |
| 14 | `#fdfcfd` | 0.36% | 0.989 |
| 15 | `#fbfdfe` | 0.27% | 0.991 |
| 16 | `#fcfefe` | 0.26% | 0.994 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#afbac8` |
| top_center | `#8696aa` |
| top_right | `#b3bec9` |
| center | `#fcfdfe` |
| bottom_left | `#bdc3cf` |
| bottom_center | `#75849d` |
| bottom_right | `#bbc3d1` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 19.06 |
| x | 54 | 13.78 |
| x | 68 | 7.01 |
| x | 88 | 9.58 |
| x | 102 | 14.11 |
| x | 116 | 7.63 |
| x | 266 | 8.4 |
| x | 320 | 20.69 |
| x | 341 | 15.66 |
| x | 361 | 20.53 |
| x | 416 | 7.33 |
| x | 559 | 10.13 |
| x | 573 | 12.63 |
| x | 621 | 11.78 |
| x | 1078 | 18.45 |
| x | 1419 | 8.83 |
| y | 34 | 16.43 |
| y | 61 | 14.57 |
| y | 75 | 28.84 |
| y | 88 | 25.28 |
| y | 157 | 17.12 |
| y | 170 | 20.5 |
| y | 197 | 16.82 |
| y | 314 | 15.5 |
| y | 416 | 18.35 |
| y | 443 | 12.84 |
| y | 525 | 15.18 |
| y | 621 | 18.84 |
| y | 655 | 15.21 |
| y | 737 | 13.52 |
| y | 969 | 18.88 |
| y | 983 | 27.63 |


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
