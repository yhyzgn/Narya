# UI Spec — `ui/settings/kernel.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/kernel.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | kernel |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 29.49% | 0.996 |
| 2 | `#fdfdfe` | 9.06% | 0.992 |
| 3 | `#fafbfe` | 7.99% | 0.984 |
| 4 | `#fcfcfd` | 7.2% | 0.989 |
| 5 | `#f9fafd` | 6.83% | 0.98 |
| 6 | `#ffffff` | 6.14% | 1.0 |
| 7 | `#f7f8fc` | 5.74% | 0.973 |
| 8 | `#f1f4f9` | 5.19% | 0.956 |
| 9 | `#dde3eb` | 4.77% | 0.887 |
| 10 | `#fafafd` | 4.63% | 0.981 |
| 11 | `#afb9c8` | 4.17% | 0.721 |
| 12 | `#fefdfe` | 2.39% | 0.993 |
| 13 | `#fffefe` | 1.97% | 0.997 |
| 14 | `#fcfdfd` | 1.67% | 0.991 |
| 15 | `#fefefd` | 1.46% | 0.996 |
| 16 | `#fefeff` | 1.29% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#c3cddb` |
| top_center | `#c7ced7` |
| top_right | `#c0c8d0` |
| center | `#fdfdfd` |
| bottom_left | `#d4dae2` |
| bottom_center | `#dbdee6` |
| bottom_right | `#cfd4de` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 24.72 |
| x | 54 | 17.78 |
| x | 81 | 15.81 |
| x | 95 | 5.88 |
| x | 109 | 14.51 |
| x | 300 | 12.85 |
| x | 314 | 12.82 |
| x | 327 | 9.18 |
| x | 354 | 5.72 |
| x | 484 | 7.88 |
| x | 498 | 20.62 |
| x | 525 | 21.0 |
| x | 1037 | 6.42 |
| x | 1160 | 18.22 |
| x | 1181 | 7.52 |
| x | 1276 | 8.08 |
| y | 61 | 15.58 |
| y | 81 | 29.26 |
| y | 129 | 16.72 |
| y | 163 | 20.88 |
| y | 177 | 13.31 |
| y | 238 | 14.51 |
| y | 279 | 17.16 |
| y | 314 | 14.46 |
| y | 341 | 14.17 |
| y | 361 | 15.48 |
| y | 409 | 13.71 |
| y | 771 | 26.59 |
| y | 846 | 13.1 |
| y | 928 | 21.89 |
| y | 983 | 21.38 |
| y | 996 | 20.64 |


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
