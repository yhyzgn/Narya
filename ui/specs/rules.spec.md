# UI Spec — `ui/rules.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/rules.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/rules` |
| Human title | 规则 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 25.29% | 0.996 |
| 2 | `#ffffff` | 11.55% | 1.0 |
| 3 | `#f8fafd` | 9.66% | 0.98 |
| 4 | `#fcfcfd` | 8.91% | 0.989 |
| 5 | `#fafbfd` | 8.09% | 0.984 |
| 6 | `#fdfdfe` | 5.87% | 0.992 |
| 7 | `#d3d8e2` | 5.4% | 0.846 |
| 8 | `#fffefe` | 5.09% | 0.997 |
| 9 | `#f5f5f9` | 5.05% | 0.962 |
| 10 | `#e7edf5` | 4.51% | 0.927 |
| 11 | `#a1a9d0` | 3.67% | 0.667 |
| 12 | `#fffffe` | 2.39% | 1.0 |
| 13 | `#fefeff` | 2.02% | 0.996 |
| 14 | `#fefdfe` | 1.28% | 0.993 |
| 15 | `#feffff` | 0.7% | 0.999 |
| 16 | `#fefefd` | 0.52% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a5b2c4` |
| top_center | `#d5dae2` |
| top_right | `#abb4be` |
| center | `#feffff` |
| bottom_left | `#a8aebd` |
| bottom_center | `#d3d6e0` |
| bottom_right | `#a9afbd` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 18.6 |
| x | 54 | 16.59 |
| x | 81 | 16.54 |
| x | 95 | 10.61 |
| x | 109 | 11.15 |
| x | 293 | 8.45 |
| x | 327 | 16.95 |
| x | 348 | 28.36 |
| x | 361 | 10.48 |
| x | 409 | 9.95 |
| x | 907 | 10.14 |
| x | 928 | 11.94 |
| x | 948 | 9.45 |
| x | 1010 | 11.19 |
| x | 1085 | 8.46 |
| x | 1160 | 18.51 |
| y | 170 | 23.54 |
| y | 238 | 20.03 |
| y | 252 | 29.5 |
| y | 300 | 30.53 |
| y | 314 | 26.99 |
| y | 430 | 20.36 |
| y | 498 | 18.83 |
| y | 525 | 39.18 |
| y | 559 | 23.08 |
| y | 573 | 18.26 |
| y | 703 | 19.64 |
| y | 853 | 19.92 |
| y | 880 | 18.71 |
| y | 921 | 23.57 |
| y | 983 | 18.16 |
| y | 996 | 19.73 |


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


### 规则 primary-page implementation notes

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
