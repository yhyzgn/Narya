# UI Spec — `ui/subscriptions/add-hiddify.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-hiddify.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add hiddify |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 16.72% | 0.996 |
| 2 | `#fdfdfe` | 14.82% | 0.992 |
| 3 | `#f9fafd` | 10.43% | 0.98 |
| 4 | `#fefdfe` | 10.17% | 0.993 |
| 5 | `#fafafd` | 9.63% | 0.981 |
| 6 | `#f3f5f9` | 8.39% | 0.96 |
| 7 | `#babfdb` | 7.83% | 0.753 |
| 8 | `#fcfcfd` | 6.9% | 0.989 |
| 9 | `#ffffff` | 4.59% | 1.0 |
| 10 | `#fdfefe` | 3.48% | 0.995 |
| 11 | `#fefeff` | 3.12% | 0.996 |
| 12 | `#fdfdfd` | 1.28% | 0.992 |
| 13 | `#fcfdfe` | 1.23% | 0.992 |
| 14 | `#fffeff` | 0.8% | 0.997 |
| 15 | `#fdfdff` | 0.42% | 0.993 |
| 16 | `#fefefd` | 0.2% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#8997aa` |
| top_center | `#b2bccc` |
| top_right | `#8f99a7` |
| center | `#fefeff` |
| bottom_left | `#8990a1` |
| bottom_center | `#dde0eb` |
| bottom_right | `#858d9e` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 19.0 |
| x | 54 | 13.19 |
| x | 81 | 12.89 |
| x | 293 | 16.95 |
| x | 314 | 7.47 |
| x | 389 | 8.56 |
| x | 505 | 8.91 |
| x | 669 | 8.46 |
| x | 709 | 9.15 |
| x | 737 | 9.67 |
| x | 894 | 15.28 |
| x | 921 | 9.51 |
| x | 1092 | 10.27 |
| x | 1112 | 9.46 |
| x | 1467 | 10.27 |
| x | 1481 | 10.24 |
| y | 34 | 26.15 |
| y | 54 | 30.86 |
| y | 184 | 22.38 |
| y | 197 | 22.02 |
| y | 266 | 22.71 |
| y | 293 | 25.39 |
| y | 361 | 21.63 |
| y | 402 | 18.71 |
| y | 450 | 22.3 |
| y | 791 | 18.96 |
| y | 812 | 18.97 |
| y | 839 | 21.53 |
| y | 853 | 19.62 |
| y | 880 | 27.53 |
| y | 907 | 22.65 |
| y | 942 | 24.55 |


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
