# UI Spec — `ui/tools/geoip-query.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/geoip-query.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | geoip query |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 28.43% | 0.992 |
| 2 | `#f9fafd` | 11.98% | 0.98 |
| 3 | `#fafbfd` | 9.18% | 0.984 |
| 4 | `#fefefe` | 8.42% | 0.996 |
| 5 | `#fcfcfd` | 6.0% | 0.989 |
| 6 | `#eff3f7` | 5.91% | 0.951 |
| 7 | `#dae0ea` | 4.99% | 0.876 |
| 8 | `#a4acc8` | 4.3% | 0.676 |
| 9 | `#f8f9fc` | 4.11% | 0.976 |
| 10 | `#fefdfe` | 3.76% | 0.993 |
| 11 | `#ffffff` | 3.66% | 1.0 |
| 12 | `#fcfdfe` | 3.1% | 0.992 |
| 13 | `#fdfdfd` | 2.44% | 0.992 |
| 14 | `#fdfefe` | 2.27% | 0.995 |
| 15 | `#f7fafb` | 1.08% | 0.978 |
| 16 | `#fdfdff` | 0.38% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#95a5b7` |
| top_center | `#c8ced6` |
| top_right | `#919dab` |
| center | `#fffffe` |
| bottom_left | `#a0a7b6` |
| bottom_center | `#c9ccd6` |
| bottom_right | `#9aa2b3` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 20.02 |
| x | 54 | 10.15 |
| x | 81 | 8.16 |
| x | 102 | 12.83 |
| x | 116 | 9.73 |
| x | 314 | 13.72 |
| x | 334 | 20.11 |
| x | 354 | 21.26 |
| x | 389 | 8.74 |
| x | 402 | 7.38 |
| x | 559 | 8.1 |
| x | 778 | 9.44 |
| x | 826 | 11.83 |
| x | 1133 | 19.36 |
| x | 1146 | 16.26 |
| x | 1208 | 7.03 |
| y | 34 | 18.81 |
| y | 61 | 18.82 |
| y | 75 | 28.43 |
| y | 88 | 27.08 |
| y | 136 | 19.13 |
| y | 177 | 23.17 |
| y | 259 | 17.64 |
| y | 273 | 17.51 |
| y | 450 | 23.32 |
| y | 464 | 24.44 |
| y | 532 | 16.04 |
| y | 559 | 27.26 |
| y | 573 | 21.43 |
| y | 607 | 18.63 |
| y | 976 | 23.45 |
| y | 989 | 26.35 |


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
