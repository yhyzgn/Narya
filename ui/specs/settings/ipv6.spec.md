# UI Spec — `ui/settings/ipv6.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/ipv6.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | ipv6 |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fdfdfe` | 16.55% | 0.992 |
| 2 | `#fefefe` | 15.69% | 0.996 |
| 3 | `#f9fafd` | 10.34% | 0.98 |
| 4 | `#fbfcfd` | 9.19% | 0.988 |
| 5 | `#edf1f5` | 8.01% | 0.943 |
| 6 | `#fefdfe` | 7.92% | 0.993 |
| 7 | `#fafbfd` | 7.74% | 0.984 |
| 8 | `#b6bfda` | 6.92% | 0.749 |
| 9 | `#ffffff` | 6.27% | 1.0 |
| 10 | `#fdfdfd` | 2.99% | 0.992 |
| 11 | `#fdfefe` | 2.48% | 0.995 |
| 12 | `#fcfdfe` | 2.1% | 0.992 |
| 13 | `#fefeff` | 1.85% | 0.996 |
| 14 | `#fffeff` | 1.19% | 0.997 |
| 15 | `#fefefd` | 0.54% | 0.996 |
| 16 | `#fdfdff` | 0.23% | 0.993 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#b6bfce` |
| top_center | `#9aa7b7` |
| top_right | `#b0bcc9` |
| center | `#f2f4f8` |
| bottom_left | `#b4bdcc` |
| bottom_center | `#7e8da4` |
| bottom_right | `#afbacd` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 34 | 19.98 |
| x | 54 | 14.03 |
| x | 95 | 14.2 |
| x | 307 | 14.25 |
| x | 327 | 12.33 |
| x | 341 | 12.14 |
| x | 368 | 9.5 |
| x | 498 | 12.8 |
| x | 675 | 10.3 |
| x | 689 | 9.47 |
| x | 723 | 9.1 |
| x | 805 | 12.63 |
| x | 1153 | 22.17 |
| x | 1419 | 12.3 |
| x | 1433 | 12.39 |
| x | 1488 | 13.05 |
| y | 75 | 21.87 |
| y | 88 | 29.28 |
| y | 163 | 24.72 |
| y | 184 | 15.89 |
| y | 232 | 19.7 |
| y | 245 | 21.98 |
| y | 307 | 16.49 |
| y | 361 | 13.55 |
| y | 395 | 49.52 |
| y | 409 | 13.32 |
| y | 430 | 40.79 |
| y | 853 | 25.84 |
| y | 894 | 14.92 |
| y | 914 | 15.65 |
| y | 976 | 17.06 |
| y | 989 | 16.04 |


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
