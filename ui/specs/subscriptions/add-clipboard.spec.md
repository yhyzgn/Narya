# UI Spec — `ui/subscriptions/add-clipboard.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-clipboard.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add clipboard |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 18.7% | 0.992 |
| 2 | `#fefefe` | 15.77% | 0.996 |
| 3 | `#fafafe` | 13.06% | 0.982 |
| 4 | `#fcfcfd` | 8.49% | 0.989 |
| 5 | `#f9fafd` | 8.24% | 0.98 |
| 6 | `#f1f3f7` | 8.08% | 0.952 |
| 7 | `#b2bad8` | 7.63% | 0.731 |
| 8 | `#fefdfe` | 6.08% | 0.993 |
| 9 | `#ffffff` | 4.55% | 1.0 |
| 10 | `#fdfefe` | 2.44% | 0.995 |
| 11 | `#fefeff` | 2.04% | 0.996 |
| 12 | `#fdfdfd` | 1.63% | 0.992 |
| 13 | `#fcfdfe` | 1.56% | 0.992 |
| 14 | `#fffeff` | 0.78% | 0.997 |
| 15 | `#fdfdff` | 0.67% | 0.993 |
| 16 | `#fefefd` | 0.29% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#9daabb` |
| top_center | `#b5bccb` |
| top_right | `#a1abb7` |
| center | `#fdfdfe` |
| bottom_left | `#afb5c2` |
| bottom_center | `#d0d3dc` |
| bottom_right | `#a6acbc` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 18.14 |
| x | 54 | 12.78 |
| x | 68 | 9.5 |
| x | 81 | 12.56 |
| x | 95 | 8.51 |
| x | 293 | 11.74 |
| x | 307 | 21.39 |
| x | 320 | 9.94 |
| x | 375 | 8.62 |
| x | 628 | 8.21 |
| x | 785 | 8.1 |
| x | 839 | 13.0 |
| x | 996 | 10.16 |
| x | 1215 | 8.91 |
| x | 1440 | 8.08 |
| x | 1460 | 9.76 |
| y | 34 | 21.29 |
| y | 54 | 28.88 |
| y | 88 | 24.09 |
| y | 191 | 26.18 |
| y | 204 | 24.83 |
| y | 238 | 23.42 |
| y | 252 | 20.98 |
| y | 273 | 23.04 |
| y | 341 | 23.05 |
| y | 464 | 21.37 |
| y | 689 | 22.73 |
| y | 716 | 21.82 |
| y | 832 | 21.65 |
| y | 901 | 33.93 |
| y | 948 | 32.13 |
| y | 983 | 20.93 |


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
