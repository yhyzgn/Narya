# UI Spec — `ui/tools/dns-query.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/dns-query.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | dns query |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 18.51% | 0.992 |
| 2 | `#fdfefe` | 14.81% | 0.995 |
| 3 | `#f9fafd` | 14.12% | 0.98 |
| 4 | `#fafbfd` | 9.26% | 0.984 |
| 5 | `#f2f4f9` | 8.25% | 0.957 |
| 6 | `#fefefe` | 7.45% | 0.996 |
| 7 | `#fcfdfe` | 7.38% | 0.992 |
| 8 | `#b9c0d8` | 7.07% | 0.754 |
| 9 | `#fbfcfd` | 5.39% | 0.988 |
| 10 | `#ffffff` | 4.32% | 1.0 |
| 11 | `#fdfdfd` | 1.33% | 0.992 |
| 12 | `#fdfeff` | 0.96% | 0.996 |
| 13 | `#fefdfe` | 0.44% | 0.993 |
| 14 | `#fdfefd` | 0.4% | 0.995 |
| 15 | `#fdfdff` | 0.26% | 0.993 |
| 16 | `#fdffff` | 0.07% | 0.998 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a7b5c5` |
| top_center | `#b8c1cd` |
| top_right | `#a8b1bc` |
| center | `#fcfdfe` |
| bottom_left | `#a7aebd` |
| bottom_center | `#d2d6df` |
| bottom_right | `#aab2c1` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 19.51 |
| x | 54 | 11.5 |
| x | 81 | 9.31 |
| x | 102 | 15.77 |
| x | 116 | 10.88 |
| x | 259 | 9.98 |
| x | 314 | 15.45 |
| x | 334 | 14.28 |
| x | 348 | 19.78 |
| x | 416 | 8.6 |
| x | 539 | 12.15 |
| x | 552 | 12.38 |
| x | 798 | 7.99 |
| x | 832 | 9.98 |
| x | 1064 | 10.44 |
| x | 1187 | 18.65 |
| y | 75 | 17.72 |
| y | 88 | 25.08 |
| y | 136 | 22.82 |
| y | 177 | 22.59 |
| y | 197 | 16.74 |
| y | 252 | 20.79 |
| y | 266 | 22.92 |
| y | 320 | 18.15 |
| y | 354 | 34.53 |
| y | 389 | 33.2 |
| y | 573 | 20.92 |
| y | 682 | 16.25 |
| y | 723 | 18.8 |
| y | 737 | 18.22 |
| y | 976 | 23.39 |
| y | 989 | 24.55 |


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
