# Narya UI Page Component Matrix

This is the human-readable companion to the generated per-image specs. It tells Gemini what each PNG is for, what controls must exist, and which implementation strategy to use.

## Global implementation rules

- Original PNG path is always the highest-priority visual source.
- Use `ui/specs/spec-index.json` for exact canvas, palette, and boundary hints.
- Use the matching `*.spec.md` file for per-image dimensions and visual constraints.
- Use Element Plus for business interactions only after Narya theme overrides are loaded.
- Do not replace Dashboard/Splash/AppShell core visuals with default Element Plus components.
- Every implemented page should be screenshot-tested against the same canvas as the source PNG.

## Main pages

| PNG | Route/state | Core regions | Required controls/interactions | Strategy |
|---|---|---|---|---|
| `ui/splash.png` | Splash window | centered brand/logo, loading status, progress area, startup state copy | timed loading progress, transition to main window, show main and close splash | custom high-fidelity CSS/animation |
| `ui/dashboard.png` | `/dashboard` | AppShell, proxy cards, quick nodes, network chart, traffic chart, protocol stats, activity log, status footer | system proxy toggle, TUN toggle, quick node select, mode dropdown/link, chart tooltips later | custom high-fidelity; behavior-only EP allowed |
| `ui/nodes.png` | `/nodes` | node table/list, filter/search toolbar, latency/status badges, side/detail zones | search, filter, latency test, select node, batch test, sort by latency | Element Plus business page with custom theme |
| `ui/config.png` | `/profiles` or `/config` | profile list/cards, active profile indicator, edit/import actions, editor entry points | create/edit/delete/switch profile, import/export, validate config | Element Plus business page |
| `ui/subscriptions.png` | `/subscriptions` | subscription list/cards/table, usage stats, update state, add entry | add subscription, update now, delete, edit URL, import methods | Element Plus business page |
| `ui/connections.png` | `/connections` | live connections table, process/source columns, rule/path columns, summary metrics | filter/search, inspect detail, disconnect connection, pause/resume refresh | Element Plus table-heavy page |
| `ui/rules.png` | `/rules` | rule groups, rule list/editor entry, simulator entry | create/edit/delete rule, reorder, enable/disable, simulate match | Element Plus forms/tables + custom rule chips |
| `ui/logs.png` | `/logs` | log stream/table, filters, severity chips, export/diagnostic action | filter by level/source, search, pause stream, export diagnostics | Element Plus table/log viewer |
| `ui/tools.png` | `/tools` | diagnostic tool grid, input panels, result cards | run DNS/MTR/port/GeoIP/speed test, show results and copy report | Element Plus forms + custom result cards |
| `ui/settings.png` | `/settings` | settings category rail, grouped forms, switches/selects/inputs | update preferences, validate fields, reset defaults, notify save | best Element Plus sample page |

## Dashboard child states

| PNG | Purpose | Must preserve | Implementation note |
|---|---|---|---|
| `ui/dashboard/system-proxy-rule-mode.png` | System proxy mode popover/dialog | placement, compact list density, active rule-mode visual state | behavior can use `ElPopover`/`ElDropdown`, but visual tokens must match Dashboard |
| `ui/dashboard/tun-smart-routing.png` | TUN smart routing popover/dialog | same shell density, compact option rows, selected state | behavior can use `ElPopover`/`ElDropdown`; avoid default EP menu styling |

## Nodes states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/nodes/latency-test.png` | latency test/progress result state | test progress, per-node latency badges, refresh/retest action | EP progress/table where theme matches |
| `ui/nodes/node-detail.png` | node detail drawer/panel | node metadata, latency, protocol, route, actions | `ElDrawer`/custom side panel with Narya styling |

## Config/Profile states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/config/visual-editor.png` | visual config editor | sections/forms, toggles/selects, validation hints | EP Form/Input/Select/Switch after theme override |
| `ui/config/yaml-editor.png` | YAML editor mode | editor pane, validation/output area, save/check actions | Monaco later; EP buttons/messages acceptable |

