# UI Spec — `ui/settings/dns.png`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `ui/settings/dns.png` |
| Canvas | 1536×1024px |
| Aspect ratio | 1.5 |
| Alpha / transparency | false |
| Kind | secondary/dialog/detail state |
| Route / state | `settings detail/state` |
| Human title | dns |
| Recommended implementation strategy | `element-plus-business-state-or-custom-modal` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
| 1 | `#fefefe` | 26.32% | 0.996 |
| 2 | `#fafbfd` | 10.43% | 0.984 |
| 3 | `#f9fafd` | 9.97% | 0.98 |
| 4 | `#edf2f6` | 7.5% | 0.946 |
| 5 | `#c0c6df` | 6.93% | 0.779 |
| 6 | `#fdfdfe` | 6.91% | 0.992 |
| 7 | `#fcfcfd` | 5.66% | 0.989 |
| 8 | `#ffffff` | 5.56% | 1.0 |
| 9 | `#fefdfe` | 5.22% | 0.993 |
| 10 | `#f7f7fb` | 4.54% | 0.97 |
| 11 | `#fdfdfd` | 3.51% | 0.992 |
| 12 | `#fdfefe` | 2.21% | 0.995 |
| 13 | `#fffefe` | 1.47% | 0.997 |
| 14 | `#fefeff` | 1.34% | 0.996 |
| 15 | `#fcfdfd` | 1.34% | 0.991 |
| 16 | `#fefefd` | 1.07% | 0.996 |

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
| top_left | `#8494a5` |
| top_center | `#ccd3db` |
| top_right | `#8995a1` |
| center | `#fefefe` |
| bottom_left | `#8991a3` |
| bottom_center | `#d2d6e3` |
| bottom_right | `#848c9c` |

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
| x | 40 | 15.53 |
| x | 54 | 12.95 |
| x | 68 | 6.1 |
| x | 81 | 13.55 |
| x | 109 | 13.83 |
| x | 293 | 13.79 |
| x | 314 | 10.16 |
| x | 327 | 11.24 |
| x | 348 | 11.7 |
| x | 471 | 8.05 |
| x | 484 | 7.15 |
| x | 880 | 6.02 |
| x | 894 | 6.09 |
| x | 1058 | 6.54 |
| x | 1133 | 18.82 |
| x | 1181 | 6.66 |
| y | 54 | 18.54 |
| y | 68 | 19.43 |
| y | 81 | 30.02 |
| y | 150 | 17.8 |
| y | 177 | 17.61 |
| y | 191 | 16.53 |
| y | 238 | 19.01 |
| y | 252 | 16.17 |
| y | 300 | 38.63 |
| y | 327 | 55.73 |
| y | 375 | 21.48 |
| y | 464 | 17.27 |
| y | 737 | 18.38 |
| y | 832 | 23.09 |
| y | 873 | 15.73 |
| y | 1003 | 20.32 |


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
