# UI Spec — `ui/subscriptions/add-file.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/subscriptions/add-file.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `subscriptions detail/state` |
| Human title | add file |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 18.66% | 0.996 |
| 2 | `#fdfdfd` | 11.78% | 0.992 |
| 3 | `#fafafd` | 9.02% | 0.981 |
| 4 | `#f9fafc` | 9.02% | 0.98 |
| 5 | `#fefdfd` | 8.67% | 0.993 |
| 6 | `#f2f4f8` | 8.01% | 0.956 |
| 7 | `#fefefd` | 7.24% | 0.996 |
| 8 | `#b8c0de` | 6.96% | 0.755 |
| 9 | `#fafbfd` | 6.36% | 0.984 |
| 10 | `#fbfcfd` | 4.8% | 0.988 |
| 11 | `#ffffff` | 4.39% | 1.0 |
| 12 | `#fdfefe` | 2.98% | 0.995 |
| 13 | `#fffefe` | 0.91% | 0.997 |
| 14 | `#fcfdfd` | 0.64% | 0.991 |
| 15 | `#fefeff` | 0.35% | 0.996 |
| 16 | `#fdfdfc` | 0.2% | 0.992 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#8190a4` |
| top_center | `#b5becc` |
| top_right | `#8f9cac` |
| center | `#fefefd` |
| bottom_left | `#8890a3` |
| bottom_center | `#e0e2ea` |
| bottom_right | `#8790a5` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 14.86 |
| x | 54 | 14.68 |
| x | 68 | 9.64 |
| x | 81 | 11.26 |
| x | 109 | 15.53 |
| x | 252 | 7.64 |
| x | 286 | 7.67 |
| x | 744 | 9.29 |
| x | 1024 | 11.06 |
| x | 1037 | 17.77 |
| x | 1051 | 20.47 |
| x | 1119 | 7.15 |
| x | 1187 | 6.94 |
| x | 1221 | 6.84 |
| x | 1460 | 12.89 |
| x | 1474 | 13.11 |
| y | 34 | 21.61 |
| y | 54 | 21.6 |
| y | 75 | 22.11 |
| y | 88 | 31.2 |
| y | 129 | 15.4 |
| y | 232 | 19.08 |
| y | 334 | 16.3 |
| y | 348 | 22.25 |
| y | 682 | 14.62 |
| y | 703 | 15.08 |
| y | 730 | 21.47 |
| y | 750 | 27.72 |
| y | 771 | 16.17 |
| y | 901 | 34.4 |
| y | 948 | 28.35 |
| y | 989 | 17.48 |


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
