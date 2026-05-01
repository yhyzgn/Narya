# UI Spec — `ui/subscriptions.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/subscriptions` |
| Human title | 订阅 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 15.63% | 0.992 |
| 2 | `#f8f9fd` | 14.25% | 0.977 |
| 3 | `#fcfdfd` | 12.05% | 0.991 |
| 4 | `#f1f3f8` | 9.88% | 0.953 |
| 5 | `#fafafd` | 9.64% | 0.981 |
| 6 | `#fefefe` | 9.49% | 0.996 |
| 7 | `#fdfdfd` | 8.0% | 0.992 |
| 8 | `#bcc4dc` | 7.56% | 0.769 |
| 9 | `#fcfcfd` | 5.87% | 0.989 |
| 10 | `#fafafc` | 4.43% | 0.981 |
| 11 | `#fdfcfd` | 0.92% | 0.989 |
| 12 | `#fbfdfe` | 0.89% | 0.991 |
| 13 | `#fefdfe` | 0.65% | 0.993 |
| 14 | `#fdfdff` | 0.53% | 0.993 |
| 15 | `#fcfefe` | 0.16% | 0.994 |
| 16 | `#fcfdfc` | 0.04% | 0.991 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#f0f4f8` |
| top_center | `#bcc2cd` |
| top_right | `#e1e8ed` |
| center | `#fcfdfd` |
| bottom_left | `#e9edf1` |
| bottom_center | `#bbc2cf` |
| bottom_right | `#e1e7ef` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 24.61 |
| x | 54 | 16.44 |
| x | 88 | 9.45 |
| x | 102 | 13.02 |
| x | 266 | 7.67 |
| x | 314 | 22.55 |
| x | 341 | 20.94 |
| x | 361 | 11.14 |
| x | 389 | 8.17 |
| x | 443 | 8.18 |
| x | 764 | 8.78 |
| x | 1146 | 15.77 |
| x | 1194 | 7.9 |
| x | 1454 | 8.31 |
| x | 1474 | 11.77 |
| x | 1508 | 10.6 |
| y | 95 | 14.23 |
| y | 116 | 14.53 |
| y | 136 | 13.24 |
| y | 184 | 19.85 |
| y | 204 | 13.77 |
| y | 273 | 13.5 |
| y | 375 | 13.13 |
| y | 389 | 14.6 |
| y | 552 | 16.41 |
| y | 621 | 14.24 |
| y | 805 | 16.66 |
| y | 819 | 13.58 |
| y | 866 | 12.41 |
| y | 880 | 12.54 |
| y | 976 | 11.99 |
| y | 989 | 24.2 |


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


### 订阅 primary-page implementation notes

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
