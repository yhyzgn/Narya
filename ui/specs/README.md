# Narya UI Reference Specs

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
| 1080x1080 | 1 |
| 1254x1254 | 1 |
| 1536x1024 | 46 |
| 1672x941 | 1 |
| 1920x1080 | 1 |
| 2048x2048 | 1 |
| 2560x1440 | 1 |

## Spec index

Machine-readable index: `ui/specs/spec-index.json`.

| Source | Spec | Canvas | Strategy |
|---|---|---:|---|
| `ui/config/visual-editor.png` | `ui/specs/config/visual-editor.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/config/yaml-editor.png` | `ui/specs/config/yaml-editor.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/config.png` | `ui/specs/config.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/connections/connection-detail.png` | `ui/specs/connections/connection-detail.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/connections.png` | `ui/specs/connections.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/dashboard/system-proxy-rule-mode.png` | `ui/specs/dashboard/system-proxy-rule-mode.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/dashboard/tun-smart-routing.png` | `ui/specs/dashboard/tun-smart-routing.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/dashboard.png` | `ui/specs/dashboard.spec.md` | 1536×1024 | high-fidelity-custom |
| `ui/icons/narya-logo-raw-chroma.png` | `ui/specs/icons/narya-logo-raw-chroma.spec.md` | 1254×1254 | hand-vector-or-logo-asset |
| `ui/icons/narya-logo-transparent-1080.png` | `ui/specs/icons/narya-logo-transparent-1080.spec.md` | 1080×1080 | hand-vector-or-logo-asset |
| `ui/icons/narya-logo-transparent-1920x1080.png` | `ui/specs/icons/narya-logo-transparent-1920x1080.spec.md` | 1920×1080 | hand-vector-or-logo-asset |
| `ui/icons/narya-logo-transparent-2k-2048.png` | `ui/specs/icons/narya-logo-transparent-2k-2048.spec.md` | 2048×2048 | hand-vector-or-logo-asset |
| `ui/icons/narya-logo-transparent-2k-2560x1440.png` | `ui/specs/icons/narya-logo-transparent-2k-2560x1440.spec.md` | 2560×1440 | hand-vector-or-logo-asset |
| `ui/logs/export-diagnostics.png` | `ui/specs/logs/export-diagnostics.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/logs.png` | `ui/specs/logs.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/nodes/latency-test.png` | `ui/specs/nodes/latency-test.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/nodes/node-detail.png` | `ui/specs/nodes/node-detail.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/nodes.png` | `ui/specs/nodes.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/rules/rule-editor.png` | `ui/specs/rules/rule-editor.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/rules/rule-simulator.png` | `ui/specs/rules/rule-simulator.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/rules.png` | `ui/specs/rules.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/settings/advanced.png` | `ui/specs/settings/advanced.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/appearance.png` | `ui/specs/settings/appearance.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/dns.png` | `ui/specs/settings/dns.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/ipv6.png` | `ui/specs/settings/ipv6.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/kernel.png` | `ui/specs/settings/kernel.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/network.png` | `ui/specs/settings/network.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/notifications.png` | `ui/specs/settings/notifications.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/security.png` | `ui/specs/settings/security.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/tun.png` | `ui/specs/settings/tun.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/update-latest.png` | `ui/specs/settings/update-latest.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings/update.png` | `ui/specs/settings/update.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/settings.png` | `ui/specs/settings.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/splash-background-2k.png` | `ui/specs/splash-background-2k.spec.md` | 1672×941 | raster-reference-or-css-gradient |
| `ui/splash.png` | `ui/specs/splash.spec.md` | 1536×1024 | high-fidelity-custom |
| `ui/subscriptions/add-clipboard.png` | `ui/specs/subscriptions/add-clipboard.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-file.png` | `ui/specs/subscriptions/add-file.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-hiddify.png` | `ui/specs/subscriptions/add-hiddify.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-manual.png` | `ui/specs/subscriptions/add-manual.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-method.png` | `ui/specs/subscriptions/add-method.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-preview.png` | `ui/specs/subscriptions/add-preview.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-qr.png` | `ui/specs/subscriptions/add-qr.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-success.png` | `ui/specs/subscriptions/add-success.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions/add-url.png` | `ui/specs/subscriptions/add-url.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/subscriptions.png` | `ui/specs/subscriptions.spec.md` | 1536×1024 | element-plus-business-page |
| `ui/tools/diagnostic-report.png` | `ui/specs/tools/diagnostic-report.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools/dns-query.png` | `ui/specs/tools/dns-query.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools/geoip-query.png` | `ui/specs/tools/geoip-query.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools/mtr-trace.png` | `ui/specs/tools/mtr-trace.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools/port-check.png` | `ui/specs/tools/port-check.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools/speed-test.png` | `ui/specs/tools/speed-test.spec.md` | 1536×1024 | element-plus-business-state-or-custom-modal |
| `ui/tools.png` | `ui/specs/tools.spec.md` | 1536×1024 | element-plus-business-page |
