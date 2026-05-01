# UI Spec — `ui/connections/connection-detail.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/connections/connection-detail.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `connections detail/state` |
| Human title | connection detail |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 31.19% | 0.996 |
| 2 | `#fbfbfe` | 11.32% | 0.985 |
| 3 | `#fdfdfe` | 8.99% | 0.992 |
| 4 | `#fafafd` | 7.83% | 0.981 |
| 5 | `#c7cad6` | 6.24% | 0.793 |
| 6 | `#eaeff3` | 6.11% | 0.934 |
| 7 | `#ffffff` | 5.74% | 1.0 |
| 8 | `#fcfcfd` | 5.48% | 0.989 |
| 9 | `#f7f7fa` | 4.55% | 0.969 |
| 10 | `#fefdfe` | 3.17% | 0.993 |
| 11 | `#fdfefe` | 2.47% | 0.995 |
| 12 | `#fcfcfe` | 1.79% | 0.989 |
| 13 | `#fefeff` | 1.45% | 0.996 |
| 14 | `#fffefe` | 1.4% | 0.997 |
| 15 | `#fcfdfd` | 1.28% | 0.991 |
| 16 | `#fefefd` | 0.99% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#cad5e4` |
| top_center | `#d7dce5` |
| top_right | `#cdd5df` |
| center | `#fdfdfe` |
| bottom_left | `#ced5e2` |
| bottom_center | `#e2e5ee` |
| bottom_right | `#cacfde` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 23.22 |
| x | 54 | 11.16 |
| x | 75 | 9.51 |
| x | 102 | 9.45 |
| x | 245 | 7.52 |
| x | 279 | 21.87 |
| x | 300 | 5.54 |
| x | 361 | 5.93 |
| x | 532 | 5.6 |
| x | 750 | 6.34 |
| x | 907 | 6.47 |
| x | 921 | 5.49 |
| x | 955 | 6.22 |
| x | 996 | 9.94 |
| x | 1030 | 6.27 |
| x | 1187 | 8.08 |
| y | 34 | 16.45 |
| y | 54 | 14.79 |
| y | 81 | 15.01 |
| y | 184 | 30.12 |
| y | 197 | 19.32 |
| y | 354 | 25.42 |
| y | 409 | 16.33 |
| y | 491 | 23.09 |
| y | 512 | 19.48 |
| y | 614 | 18.87 |
| y | 716 | 19.27 |
| y | 853 | 14.9 |
| y | 866 | 16.7 |
| y | 914 | 14.22 |
| y | 928 | 14.71 |
| y | 989 | 16.87 |


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


### connections secondary-state notes

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
