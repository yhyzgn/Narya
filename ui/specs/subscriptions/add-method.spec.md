# UI Spec — `ui/subscriptions/add-method.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-method.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add method |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 32.97% | 0.996 |
| 2 | `#ffffff` | 7.74% | 1.0 |
| 3 | `#fdfdfe` | 7.14% | 0.992 |
| 4 | `#f7f7fc` | 6.75% | 0.97 |
| 5 | `#fcfcfe` | 6.73% | 0.989 |
| 6 | `#fafbfe` | 6.26% | 0.984 |
| 7 | `#dde0ec` | 4.6% | 0.879 |
| 8 | `#fbfbfe` | 4.18% | 0.985 |
| 9 | `#fffeff` | 4.08% | 0.997 |
| 10 | `#f9fafd` | 4.02% | 0.98 |
| 11 | `#a1a8ce` | 3.72% | 0.664 |
| 12 | `#f1f4fa` | 3.29% | 0.956 |
| 13 | `#fefeff` | 2.95% | 0.996 |
| 14 | `#fafafe` | 2.81% | 0.982 |
| 15 | `#fdfefe` | 2.31% | 0.995 |
| 16 | `#fefefd` | 0.44% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#dfe7f0` |
| top_center | `#e3e8f0` |
| top_right | `#e7ecf3` |
| center | `#fefefe` |
| bottom_left | `#e8ebf2` |
| bottom_center | `#e4e7ef` |
| bottom_right | `#e1e5ed` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 17.84 |
| x | 54 | 11.97 |
| x | 68 | 7.34 |
| x | 81 | 13.95 |
| x | 95 | 13.15 |
| x | 109 | 10.36 |
| x | 307 | 14.92 |
| x | 402 | 11.41 |
| x | 436 | 8.0 |
| x | 477 | 9.17 |
| x | 573 | 12.16 |
| x | 703 | 7.71 |
| x | 1024 | 9.31 |
| x | 1228 | 34.01 |
| x | 1242 | 31.15 |
| x | 1256 | 10.17 |
| y | 61 | 21.19 |
| y | 81 | 17.37 |
| y | 95 | 23.73 |
| y | 423 | 21.16 |
| y | 443 | 19.28 |
| y | 464 | 16.21 |
| y | 498 | 18.6 |
| y | 682 | 17.59 |
| y | 696 | 17.73 |
| y | 716 | 15.83 |
| y | 846 | 19.41 |
| y | 860 | 17.22 |
| y | 901 | 20.11 |
| y | 942 | 38.84 |
| y | 983 | 17.7 |
| y | 996 | 24.28 |


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
