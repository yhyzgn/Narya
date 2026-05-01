# UI Spec — `ui/subscriptions/add-qr.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-qr.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add qr |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 17.98% | 0.996 |
| 2 | `#e5e7f1` | 15.53% | 0.907 |
| 3 | `#fdfdfe` | 12.56% | 0.992 |
| 4 | `#fafafd` | 9.43% | 0.981 |
| 5 | `#fbfbfd` | 7.61% | 0.985 |
| 6 | `#a7acd0` | 6.74% | 0.681 |
| 7 | `#f6f8fb` | 6.67% | 0.972 |
| 8 | `#fefdfe` | 6.47% | 0.993 |
| 9 | `#ffffff` | 4.11% | 1.0 |
| 10 | `#fbfcfd` | 3.72% | 0.988 |
| 11 | `#fdfdfd` | 2.67% | 0.992 |
| 12 | `#fdfefe` | 2.55% | 0.995 |
| 13 | `#fffefe` | 1.29% | 0.997 |
| 14 | `#fdfcfe` | 1.1% | 0.99 |
| 15 | `#fefeff` | 0.83% | 0.996 |
| 16 | `#fefefd` | 0.75% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#aab7c8` |
| top_center | `#d1d8e1` |
| top_right | `#b4bdc9` |
| center | `#ebecf3` |
| bottom_left | `#adb3c2` |
| bottom_center | `#d5d9e5` |
| bottom_right | `#aeb4c4` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 18.3 |
| x | 54 | 12.71 |
| x | 81 | 12.48 |
| x | 95 | 10.55 |
| x | 259 | 9.42 |
| x | 293 | 8.46 |
| x | 307 | 20.58 |
| x | 505 | 9.75 |
| x | 525 | 24.86 |
| x | 573 | 28.51 |
| x | 655 | 10.02 |
| x | 675 | 15.41 |
| x | 709 | 20.11 |
| x | 928 | 9.45 |
| x | 996 | 16.79 |
| x | 1030 | 7.74 |
| y | 34 | 17.83 |
| y | 88 | 21.69 |
| y | 136 | 21.59 |
| y | 225 | 23.91 |
| y | 245 | 20.04 |
| y | 293 | 32.82 |
| y | 327 | 48.02 |
| y | 375 | 18.72 |
| y | 477 | 30.73 |
| y | 607 | 17.74 |
| y | 634 | 18.5 |
| y | 880 | 20.24 |
| y | 907 | 22.87 |
| y | 948 | 35.53 |
| y | 983 | 19.02 |
| y | 996 | 17.81 |


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
