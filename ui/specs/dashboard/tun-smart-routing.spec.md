# UI Spec — `ui/dashboard/tun-smart-routing.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/dashboard/tun-smart-routing.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `dashboard detail/state` |
| Human title | tun smart routing |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 25.76% | 0.996 |
| 2 | `#fafbfd` | 10.09% | 0.984 |
| 3 | `#f1f5f6` | 9.52% | 0.958 |
| 4 | `#fdfdfe` | 8.51% | 0.992 |
| 5 | `#fafafd` | 7.88% | 0.981 |
| 6 | `#fcfcfe` | 6.53% | 0.989 |
| 7 | `#bfc9d5` | 6.44% | 0.783 |
| 8 | `#f8fafc` | 5.59% | 0.979 |
| 9 | `#ffffff` | 5.43% | 1.0 |
| 10 | `#fbfbfd` | 4.5% | 0.985 |
| 11 | `#fdfefe` | 2.76% | 0.995 |
| 12 | `#fbfdfd` | 1.93% | 0.99 |
| 13 | `#fffefe` | 1.69% | 0.997 |
| 14 | `#fefdfe` | 1.64% | 0.993 |
| 15 | `#fefeff` | 1.08% | 0.996 |
| 16 | `#fefefd` | 0.64% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a1afc0` |
| top_center | `#ccd3da` |
| top_right | `#aab4bf` |
| center | `#f0f9f4` |
| bottom_left | `#a9afbd` |
| bottom_center | `#d7dae3` |
| bottom_right | `#a9afbf` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 24.52 |
| x | 54 | 15.18 |
| x | 81 | 10.33 |
| x | 102 | 13.99 |
| x | 116 | 15.84 |
| x | 320 | 21.66 |
| x | 341 | 11.65 |
| x | 375 | 15.39 |
| x | 894 | 9.98 |
| x | 948 | 11.7 |
| x | 989 | 8.52 |
| x | 1071 | 8.14 |
| x | 1133 | 18.41 |
| x | 1174 | 8.74 |
| x | 1433 | 10.38 |
| x | 1501 | 7.91 |
| y | 143 | 30.25 |
| y | 157 | 18.22 |
| y | 191 | 12.45 |
| y | 204 | 14.11 |
| y | 232 | 11.36 |
| y | 259 | 12.92 |
| y | 307 | 14.57 |
| y | 334 | 13.1 |
| y | 354 | 12.99 |
| y | 662 | 27.99 |
| y | 675 | 22.68 |
| y | 716 | 12.48 |
| y | 791 | 10.77 |
| y | 894 | 15.31 |
| y | 976 | 24.84 |
| y | 989 | 24.74 |


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
