# UI Spec — `ui/settings/update.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/update.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | update |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 24.64% | 0.996 |
| 2 | `#fdfdfe` | 12.16% | 0.992 |
| 3 | `#f9fafd` | 10.69% | 0.98 |
| 4 | `#fafbfe` | 8.78% | 0.984 |
| 5 | `#f1f3f9` | 8.6% | 0.953 |
| 6 | `#bdc5da` | 7.87% | 0.772 |
| 7 | `#fbfcfd` | 6.86% | 0.988 |
| 8 | `#ffffff` | 6.19% | 1.0 |
| 9 | `#fdfefe` | 3.7% | 0.995 |
| 10 | `#fefdfe` | 2.6% | 0.993 |
| 11 | `#fdfdfd` | 2.2% | 0.992 |
| 12 | `#fcfdfe` | 2.07% | 0.992 |
| 13 | `#fefeff` | 1.63% | 0.996 |
| 14 | `#fffefe` | 1.24% | 0.997 |
| 15 | `#fefefd` | 0.61% | 0.996 |
| 16 | `#fdfdff` | 0.16% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#c5d0df` |
| top_center | `#dadfe6` |
| top_right | `#c4cdd7` |
| center | `#fefefe` |
| bottom_left | `#d2d8e5` |
| bottom_center | `#d6dbe5` |
| bottom_right | `#c0c6d6` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 20.99 |
| x | 54 | 18.9 |
| x | 81 | 16.44 |
| x | 109 | 15.52 |
| x | 259 | 6.71 |
| x | 300 | 14.06 |
| x | 314 | 7.27 |
| x | 341 | 10.0 |
| x | 361 | 13.48 |
| x | 375 | 8.07 |
| x | 484 | 32.67 |
| x | 498 | 14.21 |
| x | 832 | 10.58 |
| x | 955 | 7.22 |
| x | 1160 | 11.47 |
| x | 1215 | 6.94 |
| y | 75 | 20.66 |
| y | 88 | 17.1 |
| y | 129 | 17.76 |
| y | 170 | 22.05 |
| y | 191 | 18.3 |
| y | 245 | 16.11 |
| y | 259 | 16.68 |
| y | 361 | 19.9 |
| y | 491 | 25.92 |
| y | 669 | 17.54 |
| y | 682 | 18.21 |
| y | 696 | 22.07 |
| y | 709 | 20.03 |
| y | 723 | 28.18 |
| y | 983 | 24.82 |
| y | 996 | 19.44 |


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
