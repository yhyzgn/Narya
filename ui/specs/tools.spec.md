# UI Spec — `ui/tools.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/tools` |
| Human title | 工具箱 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 22.12% | 0.992 |
| 2 | `#f9fafd` | 14.49% | 0.98 |
| 3 | `#fefefe` | 11.96% | 0.996 |
| 4 | `#fafbfd` | 10.51% | 0.984 |
| 5 | `#f3f5f9` | 9.47% | 0.96 |
| 6 | `#bbc1dd` | 7.88% | 0.76 |
| 7 | `#fcfcfd` | 5.07% | 0.989 |
| 8 | `#ffffff` | 4.31% | 1.0 |
| 9 | `#fefdfe` | 3.8% | 0.993 |
| 10 | `#fdfefe` | 2.81% | 0.995 |
| 11 | `#fdfdfd` | 2.36% | 0.992 |
| 12 | `#fefeff` | 1.94% | 0.996 |
| 13 | `#fcfdfe` | 1.91% | 0.992 |
| 14 | `#fffeff` | 0.88% | 0.997 |
| 15 | `#fdfdff` | 0.24% | 0.993 |
| 16 | `#fefefd` | 0.24% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#99a8b9` |
| top_center | `#cad0d9` |
| top_right | `#a0abb8` |
| center | `#fdfdfe` |
| bottom_left | `#9fa7b7` |
| bottom_center | `#cacfd9` |
| bottom_right | `#9fa7b7` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 20.54 |
| x | 54 | 10.33 |
| x | 81 | 11.23 |
| x | 109 | 16.14 |
| x | 252 | 9.38 |
| x | 300 | 24.98 |
| x | 327 | 28.39 |
| x | 354 | 13.13 |
| x | 566 | 10.65 |
| x | 607 | 8.9 |
| x | 846 | 8.95 |
| x | 866 | 8.57 |
| x | 942 | 8.57 |
| x | 1017 | 17.24 |
| x | 1160 | 11.24 |
| x | 1256 | 11.74 |
| y | 68 | 13.22 |
| y | 81 | 21.22 |
| y | 163 | 21.94 |
| y | 177 | 18.44 |
| y | 191 | 13.21 |
| y | 307 | 14.61 |
| y | 327 | 24.75 |
| y | 361 | 35.76 |
| y | 518 | 23.59 |
| y | 669 | 14.27 |
| y | 798 | 12.51 |
| y | 853 | 11.76 |
| y | 880 | 22.31 |
| y | 894 | 13.02 |
| y | 935 | 23.74 |
| y | 989 | 18.81 |


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


### 工具箱 primary-page implementation notes

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
