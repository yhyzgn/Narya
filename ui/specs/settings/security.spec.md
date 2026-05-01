# UI Spec — `ui/settings/security.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/security.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | security |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 35.21% | 0.996 |
| 2 | `#ffffff` | 7.87% | 1.0 |
| 3 | `#fbfbfd` | 7.23% | 0.985 |
| 4 | `#fdfdfe` | 6.63% | 0.992 |
| 5 | `#fcfcfe` | 5.49% | 0.989 |
| 6 | `#f9f9fc` | 5.28% | 0.977 |
| 7 | `#f1f4f8` | 5.27% | 0.955 |
| 8 | `#dde3eb` | 4.97% | 0.887 |
| 9 | `#fbfcfe` | 4.73% | 0.988 |
| 10 | `#fffefe` | 4.27% | 0.997 |
| 11 | `#adb9cd` | 4.22% | 0.721 |
| 12 | `#fafbfd` | 3.41% | 0.984 |
| 13 | `#fffffe` | 1.73% | 1.0 |
| 14 | `#fefeff` | 1.66% | 0.996 |
| 15 | `#fdfefe` | 1.07% | 0.995 |
| 16 | `#fefefd` | 0.95% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#98a6b7` |
| top_center | `#cad1d9` |
| top_right | `#9fabb7` |
| center | `#fefefe` |
| bottom_left | `#9fa5b5` |
| bottom_center | `#c1c7d6` |
| bottom_right | `#9ca4b5` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 30.42 |
| x | 61 | 13.43 |
| x | 81 | 8.15 |
| x | 102 | 11.47 |
| x | 116 | 6.46 |
| x | 307 | 14.19 |
| x | 334 | 9.2 |
| x | 348 | 6.27 |
| x | 361 | 17.52 |
| x | 498 | 11.85 |
| x | 716 | 13.92 |
| x | 730 | 9.66 |
| x | 791 | 7.1 |
| x | 1112 | 21.23 |
| x | 1126 | 9.87 |
| x | 1167 | 7.37 |
| y | 61 | 14.36 |
| y | 75 | 30.06 |
| y | 88 | 23.33 |
| y | 157 | 17.77 |
| y | 170 | 15.46 |
| y | 191 | 15.14 |
| y | 259 | 13.9 |
| y | 279 | 21.53 |
| y | 293 | 14.01 |
| y | 348 | 18.63 |
| y | 546 | 20.59 |
| y | 566 | 17.03 |
| y | 730 | 19.79 |
| y | 907 | 16.32 |
| y | 989 | 16.14 |
| y | 1003 | 15.29 |


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


### Settings child-page notes

- Best candidate family for the Element Plus sample migration.
- Use `ElForm`, `ElSwitch`, `ElSelect`, `ElInputNumber`, `ElTabs`/segmented nav, `ElTooltip`, and `ElMessage` where useful.
- Theme every control to Narya density: compact labels, 32–36px controls, soft border, subtle active blue/green.
- Preserve left settings category navigation and right form-section grouping exactly from the screenshot.

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
