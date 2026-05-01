# UI Spec — `ui/icons/narya-logo-raw-chroma.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/icons/narya-logo-raw-chroma.png` |
| Canvas | 1254×1254px |
| Aspect ratio | 1.0 |
| Alpha / transparency | false |
| Kind | brand/icon asset |
| Route / state | `asset` |
| Human title | narya-logo-raw-chroma.png |
| Recommended implementation strategy | `hand-vector-or-logo-asset` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#07f90c` | 17.39% | 0.708 |
| 2 | `#07f90d` | 15.13% | 0.708 |
| 3 | `#185cf9` | 11.88% | 0.349 |
| 4 | `#07f90b` | 9.31% | 0.707 |
| 5 | `#06f90c` | 8.98% | 0.707 |
| 6 | `#06f90b` | 7.71% | 0.706 |
| 7 | `#3d9dfa` | 7.37% | 0.562 |
| 8 | `#07fa0c` | 5.88% | 0.71 |
| 9 | `#7f65fb` | 4.54% | 0.46 |
| 10 | `#753efc` | 3.8% | 0.343 |
| 11 | `#63d6da` | 3.14% | 0.744 |
| 12 | `#07f90e` | 2.59% | 0.708 |
| 13 | `#05fa0a` | 0.81% | 0.708 |
| 14 | `#04f909` | 0.73% | 0.704 |
| 15 | `#1cfa21` | 0.69% | 0.734 |
| 16 | `#05c292` | 0.05% | 0.59 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#15f114` |
| top_center | `#09f80b` |
| top_right | `#15ed11` |
| center | `#436afa` |
| bottom_left | `#16f115` |
| bottom_center | `#06f909` |
| bottom_right | `#16f314` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 245 | 103.47 |
| x | 261 | 7.86 |
| x | 457 | 12.47 |
| x | 473 | 72.52 |
| x | 785 | 4.66 |
| x | 802 | 72.99 |
| x | 819 | 10.7 |
| x | 1003 | 5.53 |
| x | 1019 | 122.93 |
| y | 206 | 25.5 |
| y | 222 | 7.37 |
| y | 501 | 9.84 |
| y | 529 | 8.57 |
| y | 696 | 7.21 |
| y | 997 | 6.12 |
| y | 1014 | 9.9 |
| y | 1031 | 27.13 |


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
