# UI Spec — `ui/splash.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/splash.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `splash-window` |
| Human title | 启动页 |
| Recommended implementation strategy | `high-fidelity-custom` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#ecf2fd` | 9.05% | 0.947 |
| 2 | `#f0f4fd` | 7.95% | 0.956 |
| 3 | `#e3ecfc` | 7.33% | 0.923 |
| 4 | `#e9f0fd` | 7.26% | 0.939 |
| 5 | `#dbe5fb` | 6.96% | 0.896 |
| 6 | `#f7f8fd` | 6.62% | 0.973 |
| 7 | `#dee9fc` | 6.45% | 0.91 |
| 8 | `#e6edfc` | 6.25% | 0.928 |
| 9 | `#c1cdf5` | 5.96% | 0.805 |
| 10 | `#f5f7fd` | 5.88% | 0.969 |
| 11 | `#f3f5fd` | 5.65% | 0.961 |
| 12 | `#f4f6fd` | 5.55% | 0.965 |
| 13 | `#eff2fd` | 5.17% | 0.95 |
| 14 | `#d3e1fb` | 4.91% | 0.878 |
| 15 | `#fafafd` | 4.76% | 0.981 |
| 16 | `#8b9ce8` | 4.25% | 0.619 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#fbf9fc` |
| top_center | `#eaf1fd` |
| top_right | `#c5c2f8` |
| center | `#f8fafd` |
| bottom_left | `#b1a1f5` |
| bottom_center | `#e5eefc` |
| bottom_right | `#e1ebf9` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 395 | 20.44 |
| x | 505 | 7.06 |
| x | 525 | 9.47 |
| x | 552 | 7.17 |
| x | 614 | 8.2 |
| x | 634 | 7.21 |
| x | 669 | 16.4 |
| x | 696 | 27.62 |
| x | 709 | 20.0 |
| x | 723 | 9.12 |
| x | 737 | 11.56 |
| x | 757 | 10.99 |
| x | 798 | 21.01 |
| x | 839 | 16.51 |
| x | 866 | 7.85 |
| x | 1140 | 24.5 |
| y | 361 | 14.55 |
| y | 395 | 16.22 |
| y | 484 | 10.85 |
| y | 525 | 21.27 |
| y | 546 | 21.76 |
| y | 573 | 42.83 |
| y | 593 | 47.25 |
| y | 614 | 9.58 |
| y | 634 | 12.67 |
| y | 682 | 13.92 |
| y | 737 | 13.57 |
| y | 785 | 14.61 |
| y | 805 | 14.45 |
| y | 860 | 9.4 |
| y | 873 | 10.68 |
| y | 901 | 19.29 |


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


### Splash-specific implementation notes

- Treat Splash as custom high-fidelity composition.
- Preserve fixed window geometry and centered logo/loading composition.
- Avoid using generic component library defaults for progress, text spacing, or glass surfaces unless theme overrides match the PNG exactly.
- If animated, the final still at representative progress states must remain spatially aligned to this PNG.

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
