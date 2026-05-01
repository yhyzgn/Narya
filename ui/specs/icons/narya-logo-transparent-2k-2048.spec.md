# UI Spec — `ui/icons/narya-logo-transparent-2k-2048.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/icons/narya-logo-transparent-2k-2048.png` |
| Canvas | 2048×2048px |
| Aspect ratio | 1.0 |
| Alpha / transparency | true |
| Kind | brand/icon asset |
| Route / state | `asset` |
| Human title | narya-logo-transparent-2k-2048.png |
| Recommended implementation strategy | `hand-vector-or-logo-asset` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#ffffff` | 68.12% | 1.0 |
| 2 | `#1764fb` | 3.84% | 0.371 |
| 3 | `#214cf6` | 3.7% | 0.31 |
| 4 | `#995ffb` | 3.36% | 0.465 |
| 5 | `#763cfc` | 3.23% | 0.338 |
| 6 | `#017bfc` | 2.99% | 0.417 |
| 7 | `#18abfc` | 2.62% | 0.571 |
| 8 | `#0395fc` | 2.59% | 0.492 |
| 9 | `#1fbefc` | 2.11% | 0.63 |
| 10 | `#6565fa` | 1.73% | 0.438 |
| 11 | `#80d2fb` | 1.47% | 0.767 |
| 12 | `#eef0fa` | 1.38% | 0.942 |
| 13 | `#138dfc` | 1.16% | 0.483 |
| 14 | `#fcfffd` | 0.84% | 0.997 |
| 15 | `#fffffe` | 0.53% | 1.0 |
| 16 | `#688cfa` | 0.34% | 0.55 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#000000` |
| top_center | `#000000` |
| top_right | `#000000` |
| center | `#436afa` |
| bottom_left | `#000000` |
| bottom_center | `#000000` |
| bottom_right | `#000000` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 400 | 172.42 |
| x | 418 | 21.99 |
| x | 755 | 11.92 |
| x | 773 | 74.85 |
| x | 1292 | 8.11 |
| x | 1310 | 105.72 |
| x | 1328 | 13.99 |
| x | 1347 | 4.94 |
| x | 1647 | 13.51 |
| x | 1665 | 169.39 |
| y | 336 | 33.42 |
| y | 354 | 10.97 |
| y | 373 | 7.11 |
| y | 391 | 5.45 |
| y | 409 | 5.15 |
| y | 427 | 5.32 |
| y | 819 | 10.62 |
| y | 864 | 7.94 |
| y | 1137 | 7.77 |
| y | 1629 | 5.06 |
| y | 1647 | 6.93 |
| y | 1665 | 10.09 |
| y | 1683 | 29.74 |


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


### Icon asset notes

- This is a brand/logo raster reference, not a UI page.
- If a real SVG is needed, hand-vector the logo with gradients; traced paths are acceptable only as a temporary approximation.
- Preserve transparent background where present and never add a white bounding box unless the source has one.

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
