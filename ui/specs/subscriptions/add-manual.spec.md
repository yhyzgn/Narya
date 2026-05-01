# UI Spec — `ui/subscriptions/add-manual.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-manual.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add manual |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfefe` | 22.79% | 0.995 |
| 2 | `#f9fbfd` | 12.24% | 0.983 |
| 3 | `#fbfcfe` | 11.06% | 0.988 |
| 4 | `#fdfdfe` | 10.43% | 0.992 |
| 5 | `#fefefe` | 9.6% | 0.996 |
| 6 | `#f2f4f8` | 7.76% | 0.956 |
| 7 | `#b0bada` | 6.77% | 0.73 |
| 8 | `#fcfdfe` | 5.45% | 0.992 |
| 9 | `#ffffff` | 5.19% | 1.0 |
| 10 | `#fbfcfd` | 4.1% | 0.988 |
| 11 | `#fdfeff` | 2.59% | 0.996 |
| 12 | `#fdfdfd` | 0.6% | 0.992 |
| 13 | `#fdfdff` | 0.51% | 0.993 |
| 14 | `#fefdfe` | 0.48% | 0.993 |
| 15 | `#fdfefd` | 0.36% | 0.995 |
| 16 | `#fdffff` | 0.06% | 0.998 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#b5c0ce` |
| top_center | `#9aa5b6` |
| top_right | `#778496` |
| center | `#f8f8f9` |
| bottom_left | `#d8dce6` |
| bottom_center | `#d3d8e2` |
| bottom_right | `#c9cfda` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 17.12 |
| x | 47 | 7.99 |
| x | 61 | 9.81 |
| x | 75 | 9.61 |
| x | 95 | 7.46 |
| x | 109 | 13.09 |
| x | 286 | 12.06 |
| x | 307 | 26.42 |
| x | 327 | 22.41 |
| x | 464 | 7.09 |
| x | 566 | 12.18 |
| x | 614 | 8.63 |
| x | 641 | 6.61 |
| x | 1160 | 15.22 |
| x | 1174 | 11.12 |
| x | 1495 | 7.12 |
| y | 27 | 16.25 |
| y | 54 | 31.02 |
| y | 68 | 17.82 |
| y | 81 | 16.77 |
| y | 184 | 16.71 |
| y | 197 | 13.53 |
| y | 382 | 22.26 |
| y | 498 | 23.58 |
| y | 532 | 22.78 |
| y | 669 | 14.73 |
| y | 682 | 15.74 |
| y | 703 | 14.55 |
| y | 907 | 33.87 |
| y | 948 | 24.6 |
| y | 983 | 21.76 |
| y | 996 | 20.89 |


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
