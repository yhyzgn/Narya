# UI Spec — `ui/nodes.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/nodes.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/nodes` |
| Human title | 节点 |
| Recommended implementation strategy | `element-plus-business-page` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 17.65% | 0.996 |
| 2 | `#fdfdfe` | 11.91% | 0.992 |
| 3 | `#f9fafd` | 10.26% | 0.98 |
| 4 | `#edf1f5` | 8.78% | 0.943 |
| 5 | `#fdfefe` | 8.54% | 0.995 |
| 6 | `#ffffff` | 7.8% | 1.0 |
| 7 | `#b9bcd4` | 7.25% | 0.742 |
| 8 | `#fbfcfd` | 6.9% | 0.988 |
| 9 | `#fafbfd` | 4.96% | 0.984 |
| 10 | `#f6f9fb` | 4.91% | 0.975 |
| 11 | `#fcfdfe` | 3.41% | 0.992 |
| 12 | `#fbfbfc` | 2.54% | 0.985 |
| 13 | `#fffefe` | 1.7% | 0.997 |
| 14 | `#fefdfe` | 1.58% | 0.993 |
| 15 | `#fefeff` | 1.3% | 0.996 |
| 16 | `#fefefd` | 0.51% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#c9d0d9` |
| top_center | `#b7bec9` |
| top_right | `#cdd4d8` |
| center | `#fdfefe` |
| bottom_left | `#cdd2da` |
| bottom_center | `#989eac` |
| bottom_right | `#c5cbd6` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 26.87 |
| x | 61 | 11.23 |
| x | 116 | 13.57 |
| x | 307 | 24.73 |
| x | 327 | 17.92 |
| x | 354 | 11.11 |
| x | 464 | 9.84 |
| x | 607 | 16.73 |
| x | 641 | 12.71 |
| x | 907 | 15.65 |
| x | 935 | 14.6 |
| x | 976 | 9.13 |
| x | 996 | 10.59 |
| x | 1174 | 8.48 |
| x | 1256 | 19.44 |
| x | 1508 | 12.34 |
| y | 109 | 25.21 |
| y | 157 | 26.46 |
| y | 170 | 19.76 |
| y | 191 | 23.47 |
| y | 225 | 24.67 |
| y | 279 | 18.17 |
| y | 293 | 19.27 |
| y | 450 | 19.11 |
| y | 464 | 18.01 |
| y | 737 | 34.91 |
| y | 750 | 23.95 |
| y | 826 | 29.54 |
| y | 839 | 19.8 |
| y | 853 | 21.32 |
| y | 976 | 16.35 |
| y | 989 | 24.8 |


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


### 节点 primary-page implementation notes

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
