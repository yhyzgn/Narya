# UI Spec — `ui/settings/network.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/network.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | network |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 19.02% | 0.996 |
| 2 | `#fdfdfe` | 17.12% | 0.992 |
| 3 | `#f9fafd` | 14.73% | 0.98 |
| 4 | `#eff2f7` | 8.06% | 0.948 |
| 5 | `#fbfbfd` | 7.69% | 0.985 |
| 6 | `#fefdfe` | 7.11% | 0.993 |
| 7 | `#b8c1db` | 6.34% | 0.757 |
| 8 | `#ffffff` | 4.9% | 1.0 |
| 9 | `#fafafd` | 3.43% | 0.981 |
| 10 | `#fdfefe` | 2.99% | 0.995 |
| 11 | `#fcfdfd` | 2.44% | 0.991 |
| 12 | `#fdfdfd` | 2.22% | 0.992 |
| 13 | `#fefeff` | 2.15% | 0.996 |
| 14 | `#fffefe` | 1.17% | 0.997 |
| 15 | `#fefefd` | 0.35% | 0.996 |
| 16 | `#fdfdff` | 0.29% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#dbe3ed` |
| top_center | `#c5ccd6` |
| top_right | `#a6b1bc` |
| center | `#fdfdfe` |
| bottom_left | `#eaedf0` |
| bottom_center | `#c2c8d6` |
| bottom_right | `#ccd2dc` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 20.13 |
| x | 54 | 19.23 |
| x | 68 | 7.7 |
| x | 81 | 15.56 |
| x | 109 | 13.5 |
| x | 252 | 6.72 |
| x | 300 | 18.73 |
| x | 314 | 8.38 |
| x | 334 | 11.87 |
| x | 361 | 8.92 |
| x | 491 | 8.88 |
| x | 552 | 6.74 |
| x | 1160 | 9.78 |
| x | 1174 | 7.12 |
| x | 1419 | 7.57 |
| x | 1433 | 8.28 |
| y | 54 | 15.16 |
| y | 75 | 18.19 |
| y | 88 | 22.51 |
| y | 129 | 24.65 |
| y | 150 | 17.14 |
| y | 170 | 20.52 |
| y | 191 | 16.64 |
| y | 273 | 14.48 |
| y | 368 | 17.22 |
| y | 546 | 13.24 |
| y | 573 | 29.95 |
| y | 600 | 29.41 |
| y | 682 | 19.07 |
| y | 894 | 15.02 |
| y | 907 | 13.32 |
| y | 989 | 18.04 |


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
