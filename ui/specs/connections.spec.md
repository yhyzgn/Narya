# UI Spec — `ui/connections.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/connections.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/connections` |
| Human title | 连接 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 25.53% | 0.996 |
| 2 | `#fdfdfe` | 10.33% | 0.992 |
| 3 | `#f0f3f7` | 10.08% | 0.952 |
| 4 | `#f9fafd` | 9.16% | 0.98 |
| 5 | `#fbfcfd` | 8.52% | 0.988 |
| 6 | `#c6ccda` | 7.27% | 0.799 |
| 7 | `#ffffff` | 6.47% | 1.0 |
| 8 | `#f6f9fb` | 5.26% | 0.975 |
| 9 | `#fafafc` | 3.8% | 0.981 |
| 10 | `#fafbfd` | 3.67% | 0.984 |
| 11 | `#fefdfe` | 2.8% | 0.993 |
| 12 | `#fdfefe` | 2.56% | 0.995 |
| 13 | `#fffefe` | 1.36% | 0.997 |
| 14 | `#fefefd` | 1.17% | 0.996 |
| 15 | `#fbfdfd` | 1.09% | 0.99 |
| 16 | `#fefeff` | 0.94% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#bcc8d6` |
| top_center | `#c9cdd6` |
| top_right | `#c5cdd5` |
| center | `#fefefe` |
| bottom_left | `#c0c6d4` |
| bottom_center | `#bbbfcc` |
| bottom_right | `#bdc3d1` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 21.74 |
| x | 54 | 12.09 |
| x | 88 | 8.98 |
| x | 102 | 11.69 |
| x | 259 | 9.19 |
| x | 300 | 31.63 |
| x | 327 | 13.67 |
| x | 341 | 9.11 |
| x | 354 | 9.95 |
| x | 566 | 19.65 |
| x | 587 | 21.23 |
| x | 846 | 9.37 |
| x | 1024 | 11.83 |
| x | 1037 | 8.92 |
| x | 1085 | 9.93 |
| x | 1174 | 12.1 |
| y | 68 | 12.93 |
| y | 129 | 12.08 |
| y | 170 | 15.68 |
| y | 184 | 11.84 |
| y | 238 | 23.52 |
| y | 252 | 13.22 |
| y | 327 | 13.63 |
| y | 368 | 13.32 |
| y | 477 | 29.49 |
| y | 491 | 15.84 |
| y | 723 | 16.17 |
| y | 737 | 14.49 |
| y | 791 | 14.63 |
| y | 887 | 12.0 |
| y | 976 | 15.97 |
| y | 989 | 26.87 |


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


### 连接 primary-page implementation notes

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
