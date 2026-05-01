# UI Spec — `ui/dashboard.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/dashboard.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | primary page |
| Route / state | `/dashboard` |
| Human title | 仪表盘 |
| Recommended implementation strategy | `high-fidelity-custom` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#f9fafc` | 19.46% | 0.98 |
| 2 | `#fefefe` | 13.21% | 0.996 |
| 3 | `#fefdfd` | 12.75% | 0.993 |
| 4 | `#fafbfd` | 11.21% | 0.984 |
| 5 | `#fdfdfd` | 11.21% | 0.992 |
| 6 | `#f4f7f9` | 9.44% | 0.967 |
| 7 | `#c2cbd6` | 7.18% | 0.792 |
| 8 | `#fefefd` | 3.64% | 0.996 |
| 9 | `#ffffff` | 3.01% | 1.0 |
| 10 | `#fdfefe` | 2.97% | 0.995 |
| 11 | `#fafcfc` | 2.78% | 0.987 |
| 12 | `#fdfcfd` | 2.16% | 0.989 |
| 13 | `#fffefe` | 0.66% | 0.997 |
| 14 | `#fefeff` | 0.25% | 0.996 |
| 15 | `#fefcfd` | 0.04% | 0.99 |
| 16 | `#fefdfc` | 0.03% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#e1e6ec` |
| top_center | `#96a3b4` |
| top_right | `#d8dde2` |
| center | `#eceef6` |
| bottom_left | `#dbdfe4` |
| bottom_center | `#96a1b6` |
| bottom_right | `#d4d9e2` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 26.64 |
| x | 61 | 18.53 |
| x | 88 | 11.89 |
| x | 109 | 10.74 |
| x | 122 | 16.05 |
| x | 273 | 10.91 |
| x | 334 | 20.56 |
| x | 354 | 12.02 |
| x | 389 | 11.38 |
| x | 791 | 8.34 |
| x | 839 | 13.91 |
| x | 853 | 10.45 |
| x | 907 | 8.1 |
| x | 962 | 11.77 |
| x | 1174 | 13.91 |
| x | 1283 | 9.99 |
| y | 61 | 12.47 |
| y | 157 | 24.56 |
| y | 204 | 32.9 |
| y | 252 | 13.37 |
| y | 266 | 14.88 |
| y | 286 | 13.32 |
| y | 327 | 17.18 |
| y | 375 | 14.44 |
| y | 423 | 12.14 |
| y | 546 | 13.11 |
| y | 559 | 20.16 |
| y | 662 | 22.38 |
| y | 675 | 22.98 |
| y | 730 | 23.2 |
| y | 976 | 16.28 |
| y | 989 | 18.15 |


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


### Dashboard-specific implementation notes

- Treat Dashboard as a custom page, not an Element Plus page.
- Canonical reference viewport is 1536×1024.
- Reference left sidebar is about 270px wide with logo/title/version at top, nav items below, connection status card near bottom, and three action icons at the very bottom.
- Top action icons in the design are compact and sit in the upper-right content header area. In the current Tauri app native window controls are used; do not draw extra min/max/close inside the page.
- Main cards: two top proxy mode cards, quick connect list, network overview, traffic usage, protocol stats, activity log.
- Toggle switches should feel compact relative to card titles; avoid visually heavy switches.
- Metric numerals should be prominent but not oversized; avoid wasting vertical space.
- Chart legends should sit in header whitespace where possible rather than stealing graph plotting area.
- Preserve the pale blue/green translucent background, soft borders, and compact desktop density.

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
