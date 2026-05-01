# UI Spec — `ui/splash-background-2k.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/splash-background-2k.png` |
| Canvas | 1672×941px |
| Aspect ratio | 1.7768 |
| Alpha / transparency | false |
| Kind | background asset |
| Route / state | `splash asset` |
| Human title | Splash background |
| Recommended implementation strategy | `raster-reference-or-css-gradient` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#f2f4fc` | 8.52% | 0.957 |
| 2 | `#f5f7fc` | 8.34% | 0.968 |
| 3 | `#f8f8fd` | 7.86% | 0.974 |
| 4 | `#abb9fc` | 7.4% | 0.733 |
| 5 | `#e4ecfc` | 6.98% | 0.923 |
| 6 | `#f9f9fd` | 6.68% | 0.978 |
| 7 | `#f0f3fc` | 6.51% | 0.953 |
| 8 | `#d9e5fc` | 6.37% | 0.895 |
| 9 | `#edf1fc` | 6.13% | 0.945 |
| 10 | `#e2eafc` | 5.67% | 0.916 |
| 11 | `#e9effc` | 5.38% | 0.936 |
| 12 | `#fbfafd` | 5.27% | 0.982 |
| 13 | `#ebf0fc` | 5.07% | 0.94 |
| 14 | `#e0e8fc` | 4.85% | 0.909 |
| 15 | `#e7edfc` | 4.67% | 0.929 |
| 16 | `#e7eefc` | 4.29% | 0.931 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#fafafc` |
| top_center | `#e8f0fd` |
| top_right | `#9db2f8` |
| center | `#f9f9fd` |
| bottom_left | `#9a8ff5` |
| bottom_center | `#e1eafb` |
| bottom_right | `#e9effa` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| — | — | — |
| — | — | — |


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


### Asset-specific notes

- Use this PNG as the precise visual source for dimensions, color, and composition.
- If implemented in code, first classify whether it is a full page, modal state, panel state, or standalone asset.

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
