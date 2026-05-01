# UI Spec — `ui/icons/narya-logo-transparent-1920x1080.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/icons/narya-logo-transparent-1920x1080.png` |
| Canvas | 1920×1080px |
| Aspect ratio | 1.7778 |
| Alpha / transparency | true |
| Kind | brand/icon asset |
| Route / state | `asset` |
| Human title | narya-logo-transparent-1920x1080.png |
| Recommended implementation strategy | `hand-vector-or-logo-asset` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#ffffff` | 82.08% | 1.0 |
| 2 | `#214cf6` | 2.08% | 0.31 |
| 3 | `#0867fc` | 1.99% | 0.367 |
| 4 | `#0289fc` | 1.91% | 0.457 |
| 5 | `#995ffb` | 1.86% | 0.465 |
| 6 | `#763cfc` | 1.84% | 0.338 |
| 7 | `#19adfc` | 1.8% | 0.577 |
| 8 | `#2563fb` | 1.2% | 0.38 |
| 9 | `#22c0fc` | 1.12% | 0.638 |
| 10 | `#f0f4fb` | 0.95% | 0.956 |
| 11 | `#148efc` | 0.78% | 0.486 |
| 12 | `#6766fa` | 0.77% | 0.443 |
| 13 | `#89d1fb` | 0.73% | 0.771 |
| 14 | `#fdfffd` | 0.39% | 0.998 |
| 15 | `#6b8ffa` | 0.33% | 0.561 |
| 16 | `#fffffe` | 0.18% | 1.0 |

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
| x | 631 | 164.24 |
| x | 657 | 7.43 |
| x | 810 | 12.06 |
| x | 827 | 80.21 |
| x | 1109 | 77.9 |
| x | 1126 | 15.07 |
| x | 1280 | 6.71 |
| x | 1297 | 150.63 |
| y | 179 | 25.36 |
| y | 196 | 7.16 |
| y | 213 | 4.96 |
| y | 230 | 4.59 |
| y | 435 | 6.61 |
| y | 597 | 6.09 |
| y | 870 | 7.44 |
| y | 887 | 22.8 |


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
