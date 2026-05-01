# UI Spec — `ui/settings/notifications.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/notifications.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | notifications |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 35.66% | 0.996 |
| 2 | `#fafafe` | 7.11% | 0.982 |
| 3 | `#fdfdfe` | 7.04% | 0.992 |
| 4 | `#fbfcfd` | 7.0% | 0.988 |
| 5 | `#ffffff` | 6.97% | 1.0 |
| 6 | `#fafbfe` | 6.77% | 0.984 |
| 7 | `#d6dce6` | 4.93% | 0.861 |
| 8 | `#f7f7fa` | 4.78% | 0.969 |
| 9 | `#f9fafd` | 4.25% | 0.98 |
| 10 | `#eceff6` | 3.88% | 0.937 |
| 11 | `#a9b1ce` | 3.34% | 0.696 |
| 12 | `#fefdfe` | 2.24% | 0.993 |
| 13 | `#fffefe` | 2.0% | 0.997 |
| 14 | `#fefefd` | 1.61% | 0.996 |
| 15 | `#fcfdfd` | 1.3% | 0.991 |
| 16 | `#fefeff` | 1.13% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#bac6d6` |
| top_center | `#dee3e9` |
| top_right | `#c1cad3` |
| center | `#fdfdfe` |
| bottom_left | `#bbc2d1` |
| bottom_center | `#dee2eb` |
| bottom_right | `#bdc3d3` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 19.87 |
| x | 61 | 11.37 |
| x | 81 | 8.76 |
| x | 95 | 10.72 |
| x | 307 | 21.86 |
| x | 368 | 12.9 |
| x | 498 | 12.52 |
| x | 621 | 20.46 |
| x | 634 | 19.97 |
| x | 648 | 8.61 |
| x | 696 | 8.37 |
| x | 928 | 11.23 |
| x | 969 | 7.99 |
| x | 1092 | 10.44 |
| x | 1181 | 10.31 |
| x | 1426 | 9.83 |
| y | 75 | 16.73 |
| y | 129 | 17.36 |
| y | 170 | 17.33 |
| y | 184 | 21.02 |
| y | 245 | 16.57 |
| y | 259 | 18.12 |
| y | 279 | 14.28 |
| y | 320 | 15.58 |
| y | 430 | 14.61 |
| y | 443 | 16.3 |
| y | 512 | 14.05 |
| y | 791 | 22.5 |
| y | 819 | 20.75 |
| y | 846 | 20.07 |
| y | 983 | 19.9 |
| y | 996 | 16.3 |


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
