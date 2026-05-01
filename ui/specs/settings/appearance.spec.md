# UI Spec — `ui/settings/appearance.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/appearance.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | appearance |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 28.05% | 0.996 |
| 2 | `#fdfdfe` | 9.22% | 0.992 |
| 3 | `#edf0f8` | 8.69% | 0.941 |
| 4 | `#a1a8c9` | 6.9% | 0.662 |
| 5 | `#f9fafd` | 6.75% | 0.98 |
| 6 | `#fafbfd` | 6.29% | 0.984 |
| 7 | `#fbfcfd` | 6.23% | 0.988 |
| 8 | `#fafafd` | 5.98% | 0.981 |
| 9 | `#ffffff` | 5.13% | 1.0 |
| 10 | `#f7f8fc` | 4.97% | 0.973 |
| 11 | `#fefdfe` | 4.6% | 0.993 |
| 12 | `#fdfefe` | 2.17% | 0.995 |
| 13 | `#fffefe` | 1.39% | 0.997 |
| 14 | `#fcfdfe` | 1.32% | 0.992 |
| 15 | `#fefefd` | 1.3% | 0.996 |
| 16 | `#fefeff` | 1.0% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#8a98a9` |
| top_center | `#c4cad4` |
| top_right | `#929eab` |
| center | `#fefefe` |
| bottom_left | `#8990a0` |
| bottom_center | `#d9dce6` |
| bottom_right | `#8890a0` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 16.96 |
| x | 81 | 9.79 |
| x | 307 | 16.14 |
| x | 341 | 10.5 |
| x | 368 | 12.86 |
| x | 491 | 10.99 |
| x | 505 | 11.43 |
| x | 641 | 11.93 |
| x | 696 | 11.31 |
| x | 744 | 28.26 |
| x | 894 | 24.03 |
| x | 1037 | 30.42 |
| x | 1078 | 10.44 |
| x | 1105 | 33.49 |
| x | 1167 | 11.68 |
| x | 1187 | 10.42 |
| y | 54 | 14.55 |
| y | 75 | 14.25 |
| y | 88 | 20.81 |
| y | 157 | 14.01 |
| y | 184 | 20.01 |
| y | 197 | 14.87 |
| y | 307 | 43.62 |
| y | 423 | 41.7 |
| y | 471 | 27.84 |
| y | 505 | 33.62 |
| y | 573 | 23.63 |
| y | 621 | 14.28 |
| y | 675 | 14.14 |
| y | 839 | 13.89 |
| y | 914 | 27.66 |
| y | 983 | 20.43 |


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
