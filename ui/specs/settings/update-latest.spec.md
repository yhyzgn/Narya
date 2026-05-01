# UI Spec — `ui/settings/update-latest.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/update-latest.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | update latest |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 33.44% | 0.996 |
| 2 | `#ffffff` | 8.39% | 1.0 |
| 3 | `#eff2f6` | 8.23% | 0.948 |
| 4 | `#fafbfe` | 7.61% | 0.984 |
| 5 | `#b0c1d2` | 7.32% | 0.748 |
| 6 | `#fbfbfe` | 7.17% | 0.985 |
| 7 | `#fdfdfe` | 6.7% | 0.992 |
| 8 | `#f8fafc` | 4.42% | 0.979 |
| 9 | `#fdfefe` | 3.36% | 0.995 |
| 10 | `#fafafc` | 3.06% | 0.981 |
| 11 | `#fcfcfd` | 3.04% | 0.989 |
| 12 | `#fffefe` | 1.91% | 0.997 |
| 13 | `#fefdfe` | 1.68% | 0.993 |
| 14 | `#fefeff` | 1.32% | 0.996 |
| 15 | `#fcfdfd` | 1.2% | 0.991 |
| 16 | `#fefefd` | 1.14% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#a8b6ca` |
| top_center | `#b9c1cd` |
| top_right | `#9eabbd` |
| center | `#fefefe` |
| bottom_left | `#d7dde9` |
| bottom_center | `#d7dae4` |
| bottom_right | `#c6cddd` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 15.55 |
| x | 54 | 9.37 |
| x | 68 | 8.06 |
| x | 81 | 8.51 |
| x | 102 | 11.59 |
| x | 259 | 7.63 |
| x | 300 | 31.02 |
| x | 314 | 6.71 |
| x | 341 | 6.8 |
| x | 525 | 6.89 |
| x | 764 | 7.29 |
| x | 887 | 17.87 |
| x | 907 | 7.3 |
| x | 1167 | 13.27 |
| x | 1310 | 6.87 |
| x | 1440 | 9.4 |
| y | 68 | 22.44 |
| y | 81 | 26.81 |
| y | 150 | 21.11 |
| y | 170 | 24.38 |
| y | 191 | 20.02 |
| y | 327 | 19.28 |
| y | 395 | 17.78 |
| y | 436 | 22.59 |
| y | 471 | 32.28 |
| y | 580 | 23.99 |
| y | 600 | 16.64 |
| y | 614 | 24.81 |
| y | 744 | 16.71 |
| y | 757 | 21.47 |
| y | 771 | 30.92 |
| y | 907 | 21.13 |


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
