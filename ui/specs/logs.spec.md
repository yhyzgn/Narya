# UI Spec — `ui/logs.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/logs.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/logs` |
| Human title | 日志 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 28.52% | 0.996 |
| 2 | `#fafbfd` | 10.11% | 0.984 |
| 3 | `#ffffff` | 9.1% | 1.0 |
| 4 | `#fdfdfe` | 6.91% | 0.992 |
| 5 | `#fbfcfe` | 6.49% | 0.988 |
| 6 | `#fbfcfd` | 6.01% | 0.988 |
| 7 | `#eaf1f4` | 5.98% | 0.94 |
| 8 | `#dddfe2` | 5.84% | 0.874 |
| 9 | `#acadd0` | 4.41% | 0.688 |
| 10 | `#f7fafb` | 3.88% | 0.978 |
| 11 | `#fffefe` | 3.81% | 0.997 |
| 12 | `#f7f6f8` | 3.18% | 0.966 |
| 13 | `#fffffe` | 2.08% | 1.0 |
| 14 | `#fefeff` | 1.69% | 0.996 |
| 15 | `#fefdfe` | 1.21% | 0.993 |
| 16 | `#fefefd` | 0.79% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#aab6c6` |
| top_center | `#b6c0cd` |
| top_right | `#a2adba` |
| center | `#fefefe` |
| bottom_left | `#afb6c6` |
| bottom_center | `#b8bfcf` |
| bottom_right | `#a7aebf` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 18.91 |
| x | 54 | 16.13 |
| x | 81 | 13.17 |
| x | 109 | 15.0 |
| x | 252 | 6.75 |
| x | 293 | 17.72 |
| x | 314 | 9.89 |
| x | 334 | 7.43 |
| x | 525 | 9.43 |
| x | 566 | 9.75 |
| x | 607 | 9.06 |
| x | 737 | 7.47 |
| x | 1064 | 19.2 |
| x | 1099 | 8.97 |
| x | 1256 | 8.36 |
| x | 1488 | 7.61 |
| y | 68 | 16.04 |
| y | 81 | 19.27 |
| y | 170 | 16.95 |
| y | 184 | 14.78 |
| y | 225 | 28.13 |
| y | 238 | 28.07 |
| y | 382 | 23.17 |
| y | 552 | 16.37 |
| y | 682 | 19.66 |
| y | 703 | 14.3 |
| y | 771 | 17.68 |
| y | 805 | 24.7 |
| y | 832 | 24.17 |
| y | 921 | 25.26 |
| y | 983 | 17.31 |
| y | 996 | 23.01 |


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


### 日志 primary-page implementation notes

- This is a business page and may use Element Plus for interaction-heavy controls.
- Keep AppShell geometry consistent with the 1536×1024 full-page references.
- Use Element Plus only after `tokens.css` and `element-plus-theme.css` are loaded.
- Match the reference page's card density, table row height, filters, side panels, and empty-space rhythm before adding dynamic data.
- Preserve the page title, toolbar hierarchy, and footer status band from the PNG.

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
