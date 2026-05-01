# UI Spec — `ui/config.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/config.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/profiles` |
| Human title | 配置 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 18.15% | 0.992 |
| 2 | `#fefefe` | 11.29% | 0.996 |
| 3 | `#fafbfd` | 10.59% | 0.984 |
| 4 | `#f9fafd` | 10.35% | 0.98 |
| 5 | `#f1f4f8` | 8.68% | 0.955 |
| 6 | `#c0c8dd` | 7.69% | 0.784 |
| 7 | `#fdfdfd` | 6.57% | 0.992 |
| 8 | `#fcfcfd` | 6.23% | 0.989 |
| 9 | `#fefdfe` | 6.21% | 0.993 |
| 10 | `#f8f8fb` | 5.79% | 0.973 |
| 11 | `#ffffff` | 3.51% | 1.0 |
| 12 | `#fdfefe` | 1.82% | 0.995 |
| 13 | `#fcfdfe` | 1.36% | 0.992 |
| 14 | `#fffefe` | 0.89% | 0.997 |
| 15 | `#fefefd` | 0.71% | 0.996 |
| 16 | `#fdfdff` | 0.15% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#adb5c2` |
| top_center | `#c3cad4` |
| top_right | `#abb3be` |
| center | `#f8f9fc` |
| bottom_left | `#b5bac6` |
| bottom_center | `#c3c8d4` |
| bottom_right | `#b6bbc6` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 18.96 |
| x | 54 | 14.03 |
| x | 88 | 8.47 |
| x | 102 | 15.08 |
| x | 259 | 15.61 |
| x | 293 | 9.88 |
| x | 307 | 23.34 |
| x | 320 | 8.57 |
| x | 334 | 19.22 |
| x | 348 | 9.78 |
| x | 628 | 15.8 |
| x | 764 | 7.99 |
| x | 955 | 7.3 |
| x | 969 | 7.81 |
| x | 1167 | 15.77 |
| x | 1181 | 15.13 |
| y | 88 | 15.95 |
| y | 116 | 22.43 |
| y | 136 | 18.39 |
| y | 177 | 22.95 |
| y | 204 | 24.21 |
| y | 252 | 17.62 |
| y | 266 | 22.89 |
| y | 286 | 15.74 |
| y | 300 | 17.2 |
| y | 395 | 16.41 |
| y | 491 | 15.24 |
| y | 819 | 74.88 |
| y | 832 | 32.74 |
| y | 873 | 17.87 |
| y | 976 | 17.18 |
| y | 989 | 22.2 |


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


### 配置 primary-page implementation notes

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
