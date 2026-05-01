# UI Spec — `ui/subscriptions/add-success.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-success.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add success |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 17.77% | 0.996 |
| 2 | `#fdfefe` | 16.37% | 0.995 |
| 3 | `#fafbfd` | 14.42% | 0.984 |
| 4 | `#fdfdfe` | 9.82% | 0.992 |
| 5 | `#f3f6f9` | 8.85% | 0.963 |
| 6 | `#fbfcfe` | 7.77% | 0.988 |
| 7 | `#b5c2d6` | 7.09% | 0.756 |
| 8 | `#ffffff` | 7.02% | 1.0 |
| 9 | `#fcfdfe` | 3.06% | 0.992 |
| 10 | `#fcfcfd` | 2.66% | 0.989 |
| 11 | `#fefeff` | 2.32% | 0.996 |
| 12 | `#fdfeff` | 1.09% | 0.996 |
| 13 | `#fffefe` | 0.94% | 0.997 |
| 14 | `#fdfefd` | 0.46% | 0.995 |
| 15 | `#fefefd` | 0.27% | 0.996 |
| 16 | `#fdfffe` | 0.08% | 0.998 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#8595a7` |
| top_center | `#75879b` |
| top_right | `#2f3d4d` |
| center | `#fdfefe` |
| bottom_left | `#9198a9` |
| bottom_center | `#dadde5` |
| bottom_right | `#808b9e` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 18.45 |
| x | 54 | 8.61 |
| x | 88 | 10.09 |
| x | 102 | 11.04 |
| x | 293 | 7.4 |
| x | 307 | 7.76 |
| x | 354 | 11.04 |
| x | 416 | 7.86 |
| x | 491 | 8.23 |
| x | 525 | 16.81 |
| x | 546 | 12.02 |
| x | 942 | 8.03 |
| x | 1160 | 23.74 |
| x | 1174 | 17.12 |
| x | 1194 | 8.71 |
| x | 1215 | 7.94 |
| y | 61 | 25.69 |
| y | 95 | 16.44 |
| y | 163 | 16.91 |
| y | 293 | 21.14 |
| y | 307 | 23.78 |
| y | 402 | 19.84 |
| y | 416 | 29.92 |
| y | 430 | 15.79 |
| y | 559 | 27.26 |
| y | 573 | 24.9 |
| y | 593 | 25.43 |
| y | 873 | 19.02 |
| y | 907 | 25.63 |
| y | 948 | 20.95 |
| y | 983 | 21.64 |
| y | 996 | 17.09 |


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


### Subscription import flow notes

- Treat as a multi-step/import-state family.
- Use Element Plus Dialog/Drawer/Form/Input/Upload/Steps only if their visual chrome is fully themed.
- Preserve modal/card size, centered alignment, success/preview states, QR/upload/manual/url method hierarchy.

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
