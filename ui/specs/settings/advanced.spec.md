# UI Spec — `ui/settings/advanced.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/advanced.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | advanced |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfd` | 20.47% | 0.992 |
| 2 | `#fefdfd` | 9.73% | 0.993 |
| 3 | `#f9fafc` | 9.15% | 0.98 |
| 4 | `#fefefe` | 9.15% | 0.996 |
| 5 | `#f5f4f7` | 7.82% | 0.959 |
| 6 | `#cccedb` | 7.76% | 0.81 |
| 7 | `#fafafc` | 7.07% | 0.981 |
| 8 | `#fbfbfd` | 6.37% | 0.985 |
| 9 | `#fafbfd` | 4.97% | 0.984 |
| 10 | `#fcfdfd` | 3.85% | 0.991 |
| 11 | `#fdfcfc` | 3.59% | 0.989 |
| 12 | `#ffffff` | 3.32% | 1.0 |
| 13 | `#fefefd` | 2.65% | 0.996 |
| 14 | `#fdfdfe` | 1.94% | 0.992 |
| 15 | `#fdfefe` | 1.91% | 0.995 |
| 16 | `#fdfdfc` | 0.24% | 0.992 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#b4bcc8` |
| top_center | `#8694aa` |
| top_right | `#ccd0d4` |
| center | `#fefdfd` |
| bottom_left | `#b8bcc8` |
| bottom_center | `#8b97ab` |
| bottom_right | `#b7bbc8` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 17.79 |
| x | 54 | 17.46 |
| x | 68 | 9.98 |
| x | 81 | 10.5 |
| x | 95 | 11.58 |
| x | 116 | 7.18 |
| x | 266 | 8.7 |
| x | 314 | 14.97 |
| x | 334 | 8.74 |
| x | 368 | 12.8 |
| x | 546 | 11.29 |
| x | 853 | 8.91 |
| x | 873 | 8.4 |
| x | 948 | 7.73 |
| x | 969 | 7.81 |
| x | 1126 | 15.98 |
| y | 75 | 22.63 |
| y | 88 | 29.5 |
| y | 129 | 17.37 |
| y | 163 | 14.56 |
| y | 177 | 14.54 |
| y | 191 | 15.39 |
| y | 238 | 11.86 |
| y | 273 | 13.63 |
| y | 675 | 14.6 |
| y | 723 | 15.17 |
| y | 805 | 20.04 |
| y | 819 | 13.12 |
| y | 907 | 12.75 |
| y | 942 | 13.65 |
| y | 976 | 13.51 |
| y | 989 | 25.12 |


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
