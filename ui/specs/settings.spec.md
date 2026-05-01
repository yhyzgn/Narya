# UI Spec — `ui/settings.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/settings` |
| Human title | 设置 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 20.8% | 0.992 |
| 2 | `#fefefe` | 13.5% | 0.996 |
| 3 | `#f9fafd` | 12.74% | 0.98 |
| 4 | `#fafbfd` | 10.11% | 0.984 |
| 5 | `#f0f3f7` | 7.89% | 0.952 |
| 6 | `#fefdfe` | 7.42% | 0.993 |
| 7 | `#b2b9d8` | 6.66% | 0.728 |
| 8 | `#fcfcfd` | 5.89% | 0.989 |
| 9 | `#ffffff` | 4.41% | 1.0 |
| 10 | `#fdfdfd` | 4.36% | 0.992 |
| 11 | `#fdfefe` | 2.0% | 0.995 |
| 12 | `#fcfdfe` | 1.57% | 0.992 |
| 13 | `#fefeff` | 1.05% | 0.996 |
| 14 | `#fffefe` | 0.9% | 0.997 |
| 15 | `#fefefd` | 0.46% | 0.996 |
| 16 | `#fdfdff` | 0.26% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#b0bccb` |
| top_center | `#939fad` |
| top_right | `#b8c2cb` |
| center | `#fdfdfe` |
| bottom_left | `#bec4d0` |
| bottom_center | `#828c9c` |
| bottom_right | `#bcc4d3` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 19.85 |
| x | 54 | 12.01 |
| x | 81 | 10.55 |
| x | 102 | 15.18 |
| x | 266 | 8.68 |
| x | 320 | 17.95 |
| x | 334 | 20.48 |
| x | 354 | 7.55 |
| x | 382 | 16.75 |
| x | 532 | 7.53 |
| x | 696 | 11.21 |
| x | 750 | 7.83 |
| x | 948 | 8.94 |
| x | 1140 | 19.09 |
| x | 1160 | 12.64 |
| x | 1481 | 9.94 |
| y | 75 | 13.96 |
| y | 88 | 25.8 |
| y | 136 | 13.2 |
| y | 157 | 27.35 |
| y | 170 | 25.81 |
| y | 184 | 13.57 |
| y | 197 | 17.79 |
| y | 266 | 13.35 |
| y | 307 | 15.14 |
| y | 764 | 13.31 |
| y | 853 | 19.61 |
| y | 880 | 26.01 |
| y | 894 | 63.05 |
| y | 928 | 58.39 |
| y | 976 | 23.52 |
| y | 989 | 26.11 |


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


### 设置 primary-page implementation notes

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
