# UI Spec — `ui/tools/port-check.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/tools/port-check.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `tools detail/state` |
| Human title | port check |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fcfdfe` | 20.01% | 0.992 |
| 2 | `#fdfdfe` | 16.04% | 0.992 |
| 3 | `#f9fafe` | 10.64% | 0.981 |
| 4 | `#fefefe` | 9.08% | 0.996 |
| 5 | `#f4f6fb` | 8.12% | 0.964 |
| 6 | `#fafbfd` | 7.76% | 0.984 |
| 7 | `#b9c2d6` | 7.68% | 0.759 |
| 8 | `#f8fafd` | 6.14% | 0.98 |
| 9 | `#fbfcfe` | 4.85% | 0.988 |
| 10 | `#fcfcfe` | 3.23% | 0.989 |
| 11 | `#f9fbfd` | 1.99% | 0.983 |
| 12 | `#fcfdfd` | 1.87% | 0.991 |
| 13 | `#fdfdff` | 1.25% | 0.993 |
| 14 | `#fdfdfd` | 0.7% | 0.992 |
| 15 | `#fcfdff` | 0.46% | 0.992 |
| 16 | `#fefdfe` | 0.19% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#d9e3f0` |
| top_center | `#e5e9ef` |
| top_right | `#e2eaf2` |
| center | `#fcfcfd` |
| bottom_left | `#dfe5ee` |
| bottom_center | `#e5e8ee` |
| bottom_right | `#dce3ed` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 21.99 |
| x | 54 | 13.46 |
| x | 88 | 10.71 |
| x | 102 | 13.19 |
| x | 116 | 10.27 |
| x | 314 | 13.88 |
| x | 334 | 10.21 |
| x | 354 | 9.44 |
| x | 375 | 7.55 |
| x | 552 | 11.58 |
| x | 1174 | 9.19 |
| x | 1187 | 7.7 |
| x | 1235 | 9.28 |
| x | 1297 | 9.42 |
| x | 1317 | 9.1 |
| x | 1358 | 9.02 |
| y | 34 | 21.09 |
| y | 75 | 22.82 |
| y | 88 | 29.61 |
| y | 136 | 14.97 |
| y | 163 | 18.02 |
| y | 177 | 15.49 |
| y | 348 | 15.96 |
| y | 375 | 26.4 |
| y | 416 | 25.82 |
| y | 477 | 16.16 |
| y | 559 | 19.83 |
| y | 600 | 25.17 |
| y | 614 | 16.84 |
| y | 894 | 19.98 |
| y | 976 | 22.14 |
| y | 989 | 27.22 |


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
