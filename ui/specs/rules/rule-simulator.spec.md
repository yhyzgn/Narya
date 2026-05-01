# UI Spec — `ui/rules/rule-simulator.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/rules/rule-simulator.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `rules detail/state` |
| Human title | rule simulator |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfd` | 16.14% | 0.992 |
| 2 | `#fcfcfd` | 10.41% | 0.989 |
| 3 | `#f8f9fc` | 9.33% | 0.976 |
| 4 | `#fefefe` | 9.12% | 0.996 |
| 5 | `#fdfdfe` | 8.81% | 0.992 |
| 6 | `#f1f3f8` | 8.13% | 0.953 |
| 7 | `#fbfbfd` | 7.25% | 0.985 |
| 8 | `#c0c5da` | 6.69% | 0.774 |
| 9 | `#f9fafc` | 6.62% | 0.98 |
| 10 | `#fdfcfd` | 5.3% | 0.989 |
| 11 | `#fafafc` | 4.88% | 0.981 |
| 12 | `#fefdfe` | 2.2% | 0.993 |
| 13 | `#fafafb` | 1.72% | 0.981 |
| 14 | `#fcfdfd` | 1.7% | 0.991 |
| 15 | `#fbfcfd` | 1.6% | 0.988 |
| 16 | `#fdfdfc` | 0.1% | 0.992 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#929eb0` |
| top_center | `#b4bbc5` |
| top_right | `#98a2af` |
| center | `#f3f6fe` |
| bottom_left | `#9ca2b3` |
| bottom_center | `#9fa5b5` |
| bottom_right | `#99a0b3` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 21.8 |
| x | 54 | 12.36 |
| x | 68 | 7.15 |
| x | 88 | 10.63 |
| x | 102 | 13.99 |
| x | 266 | 6.88 |
| x | 307 | 18.5 |
| x | 375 | 7.44 |
| x | 491 | 8.09 |
| x | 552 | 7.55 |
| x | 614 | 8.77 |
| x | 648 | 7.06 |
| x | 853 | 7.34 |
| x | 1160 | 16.91 |
| x | 1181 | 14.23 |
| x | 1474 | 8.32 |
| y | 54 | 12.91 |
| y | 75 | 24.19 |
| y | 88 | 23.82 |
| y | 129 | 12.56 |
| y | 170 | 21.92 |
| y | 184 | 17.83 |
| y | 197 | 13.37 |
| y | 266 | 14.48 |
| y | 300 | 13.09 |
| y | 320 | 13.25 |
| y | 354 | 18.31 |
| y | 614 | 30.27 |
| y | 648 | 35.72 |
| y | 682 | 12.7 |
| y | 969 | 13.45 |
| y | 983 | 18.36 |


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


### rules secondary-state notes

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
