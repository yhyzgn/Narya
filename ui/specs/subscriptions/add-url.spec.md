# UI Spec — `ui/subscriptions/add-url.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-url.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add url |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 33.74% | 0.996 |
| 2 | `#fafbfd` | 8.75% | 0.984 |
| 3 | `#fdfdfe` | 7.89% | 0.992 |
| 4 | `#f0f3f7` | 6.94% | 0.952 |
| 5 | `#b7c0d9` | 6.45% | 0.753 |
| 6 | `#ffffff` | 5.44% | 1.0 |
| 7 | `#fbfcfd` | 5.15% | 0.988 |
| 8 | `#f7fbfb` | 5.13% | 0.981 |
| 9 | `#fafafc` | 4.79% | 0.981 |
| 10 | `#fbfbfd` | 4.48% | 0.985 |
| 11 | `#fefdfe` | 4.34% | 0.993 |
| 12 | `#fdfefe` | 1.79% | 0.995 |
| 13 | `#fffefe` | 1.75% | 0.997 |
| 14 | `#fefefd` | 1.53% | 0.996 |
| 15 | `#fafdfc` | 0.94% | 0.989 |
| 16 | `#fefeff` | 0.91% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a4b1c1` |
| top_center | `#dce1e6` |
| top_right | `#aab5c2` |
| center | `#fefefd` |
| bottom_left | `#a6aebe` |
| bottom_center | `#d9dde5` |
| bottom_right | `#a5abbb` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 24.42 |
| x | 61 | 9.18 |
| x | 81 | 7.49 |
| x | 95 | 10.71 |
| x | 116 | 10.46 |
| x | 300 | 12.99 |
| x | 361 | 6.2 |
| x | 389 | 6.45 |
| x | 498 | 5.65 |
| x | 518 | 7.48 |
| x | 928 | 5.46 |
| x | 1017 | 15.28 |
| x | 1037 | 11.26 |
| x | 1051 | 8.07 |
| x | 1392 | 8.1 |
| x | 1460 | 5.84 |
| y | 34 | 23.52 |
| y | 61 | 16.9 |
| y | 75 | 20.99 |
| y | 88 | 25.62 |
| y | 307 | 15.49 |
| y | 320 | 18.66 |
| y | 532 | 14.94 |
| y | 573 | 15.22 |
| y | 634 | 13.97 |
| y | 709 | 21.43 |
| y | 757 | 25.92 |
| y | 894 | 25.38 |
| y | 914 | 22.31 |
| y | 928 | 15.23 |
| y | 983 | 18.59 |
| y | 996 | 14.97 |


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
