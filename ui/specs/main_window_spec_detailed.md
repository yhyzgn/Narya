# Narya Main Window – Detailed UI Spec (Engineering Grade)

---

# 0. Design System Foundation

## Spacing Scale (8px grid)
- xs: 4px
- sm: 8px
- md: 12px
- lg: 16px
- xl: 20px
- xxl: 24px

## Radius
- sm: 6px
- md: 8px
- lg: 10px

## Colors
- Background: #F5F7FB
- Surface: #FFFFFF
- Border: #E5E7EB
- Text Primary: #111827
- Text Secondary: #6B7280
- Brand: #3B82F6
- Brand Soft: #E8F0FF → #EEF2FF

## Typography
- Title: 18px / 600
- Subtitle: 13px / 400
- Body: 14px / 400
- Small: 12px / 400

---

# 1. Root Layout Tree

AppWindow
├── TitleBar (48px)
├── Body (flex row)
│   ├── Sidebar (220px fixed)
│   └── Main (flex column)
│       ├── Header (64px)
│       └── Content (flex 1)
└── Footer (36px)

---

# 2. Window Frame

## Container
- display: flex
- flex-direction: column
- border-radius: 10px
- overflow: hidden
- box-shadow: 0 8px 24px rgba(0,0,0,0.08)
- background: #FFFFFF

---

# 3. TitleBar

## Layout
- height: 48px
- display: flex
- align-items: center
- justify-content: space-between
- padding: 0 16px
- border-bottom: 1px solid #E5E7EB

## Left Section
- display: flex
- align-items: center
- gap: 8px

Elements:
1. App Icon
   - size: 24x24

2. App Name
   - font-size: 14px
   - font-weight: 500

3. Version
   - font-size: 12px
   - color: #6B7280

## Right Section (Window Controls)
Buttons:
- size: 32x32
- hover: background #F3F4F6
- icons: minimize / maximize / close

---

# 4. Sidebar

## Container
- width: 220px
- display: flex
- flex-direction: column
- justify-content: space-between
- background: #FFFFFF
- border-right: 1px solid #E5E7EB

---

## 4.1 Header (Brand Area)
- height: 60px
- padding: 0 20px
- display: flex
- align-items: center
- gap: 10px

Logo:
- 32x32

Text:
- title: 16px bold
- subtitle: 12px secondary

---

## 4.2 Menu List

### Container
- padding: 8px
- display: flex
- flex-direction: column
- gap: 4px

### Menu Item
- height: 44px
- display: flex
- align-items: center
- gap: 10px
- padding: 0 12px
- border-radius: 8px
- cursor: pointer

### Icon
- size: 18px

### States

#### Default
- background: transparent

#### Hover
- background: #F3F4F6

#### Active
- background: linear-gradient(90deg, #E8F0FF, #EEF2FF)
- color: #3B82F6
- font-weight: 500

---

## 4.3 Footer Area

### Status Card
- margin: 12px
- padding: 12px
- border-radius: 10px
- background: #F9FAFB
- border: 1px solid #E5E7EB

#### Layout
vertical stack, gap 8px

#### Content
- Status Row (icon + 已连接)
- Node Name (香港 HK01)
- Latency Badge (48ms)
- Metrics Row:
  - ↓ Download
  - ↑ Upload
- Mini Chart
  - height: 40px

---

## 4.4 Bottom Actions

- padding: 8px 12px
- display: flex
- gap: 12px

Icons:
- GitHub
- Theme toggle
- Notification (with badge)

---

# 5. Main Area

## Container
- flex: 1
- display: flex
- flex-direction: column
- background: #F5F7FB

---

# 5.1 Header

## Layout
- height: 64px
- display: flex
- align-items: center
- justify-content: space-between
- padding: 0 20px
- background: #FFFFFF
- border-bottom: 1px solid #E5E7EB

---

## Left Section

- display: flex
- flex-direction: column
- gap: 4px

Title:
- font-size: 18px
- font-weight: 600

Subtitle:
- font-size: 13px
- color: #6B7280

---

## Right Section (Toolbar)

- display: flex
- gap: 12px

### Button Style
- height: 32px
- padding: 0 10px
- display: flex
- align-items: center
- gap: 6px
- border-radius: 6px
- font-size: 13px

#### Types
1. Ghost Button
   - background: transparent
   - hover: #F3F4F6

2. Icon Button
   - size: 32x32
   - center aligned

---

## Actions
- 添加
- 刷新全部
- 导入
- 导出
- 设置
- 更多

---

# 5.2 Content (Empty Placeholder)

- flex: 1
- padding: 16px 20px
- overflow: auto

---

# 6. Footer

## Layout
- height: 36px
- display: flex
- align-items: center
- justify-content: space-between
- padding: 0 16px
- border-top: 1px solid #E5E7EB
- background: #FFFFFF

---

## Left Section
- display: flex
- gap: 16px

Items:
- 内核: sing-box
- 配置: Narya Default
- 订阅: 机场 A · 128 节点

---

## Right Section
- display: flex
- gap: 12px

Items:
- 检查更新 (link style)
- Version: 1.0.0

---

# 7. Interaction Rules

## Hover
- All clickable elements must have hover feedback (#F3F4F6)

## Active
- Sidebar item uses gradient highlight

## Cursor
- pointer for all interactive elements

---

# 8. Implementation Notes (Important)

- Use Flexbox only (no absolute unless necessary)
- Follow 8px spacing grid strictly
- Avoid hardcoded pixel chaos
- Keep layout responsive above 1100px
- Sidebar MUST be fixed width
- Header/Footer MUST NOT scroll
- Content area scrolls independently

---

# 9. Gemini CLI Prompt (High Fidelity)

Use this to generate layout:

Build a desktop SaaS dashboard shell with:

- Top title bar (48px)
- Left sidebar (220px fixed)
- Main header (64px)
- Bottom footer (36px)

Use flex layout only.

Sidebar:
- vertical menu with active highlight (light blue gradient)
- bottom status card with metrics and chart

Header:
- title + subtitle (left)
- action buttons (right)

Footer:
- system info (left)
- version + update (right)

Style:
- modern, minimal
- soft gray background
- white surfaces
- subtle borders
- rounded corners
