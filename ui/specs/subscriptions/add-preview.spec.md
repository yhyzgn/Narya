# UI Spec — `ui/subscriptions/add-preview.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-preview.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add preview |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 23.92% | 0.996 |
| 2 | `#fdfdfe` | 11.77% | 0.992 |
| 3 | `#fafbfd` | 10.95% | 0.984 |
| 4 | `#fefdfe` | 8.21% | 0.993 |
| 5 | `#f1f3f7` | 7.91% | 0.952 |
| 6 | `#b2b7d7` | 7.34% | 0.723 |
| 7 | `#fbfcfd` | 6.48% | 0.988 |
| 8 | `#f8fafc` | 5.33% | 0.979 |
| 9 | `#ffffff` | 5.05% | 1.0 |
| 10 | `#fdfdfd` | 3.31% | 0.992 |
| 11 | `#fdfefe` | 2.68% | 0.995 |
| 12 | `#fbfbfc` | 2.46% | 0.985 |
| 13 | `#fcfdfe` | 1.48% | 0.992 |
| 14 | `#fefeff` | 1.44% | 0.996 |
| 15 | `#fefefd` | 0.85% | 0.996 |
| 16 | `#fffefe` | 0.83% | 0.997 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#c8d3e1` |
| top_center | `#e0e4ea` |
| top_right | `#ccd5df` |
| center | `#f9fdfb` |
| bottom_left | `#ccd1df` |
| bottom_center | `#dde0e8` |
| bottom_right | `#c6ccdb` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 17.68 |
| x | 54 | 15.76 |
| x | 81 | 12.45 |
| x | 109 | 11.74 |
| x | 300 | 34.81 |
| x | 320 | 21.77 |
| x | 341 | 19.48 |
| x | 354 | 16.14 |
| x | 375 | 13.06 |
| x | 395 | 19.83 |
| x | 443 | 9.25 |
| x | 757 | 9.76 |
| x | 1194 | 32.42 |
| x | 1208 | 17.66 |
| x | 1447 | 12.48 |
| x | 1467 | 9.5 |
| y | 61 | 24.79 |
| y | 157 | 16.14 |
| y | 238 | 20.82 |
| y | 252 | 33.79 |
| y | 532 | 16.52 |
| y | 546 | 15.2 |
| y | 559 | 15.84 |
| y | 628 | 15.91 |
| y | 641 | 19.88 |
| y | 669 | 16.79 |
| y | 682 | 20.02 |
| y | 750 | 19.34 |
| y | 887 | 23.78 |
| y | 928 | 45.03 |
| y | 976 | 15.87 |
| y | 989 | 31.52 |


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
