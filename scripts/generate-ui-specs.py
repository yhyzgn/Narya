#!/usr/bin/env python3
from __future__ import annotations

import json
import math
import os
from collections import Counter
from pathlib import Path
from typing import Dict, List, Tuple

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
UI_DIR = ROOT / "ui"
SPEC_DIR = UI_DIR / "specs"

PRIMARY_PAGES = {
    "dashboard.png": ("/dashboard", "仪表盘", "high-fidelity-custom"),
    "nodes.png": ("/nodes", "节点", "element-plus-business-page"),
    "config.png": ("/profiles", "配置", "element-plus-business-page"),
    "subscriptions.png": ("/subscriptions", "订阅", "element-plus-business-page"),
    "connections.png": ("/connections", "连接", "element-plus-business-page"),
    "rules.png": ("/rules", "规则", "element-plus-business-page"),
    "logs.png": ("/logs", "日志", "element-plus-business-page"),
    "tools.png": ("/tools", "工具箱", "element-plus-business-page"),
    "settings.png": ("/settings", "设置", "element-plus-business-page"),
    "splash.png": ("splash-window", "启动页", "high-fidelity-custom"),
}

PAGE_FAMILIES = {
    "dashboard": "Dashboard dashboard child/detail state",
    "nodes": "Nodes node management child/detail state",
    "config": "Profile/config editor child state",
    "subscriptions": "Subscription import/update child state",
    "connections": "Connection inspection child/detail state",
    "rules": "Rule editor/simulator child state",
    "logs": "Logs/export diagnostics child state",
    "settings": "Settings category child page",
    "tools": "Tools/diagnostics child state",
    "icons": "Brand/icon asset",
}

GLOBAL_SHELL_SPEC = """
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

"""

GLOBAL_STYLE_SPEC = """
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

"""


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def hex_color(rgb: Tuple[int, int, int]) -> str:
    return "#%02x%02x%02x" % rgb


def luminance(rgb: Tuple[int, int, int]) -> float:
    r, g, b = [v / 255 for v in rgb]
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def color_distance(a: Tuple[int, int, int], b: Tuple[int, int, int]) -> float:
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))


def palette(im: Image.Image, n: int = 16) -> List[Dict[str, object]]:
    rgba = im.convert("RGBA")
    # Composite transparent pixels over white so transparent logo assets still produce useful palettes.
    white = Image.new("RGBA", rgba.size, (255, 255, 255, 255))
    comp = Image.alpha_composite(white, rgba).convert("RGB")
    small = comp.copy()
    small.thumbnail((360, 240))
    pal = small.quantize(colors=n, method=Image.Quantize.MEDIANCUT).convert("RGB")
    counts = Counter(pal.getdata())
    total = sum(counts.values()) or 1
    rows = []
    for color, count in counts.most_common(n):
        rows.append({
            "hex": hex_color(color),
            "rgb": color,
            "percent": round(count / total * 100, 2),
            "luminance": round(luminance(color), 3),
        })
    return rows