## Subscription import flow

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/subscriptions/add-method.png` | choose import method | method cards/buttons | EP Dialog + custom method cards |
| `ui/subscriptions/add-url.png` | import by URL | URL input, validation, confirm/cancel | EP Form/Input/Dialog |
| `ui/subscriptions/add-file.png` | import file | upload/dropzone, accepted type hint | EP Upload themed |
| `ui/subscriptions/add-clipboard.png` | import clipboard | paste area, parse/validate action | EP Input textarea/Dialog |
| `ui/subscriptions/add-qr.png` | import QR | QR scan/upload area, status | custom panel + EP Upload |
| `ui/subscriptions/add-hiddify.png` | Hiddify import | account/link fields, connect action | EP Form/Dialog |
| `ui/subscriptions/add-manual.png` | manual subscription creation | name/url/options fields | EP Form/Dialog |
| `ui/subscriptions/add-preview.png` | preview parsed subscription | node list preview, traffic info, confirm import | EP Table/List themed |
| `ui/subscriptions/add-success.png` | success result | success icon, summary, finish action | EP Result acceptable only if fully themed; custom preferred |

## Connections states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/connections/connection-detail.png` | connection detail panel | process, remote address, rule, path, timing, disconnect action | EP Drawer/Descriptions/Table with custom density |

## Rules states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/rules/rule-editor.png` | rule edit/create | rule type select, value inputs, action/policy selectors, validation | EP Form/Dialog/Select/Input |
| `ui/rules/rule-simulator.png` | rule simulation | target input, process/context options, matched path/result | EP Form + custom result timeline |

## Logs states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/logs/export-diagnostics.png` | export diagnostics dialog | checkbox groups, range/source selectors, export progress/result | EP Dialog/Form/Checkbox/Progress |

## Tools states

| PNG | Purpose | Required controls | Strategy |
|---|---|---|---|
| `ui/tools/dns-query.png` | DNS lookup tool | domain input, server select, run button, result rows | EP Form/Table |
| `ui/tools/mtr-trace.png` | MTR/traceroute tool | host input, run/stop, hop table/timeline | EP Form/Table + custom timeline |
| `ui/tools/port-check.png` | port check tool | host/port inputs, protocol select, result status | EP Form/InputNumber/Result |
| `ui/tools/geoip-query.png` | GeoIP query tool | IP/domain input, geo result card/map-ish layout | EP Form + custom result card |
| `ui/tools/speed-test.png` | speed test tool | start button, progress/gauge, download/upload stats | custom chart/gauge + EP button |
| `ui/tools/diagnostic-report.png` | diagnostic report | generated report sections, copy/export actions | EP Collapse/Table/Button |

## Settings child pages

All settings pages should be implemented as the first Element Plus sample family because they are form-heavy and validate the theme quickly.

| PNG | Category | Required controls |
|---|---|---|
| `ui/settings/appearance.png` | appearance | theme mode, accent color, density, language, font/display options |
| `ui/settings/network.png` | network | proxy/TUN related defaults, port fields, bypass options |
| `ui/settings/ipv6.png` | IPv6 | enable/disable, strategy select, DNS/route related options |
| `ui/settings/kernel.png` | kernel | kernel selection, version/path, update/check actions |
| `ui/settings/tun.png` | TUN | stack, auto-route, strict-route, interface/DNS options |
| `ui/settings/dns.png` | DNS | DNS server list, mode, fake-ip options, test action |
| `ui/settings/security.png` | security | permission/security toggles, certificate/trust options |
| `ui/settings/notifications.png` | notifications | notification toggles, severity/event selectors |
| `ui/settings/update.png` | update settings | update channel, auto-check, frequency, download options |
| `ui/settings/update-latest.png` | latest update state | current/latest version, success/check status, release notes link |
| `ui/settings/advanced.png` | advanced | diagnostics, logs, developer options, reset/export buttons |

## Brand/icon assets

| PNG | Purpose | Notes |
|---|---|---|
| `ui/icons/narya-logo-raw-chroma.png` | raw logo reference | preserve gradient/chroma shape; hand-vector if real SVG is needed |
| `ui/icons/narya-logo-transparent-1080.png` | transparent logo | preserve alpha; use for high-res export only |
| `ui/icons/narya-logo-transparent-1920x1080.png` | wide transparent logo | likely marketing/splash composition reference |
| `ui/icons/narya-logo-transparent-2k-2048.png` | square 2k logo | app/icon source candidate |
| `ui/icons/narya-logo-transparent-2k-2560x1440.png` | wide 2k logo | background/splash source candidate |
| `ui/splash-background-2k.png` | splash background | should be implemented as image asset or carefully recreated gradient; not a component page |
