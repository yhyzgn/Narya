# UI Spec — `ui/settings/tun.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/tun.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | tun |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 23.27% | 0.996 |
| 2 | `#fdfdfe` | 10.33% | 0.992 |
| 3 | `#f9fafd` | 9.95% | 0.98 |
| 4 | `#fafbfd` | 8.21% | 0.984 |
| 5 | `#fbfcfd` | 7.55% | 0.988 |
| 6 | `#eaf0f5` | 7.31% | 0.938 |
| 7 | `#bfc9d8` | 7.22% | 0.784 |
| 8 | `#ffffff` | 6.79% | 1.0 |
| 9 | `#fefdfe` | 4.2% | 0.993 |
| 10 | `#fdfdfd` | 4.0% | 0.992 |
| 11 | `#f6f7fa` | 3.94% | 0.969 |
| 12 | `#fdfefe` | 2.31% | 0.995 |
| 13 | `#fffefe` | 1.37% | 0.997 |
| 14 | `#fcfdfd` | 1.31% | 0.991 |
| 15 | `#fefeff` | 1.19% | 0.996 |
| 16 | `#fefefd` | 1.05% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#c2cfdf` |
| top_center | `#aab3c2` |
| top_right | `#cad2d8` |
| center | `#fefefe` |
| bottom_left | `#c7ced9` |
| bottom_center | `#c8ccd6` |
| bottom_right | `#b9c2ce` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 20.24 |
| x | 54 | 20.25 |
| x | 88 | 9.65 |
| x | 102 | 12.01 |
| x | 300 | 19.95 |
| x | 314 | 9.98 |
| x | 348 | 10.49 |
| x | 361 | 15.57 |
| x | 477 | 9.93 |
| x | 607 | 13.36 |
| x | 1037 | 12.03 |
| x | 1058 | 8.92 |
| x | 1105 | 8.62 |
| x | 1160 | 18.93 |
| x | 1221 | 8.25 |
| x | 1454 | 8.04 |
| y | 75 | 19.97 |
| y | 88 | 27.66 |
| y | 157 | 25.24 |
| y | 170 | 28.19 |
| y | 184 | 19.14 |
| y | 259 | 17.75 |
| y | 279 | 17.69 |
| y | 293 | 17.35 |
| y | 471 | 14.64 |
| y | 484 | 19.62 |
| y | 709 | 18.33 |
| y | 723 | 18.75 |
| y | 737 | 23.61 |
| y | 826 | 30.16 |
| y | 983 | 18.21 |
| y | 996 | 21.73 |


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
