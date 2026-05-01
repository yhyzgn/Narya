# Global Narya UI Design System Spec

This file consolidates implementation rules that apply across the generated per-image specs.


### Global Narya visual rules

- Visual truth source: the referenced PNG itself; do not reinterpret proportions from memory.
- Preferred font stack: `SF Pro Text`, `Inter`, `Segoe UI`, `Noto Sans SC`, system-ui, sans-serif.
- Overall type density: compact desktop UI, closer to 12–14px body text and 16–21px card titles.
- Emphasized metric numerals: avoid oversized dashboard numerals; current dashboard density uses around 23px for major stats.
- Icon style: Lucide-like 2–2.35 stroke, compact sizes; avoid oversized decorative icons except explicit gradient card icons.
- Surface style: glassy white/translucent cards over pale blue background; keep borders and shadows restrained.
- Element Plus strategy: Element Plus is allowed for reusable controls and interaction behavior across the app, including Splash/Dashboard, **only after Narya theme overrides and per-component screenshot tuning**.
- Do not use default Element Plus blue/radius/row height directly. This means: never ship naked Element Plus defaults; always override them through `element-plus-theme.css`, Narya tokens, scoped `:deep(...)`, fixed dimensions, and screenshot comparison.
- Dashboard/Splash/AppShell are custom high-fidelity compositions for layout, geometry, card shells, charts, and brand visuals. Their inner controls may use themed Element Plus when the final pixels match the PNG/spec.



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


## Element Plus adoption boundary

Element Plus is a **control/interaction implementation layer**, not the visual art direction. Use it where it reduces custom interaction work, then theme it until it visually belongs to Narya.

| Area | Element Plus usage | Required customization / reason |
|---|---|---|
| Splash | Allowed for `ElProgress`, `ElText`, `ElImage`, `ElIcon`, lightweight status elements | The logo composition, window geometry, background, loading choreography, and final spacing remain custom high-fidelity. Progress/text/image components must be themed to the PNG. |
| Dashboard | Allowed for `ElSwitch`, `ElDropdown`, `ElPopover`, `ElTooltip`, `ElButton`, `ElTag`, `ElAvatar`, `ElScrollbar`, `ElDivider`, `ElStatistic`, `ElSelect` | Dashboard card layout, glass panels, grid geometry, line/area charts, Sidebar/TopBar/StatusFooter, and special gradients remain custom. Every EP control must be dimension-tuned to the PNG/spec. |
| AppShell | Avoid EP layout containers; allow only small themed action controls if they match | Sidebar/header/footer geometry must remain consistent across references; do not replace shell structure with EP Container/Menu defaults. |
| Settings/Profile/Nodes/Subscriptions/Connections/Rules/Logs/Tools | Prefer Element Plus after theme override | These are interaction-heavy business pages; use EP tables/forms/dialogs/drawers/selects where useful, but keep Narya spacing, density, colors, and glass surfaces. |

## Element Plus component tuning checklist

Before shipping any Element Plus component in Narya UI, verify:

- [ ] It imports after Narya tokens and `element-plus-theme.css` overrides.
- [ ] Default Element Plus primary blue is replaced with Narya blue/purple tokens where visible.
- [ ] Radius matches nearby Narya surfaces (typically 8–12px; pills only where the PNG shows pills).
- [ ] Height, padding, font size, icon size, and row height match the PNG/spec.
- [ ] Focus/hover/active/disabled states are themed and not visually jarring.
- [ ] Scoped `:deep(...)` overrides are used where CSS variables are insufficient.
- [ ] Screenshot at source canvas size confirms it does not look like a generic Element Plus admin template.

## Current dashboard density constraints captured from implementation feedback

- Top action icon target: about 20px.
- Sidebar bottom action icon target: about 23px.
- Dashboard switch target when using either custom CSS or themed `ElSwitch`: about 52×29px with 23px knob.
- Main emphasized dashboard metric numerals: about 23px, including themed `ElStatistic` values.
- Network chart legend belongs in right-side header whitespace.
- Traffic chart legend belongs in the traffic card header, before the date selector.
- Avoid outer body/frame border when Tauri native window chrome is enabled.
- Charts remain custom SVG/Canvas/ECharts/uPlot work; do not expect Element Plus to solve the line/area chart fidelity.

## Recommended workflow for Gemini

1. Read `ui/specs/README.md` and this file.
2. Open the specific `*.spec.md` for the target PNG.
3. Use `spec-index.json` to query exact dimensions, palette, and boundary hints.
4. Implement static layout first.
5. Add Element Plus controls where they reduce interaction work, but theme/tune each one before visual QA.
6. Screenshot at the same viewport and compare to source PNG.