def sample_points(im: Image.Image) -> Dict[str, str]:
    rgb = im.convert("RGB")
    w, h = rgb.size
    pts = {
        "top_left": (0, 0),
        "top_center": (w // 2, 0),
        "top_right": (w - 1, 0),
        "center": (w // 2, h // 2),
        "bottom_left": (0, h - 1),
        "bottom_center": (w // 2, h - 1),
        "bottom_right": (w - 1, h - 1),
    }
    return {name: hex_color(rgb.getpixel(pt)) for name, pt in pts.items()}


def detect_separators(im: Image.Image, axis: str, max_items: int = 16) -> List[Dict[str, object]]:
    """Detect likely large visual boundaries by comparing block averages.

    This is intentionally conservative and is only used as a machine hint, not a claim
    of exact component geometry.
    """
    rgb = im.convert("RGB")
    w, h = rgb.size
    # Downsample long screenshots to reduce cost but keep coordinate scale.
    max_dim = 900
    scale = min(1.0, max_dim / max(w, h))
    if scale < 1:
        small = rgb.resize((max(1, int(w * scale)), max(1, int(h * scale))))
    else:
        small = rgb
    sw, sh = small.size
    pix = small.load()
    block = 4
    values = []
    if axis == "x":
        for x in range(0, sw, block):
            rs = gs = bs = c = 0
            for xx in range(x, min(sw, x + block)):
                for yy in range(0, sh, max(1, sh // 180)):
                    r, g, b = pix[xx, yy]
                    rs += r; gs += g; bs += b; c += 1
            values.append((x, (rs / c, gs / c, bs / c)))
    else:
        for y in range(0, sh, block):
            rs = gs = bs = c = 0
            for yy in range(y, min(sh, y + block)):
                for xx in range(0, sw, max(1, sw // 180)):
                    r, g, b = pix[xx, yy]
                    rs += r; gs += g; bs += b; c += 1
            values.append((y, (rs / c, gs / c, bs / c)))
    diffs = []
    for i in range(1, len(values)):
        pos = values[i][0]
        a = values[i-1][1]; b = values[i][1]
        d = color_distance(a, b)
        # ignore near-image edges and tiny noise
        orig_pos = int(pos / scale) if scale else pos
        dim = w if axis == "x" else h
        if 8 < orig_pos < dim - 8 and d > 4.5:
            diffs.append((orig_pos, d))
    # Merge nearby peaks.
    diffs.sort(key=lambda x: x[1], reverse=True)
    chosen = []
    for pos, score in diffs:
        if all(abs(pos - p) > 12 for p, _ in chosen):
            chosen.append((pos, score))
        if len(chosen) >= max_items:
            break
    chosen.sort()
    return [{"pos": p, "score": round(s, 2)} for p, s in chosen]


def page_meta(path: Path, w: int, h: int) -> Dict[str, str]:
    relp = rel(path)
    name = path.name
    parent = path.parent.name
    if relp.startswith("ui/icons/"):
        return {"kind": "brand/icon asset", "route": "asset", "title": name, "strategy": "hand-vector-or-logo-asset"}
    if name in PRIMARY_PAGES:
        route, title, strategy = PRIMARY_PAGES[name]
        return {"kind": "primary page", "route": route, "title": title, "strategy": strategy}
    if name == "splash-background-2k.png":
        return {"kind": "background asset", "route": "splash asset", "title": "Splash background", "strategy": "raster-reference-or-css-gradient"}
    fam = PAGE_FAMILIES.get(parent, f"{parent} child state")
    return {"kind": "secondary/dialog/detail state", "route": f"{parent} detail/state", "title": path.stem.replace('-', ' '), "strategy": "element-plus-business-state-or-custom-modal", "family": fam}


def specific_notes(path: Path, w: int, h: int) -> str:
    relp = rel(path)
    name = path.name
    parent = path.parent.name
    if name == "dashboard.png":
        return """
### Dashboard-specific implementation notes

- Treat Dashboard as a custom page, not an Element Plus page.
- Canonical reference viewport is 1536×1024.
- Reference left sidebar is about 270px wide with logo/title/version at top, nav items below, connection status card near bottom, and three action icons at the very bottom.
- Top action icons in the design are compact and sit in the upper-right content header area. In the current Tauri app native window controls are used; do not draw extra min/max/close inside the page.
- Main cards: two top proxy mode cards, quick connect list, network overview, traffic usage, protocol stats, activity log.
- Toggle switches should feel compact relative to card titles; avoid visually heavy switches.
- Metric numerals should be prominent but not oversized; avoid wasting vertical space.
- Chart legends should sit in header whitespace where possible rather than stealing graph plotting area.
- Preserve the pale blue/green translucent background, soft borders, and compact desktop density.
"""
    if name == "splash.png":
        return """
### Splash-specific implementation notes

- Treat Splash as custom high-fidelity composition.
- Preserve fixed window geometry and centered logo/loading composition.
- Avoid using generic component library defaults for progress, text spacing, or glass surfaces unless theme overrides match the PNG exactly.
- If animated, the final still at representative progress states must remain spatially aligned to this PNG.
"""
    if name in PRIMARY_PAGES and name not in {"dashboard.png", "splash.png"}:
        return f"""
### {PRIMARY_PAGES[name][1]} primary-page implementation notes

- This is a business page and may use Element Plus for interaction-heavy controls.
- Keep AppShell geometry consistent with the 1536×1024 full-page references.
- Use Element Plus only after `tokens.css` and `element-plus-theme.css` are loaded.
- Match the reference page's card density, table row height, filters, side panels, and empty-space rhythm before adding dynamic data.
- Preserve the page title, toolbar hierarchy, and footer status band from the PNG.
"""
    if relp.startswith("ui/settings/"):
        return """
### Settings child-page notes

- Best candidate family for the Element Plus sample migration.
- Use `ElForm`, `ElSwitch`, `ElSelect`, `ElInputNumber`, `ElTabs`/segmented nav, `ElTooltip`, and `ElMessage` where useful.
- Theme every control to Narya density: compact labels, 32–36px controls, soft border, subtle active blue/green.
- Preserve left settings category navigation and right form-section grouping exactly from the screenshot.
"""
    if relp.startswith("ui/subscriptions/add-"):
        return """
### Subscription import flow notes

- Treat as a multi-step/import-state family.
- Use Element Plus Dialog/Drawer/Form/Input/Upload/Steps only if their visual chrome is fully themed.
- Preserve modal/card size, centered alignment, success/preview states, QR/upload/manual/url method hierarchy.
"""
    if parent in {"nodes", "connections", "rules", "logs", "tools", "config"}:
        return f"""
### {parent} secondary-state notes

- This screenshot describes a specific interaction/detail state, not just a static page.
- Preserve modal/detail panel geometry, table/form density, selected row state, and action hierarchy.
- Element Plus can supply the controls, but the layout, colors, and spacing must be driven by this PNG.
"""
    if relp.startswith("ui/icons/"):
        return """
### Icon asset notes

- This is a brand/logo raster reference, not a UI page.
- If a real SVG is needed, hand-vector the logo with gradients; traced paths are acceptable only as a temporary approximation.
- Preserve transparent background where present and never add a white bounding box unless the source has one.
"""
    return """
### Asset-specific notes

- Use this PNG as the precise visual source for dimensions, color, and composition.
- If implemented in code, first classify whether it is a full page, modal state, panel state, or standalone asset.
"""


def write_spec(path: Path) -> Dict[str, object]:
    im = Image.open(path)
    w, h = im.size
    meta = page_meta(path, w, h)
    pal = palette(im)
    samples = sample_points(im)
    xsep = detect_separators(im, "x")
    ysep = detect_separators(im, "y")
    out_path = SPEC_DIR / path.relative_to(UI_DIR).with_suffix(".spec.md")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    aspect = round(w / h, 4)
    alpha = im.mode in ("RGBA", "LA") or ("transparency" in im.info)

    palette_table = "\n".join(
        f"| {i+1} | `{row['hex']}` | {row['percent']}% | {row['luminance']} |"
        for i, row in enumerate(pal)
    )
    sample_table = "\n".join(f"| {k} | `{v}` |" for k, v in samples.items())
    x_table = "\n".join(f"| x | {r['pos']} | {r['score']} |" for r in xsep) or "| — | — | — |"
    y_table = "\n".join(f"| y | {r['pos']} | {r['score']} |" for r in ysep) or "| — | — | — |"

    shared = GLOBAL_SHELL_SPEC if (w, h) == (1536, 1024) and not rel(path).startswith("ui/icons/") else ""
    content = f"""# UI Spec — `{rel(path)}`

> Generated from the PNG reference as a developer handoff spec. This file intentionally contains no embedded raster image. Use the source PNG for final pixel comparison and this spec for layout, token, and implementation guidance.

## 1. Identity

| Field | Value |
|---|---|
| Source PNG | `{rel(path)}` |
| Canvas | {w}×{h}px |
| Aspect ratio | {aspect} |
| Alpha / transparency | {str(alpha).lower()} |
| Kind | {meta.get('kind', '')} |
| Route / state | `{meta.get('route', '')}` |
| Human title | {meta.get('title', '')} |
| Recommended implementation strategy | `{meta.get('strategy', '')}` |

## 2. Pixel-derived palette

These are quantized dominant colors sampled from the screenshot. Use them as implementation hints, then verify against the PNG.

| Rank | Color | Approx. coverage | Luminance |
|---:|---|---:|---:|
{palette_table}

## 3. Edge/corner samples

| Sample point | Color |
|---|---|
{sample_table}

## 4. Machine-detected visual boundary hints

These positions are high-contrast boundary candidates; they are not a substitute for visual QA but are useful for quickly finding sidebars, cards, separators, dialogs, and footer bands.

| Axis | Position px | Relative score |
|---|---:|---:|
{x_table}
{y_table}

{shared}{GLOBAL_STYLE_SPEC}{specific_notes(path, w, h)}
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
"""
    out_path.write_text(content)
    return {
        "source": rel(path),
        "spec": rel(out_path),
        "width": w,
        "height": h,
        "aspectRatio": aspect,
        "alpha": alpha,
        "kind": meta.get("kind", ""),
        "route": meta.get("route", ""),
        "title": meta.get("title", ""),
        "strategy": meta.get("strategy", ""),
        "palette": pal,
        "samples": samples,
        "separators": {"x": xsep, "y": ysep},
    }


def main() -> None:
    SPEC_DIR.mkdir(parents=True, exist_ok=True)
    pngs = [p for p in sorted(UI_DIR.rglob("*.png")) if "specs" not in p.parts]
    records = [write_spec(p) for p in pngs]

    index = {
        "generatedBy": "scripts/generate-ui-specs.py",
        "purpose": "Gemini/Codex UI implementation handoff; text specs derived from ui PNG references without embedding raster images.",
        "count": len(records),
        "records": records,
    }
    (SPEC_DIR / "spec-index.json").write_text(json.dumps(index, ensure_ascii=False, indent=2))

    # README and global guide.
    by_size: Dict[str, int] = {}
    for r in records:
        key = f"{r['width']}x{r['height']}"
        by_size[key] = by_size.get(key, 0) + 1
    rows = "\n".join(f"| {k} | {v} |" for k, v in sorted(by_size.items()))
    spec_rows = "\n".join(f"| `{r['source']}` | `{r['spec']}` | {r['width']}×{r['height']} | {r['strategy']} |" for r in records)
    (SPEC_DIR / "README.md").write_text(f"""# Narya UI Reference Specs

This directory contains generated, text-first specs for every PNG under `ui/`.

The specs are meant for Gemini/Codex implementation handoff: the original PNG remains the visual truth source, while the Markdown/JSON files provide dimensions, palette, route/state classification, boundary hints, and implementation rules.

## Important constraints

- No generated spec embeds the original PNG.
- These files are not traced SVGs; they are implementation specs.
- Full semantic vector reconstruction from PNG is not lossless. If true SVG design assets are required, they must be hand-authored or exported from the original design tool.
- Dashboard/Splash/AppShell remain custom high-fidelity areas.
- Business pages may use Element Plus only after Narya token/theme overrides are applied.

## Size summary

| Canvas | Count |
|---|---:|
{rows}

## Spec index

Machine-readable index: `ui/specs/spec-index.json`.

| Source | Spec | Canvas | Strategy |
|---|---|---:|---|
{spec_rows}
""")

    (SPEC_DIR / "_global-design-system.spec.md").write_text(f"""# Global Narya UI Design System Spec

This file consolidates implementation rules that apply across the generated per-image specs.

{GLOBAL_STYLE_SPEC}
{GLOBAL_SHELL_SPEC}
## Element Plus adoption boundary

| Area | Element Plus usage | Reason |
|---|---|---|
| Splash | Avoid for core visuals | Requires exact custom composition and animation timing. |
| Dashboard | Avoid for cards/layout/switches; allow tooltip/popover/dialog behavior if themed | Dashboard is the most visually sensitive reference page. |
| AppShell | Avoid default layout components | Sidebar/header/footer geometry must remain consistent across references. |
| Settings/Profile/Nodes/Subscriptions/Connections/Rules/Logs/Tools | Use Element Plus after theme override | These are interaction-heavy business pages. |

## Current dashboard density constraints captured from implementation feedback

- Top action icon target: about 20px.
- Sidebar bottom action icon target: about 23px.
- Dashboard custom switch target: about 52×29px with 23px knob.
- Main emphasized dashboard metric numerals: about 23px.
- Network chart legend belongs in right-side header whitespace.
- Traffic chart legend belongs in the traffic card header, before the date selector.
- Avoid outer body/frame border when Tauri native window chrome is enabled.

## Recommended workflow for Gemini

1. Read `ui/specs/README.md` and this file.
2. Open the specific `*.spec.md` for the target PNG.
3. Use `spec-index.json` to query exact dimensions, palette, and boundary hints.
4. Implement static layout first.
5. Add Element Plus controls only for business interactions and only after theme override.
6. Screenshot at the same viewport and compare to source PNG.
""")

    print(f"generated {len(records)} specs in {SPEC_DIR}")

if __name__ == "__main__":
    main()
