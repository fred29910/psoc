# PSOC UI 升级规划

## 项目概述

基于 `docs/design/dashboard.html` 设计文档和当前 UI 状态分析，制定 PSOC 图像编辑器界面升级计划。目标是实现现代化的 Office 风格菜单系统，提升用户体验，同时保持现有功能完整性。

## 当前状态分析

### 现有 UI 架构
- **框架**: Iced GUI 框架
- **主题**: 基础深色主题 (PsocTheme::Dark)
- **布局**: 传统工具栏式布局
  - `menu_bar()`: 基础菜单栏，使用 `localized_menu_bar()`
  - `toolbar()`: 工具栏，包含 `tool_palette()`
  - `left_panel()`: 左侧面板（工具和图层）
  - `canvas_area()`: 画布区域
  - `right_panel()`: 右侧属性面板

### 设计目标
- **主色调**: 深色背景 (#222222) + 科技蓝强调色 (#00BFFF)
- **菜单分类**: 文件/编辑/图像/图层/文字/选择/滤镜/视图/窗口/帮助
- **交互模式**: Office 风格下拉菜单系统
- **视觉效果**: 现代化磨砂玻璃效果、动画过渡

## 升级计划

### 阶段一：核心菜单系统重构

#### 1.1 新增菜单组件 (`src/ui/components/menu_system.rs`)
```rust
// 新增文件结构
pub struct DropdownMenu {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub is_open: bool,
    pub position: MenuPosition,
}

pub struct MenuItem {
    pub label: String,
    pub action: Message,
    pub icon: Option<Icon>,
    pub shortcut: Option<String>,
    pub submenu: Option<Vec<MenuItem>>,
    pub separator: bool,
}

pub enum MenuCategory {
    File,      // 文件
    Edit,      // 编辑  
    Image,     // 图像
    Layer,     // 图层
    Text,      // 文字
    Select,    // 选择
    Filter,    // 滤镜
    View,      // 视图
    Window,    // 窗口
    Help,      // 帮助
}
```

#### 1.2 菜单分类重组
- **文件 (File)**: 新建、打开、保存、导入、导出、最近文件
- **编辑 (Edit)**: 撤销、重做、复制、粘贴、变换、首选项
- **图像 (Image)**: 图像大小、画布大小、旋转、翻转、色彩模式
- **图层 (Layer)**: 新建图层、复制图层、删除图层、图层样式、混合模式
- **文字 (Text)**: 文字工具、字体、大小、样式、对齐
- **选择 (Select)**: 全选、取消选择、反选、羽化、边界
- **滤镜 (Filter)**: 模糊、锐化、噪点、艺术效果、扭曲
- **视图 (View)**: 缩放、标尺、网格、参考线、全屏
- **窗口 (Window)**: 面板管理、工作区、排列
- **帮助 (Help)**: 关于、帮助文档、快捷键

#### 1.3 主题系统增强 (`src/ui/theme.rs`)
```rust
// 扩展现有 ColorPalette
pub struct ColorPalette {
    // 现有字段...
    pub tech_blue: Color,        // #00BFFF
    pub dark_bg: Color,          // #222222  
    pub dark_panel: Color,       // #2a2a2e
    pub menu_hover: Color,       // 菜单悬停色
    pub menu_active: Color,      // 菜单激活色
    pub glass_bg: Color,         // 磨砂玻璃背景
}

// 新增菜单样式
pub enum MenuStyle {
    TopLevel,
    Dropdown,
    MenuItem,
    Separator,
}
```

### 阶段二：视觉效果升级

#### 2.1 现代化样式组件
- **磨砂玻璃效果**: 右侧面板背景模糊
- **动画过渡**: 菜单展开/收起动画
- **悬停效果**: 按钮和菜单项交互反馈
- **阴影系统**: 层次感增强

#### 2.2 图标系统优化 (`src/ui/icons.rs`)
- 统一图标风格
- 增加菜单分类图标
- 支持多尺寸图标

### 阶段三：交互体验优化

#### 3.1 键盘导航
- 菜单快捷键支持
- Tab 键导航
- 快捷键提示显示

#### 3.2 响应式布局
- 面板大小调整
- 最小化/最大化状态
- 自适应屏幕尺寸

### 阶段四：国际化支持

#### 4.1 菜单本地化 (`src/i18n/mod.rs`)
- 中英文菜单切换
- 动态语言加载
- 菜单文本翻译

## 实施细节

### 文件修改清单

#### 新增文件
1. `src/ui/components/menu_system.rs` - 下拉菜单系统核心
2. `src/ui/components/modern_menu.rs` - 现代化菜单组件
3. `src/ui/components/office_menu.rs` - Office 风格菜单实现
4. `src/ui/styles/menu_styles.rs` - 菜单样式定义
5. `src/ui/animations/menu_animations.rs` - 菜单动画系统
6. `tests/ui/menu_system_tests.rs` - 菜单系统单元测试
7. `tests/ui/menu_integration_tests.rs` - 菜单集成测试

#### 修改文件
1. `src/ui/application.rs` - 主应用界面集成新菜单系统
2. `src/ui/components.rs` - 组件模块导出更新
3. `src/ui/components/mod.rs` - 新增菜单组件模块
4. `src/ui/theme.rs` - 主题系统扩展（新增菜单样式）
5. `src/ui/icons.rs` - 图标系统增强（菜单图标）
6. `src/i18n/mod.rs` - 国际化支持（菜单文本）
7. `src/ui/mod.rs` - UI 模块导出更新

### 详细实施步骤

#### 步骤 1: 基础菜单组件创建

**目标**: 创建可复用的菜单组件基础架构

**文件**: `src/ui/components/menu_system.rs`
```rust
// 基础菜单数据结构
pub struct MenuSystem {
    pub categories: Vec<MenuCategory>,
    pub active_menu: Option<usize>,
    pub hover_item: Option<(usize, usize)>,
    pub animation_state: AnimationState,
}

// 菜单分类定义
pub struct MenuCategory {
    pub id: MenuCategoryId,
    pub title: String,
    pub items: Vec<MenuItem>,
    pub position: Point,
    pub is_open: bool,
}

// 菜单项定义
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub action: Option<Message>,
    pub icon: Option<Icon>,
    pub shortcut: Option<String>,
    pub submenu: Option<Vec<MenuItem>>,
    pub is_separator: bool,
    pub is_enabled: bool,
}
```

#### 步骤 2: 主题系统扩展

**目标**: 为新菜单系统添加样式支持

**文件**: `src/ui/theme.rs`
```rust
// 扩展 ColorPalette
impl ColorPalette {
    // 新增菜单相关颜色
    pub fn menu_background(&self) -> Color { /* 实现 */ }
    pub fn menu_hover(&self) -> Color { /* 实现 */ }
    pub fn menu_active(&self) -> Color { /* 实现 */ }
    pub fn menu_separator(&self) -> Color { /* 实现 */ }
}

// 新增菜单样式枚举
#[derive(Debug, Clone, Copy)]
pub enum MenuStyle {
    TopBar,
    Dropdown,
    MenuItem,
    Separator,
    Submenu,
}
```

#### 步骤 3: 应用程序集成

**目标**: 将新菜单系统集成到主应用中

**文件**: `src/ui/application.rs`
```rust
// 在 PsocApp 中添加菜单状态
pub struct PsocApp {
    // 现有字段...
    menu_system: MenuSystem,
    menu_animations: HashMap<MenuCategoryId, AnimationState>,
}

// 新增菜单消息处理
impl PsocApp {
    fn handle_menu_message(&mut self, message: MenuMessage) -> Task<Message> {
        match message {
            MenuMessage::OpenMenu(category) => {
                self.menu_system.open_menu(category);
                Task::none()
            },
            MenuMessage::CloseAllMenus => {
                self.menu_system.close_all();
                Task::none()
            },
            // 其他菜单消息处理...
        }
    }
}
```

### 测试策略

#### 单元测试计划

**菜单组件测试** (`tests/ui/menu_system_tests.rs`)
```rust
#[test]
fn test_menu_creation() {
    // 测试菜单创建和初始化
}

#[test]
fn test_menu_item_actions() {
    // 测试菜单项点击事件
}

#[test]
fn test_menu_keyboard_navigation() {
    // 测试键盘导航功能
}

#[test]
fn test_menu_state_management() {
    // 测试菜单状态管理
}
```

**主题测试** (`tests/ui/theme_tests.rs`)
```rust
#[test]
fn test_menu_theme_application() {
    // 测试菜单主题应用
}

#[test]
fn test_theme_switching() {
    // 测试主题切换对菜单的影响
}
```

**国际化测试** (`tests/ui/i18n_menu_tests.rs`)
```rust
#[test]
fn test_menu_localization() {
    // 测试菜单文本本地化
}

#[test]
fn test_language_switching() {
    // 测试语言切换功能
}
```

#### 集成测试计划

**完整工作流测试** (`tests/integration/menu_workflow_tests.rs`)
```rust
#[test]
fn test_file_menu_workflow() {
    // 测试文件菜单完整操作流程
}

#[test]
fn test_edit_menu_workflow() {
    // 测试编辑菜单完整操作流程
}
```

**性能测试** (`tests/integration/menu_performance_tests.rs`)
```rust
#[test]
fn test_menu_rendering_performance() {
    // 测试菜单渲染性能
}

#[test]
fn test_animation_performance() {
    // 测试动画性能
}
```

#### 视觉回归测试
- 界面截图对比测试
- 动画效果验证
- 跨平台兼容性测试
- 不同分辨率适配测试

## 风险评估与缓解

### 技术风险
- **Iced 框架限制**: 某些高级 UI 效果可能需要自定义实现
- **性能影响**: 复杂动画可能影响渲染性能
- **兼容性问题**: 不同操作系统的表现差异

### 缓解措施
- 渐进式升级，保持向后兼容
- 性能监控和优化
- 跨平台测试覆盖

## 详细时间规划

### 第 1-2 周：基础架构搭建
**里程碑**: 菜单系统基础框架完成

**具体任务**:
- [ ] 创建 `menu_system.rs` 基础组件
- [ ] 实现 `MenuCategory` 和 `MenuItem` 数据结构
- [ ] 扩展主题系统支持菜单样式
- [ ] 创建基础菜单渲染逻辑
- [ ] 编写单元测试框架

**交付物**:
- 可编译的菜单组件基础代码
- 基础单元测试套件
- 主题系统扩展

### 第 3 周：Office 风格菜单实现
**里程碑**: 下拉菜单系统完成

**具体任务**:
- [ ] 实现下拉菜单展开/收起逻辑
- [ ] 添加菜单项点击事件处理
- [ ] 实现菜单分类（文件、编辑等）
- [ ] 集成到主应用界面
- [ ] 键盘导航基础支持

**交付物**:
- 功能完整的下拉菜单系统
- 所有菜单分类实现
- 基础交互功能

### 第 4 周：视觉效果和动画
**里程碑**: 现代化视觉效果完成

**具体任务**:
- [ ] 实现菜单展开/收起动画
- [ ] 添加悬停效果和过渡动画
- [ ] 实现磨砂玻璃背景效果
- [ ] 优化菜单阴影和边框样式
- [ ] 响应式布局适配

**交付物**:
- 完整的动画系统
- 现代化视觉效果
- 响应式菜单布局

### 第 5 周：交互优化和国际化
**里程碑**: 用户体验优化完成

**具体任务**:
- [ ] 完善键盘导航功能
- [ ] 实现菜单快捷键支持
- [ ] 添加菜单文本国际化
- [ ] 优化菜单性能
- [ ] 跨平台兼容性测试

**交付物**:
- 完整的键盘导航系统
- 中英文菜单支持
- 性能优化版本

### 第 6 周：测试和优化
**里程碑**: 完整测试覆盖和性能优化

**具体任务**:
- [ ] 完善单元测试覆盖率
- [ ] 编写集成测试
- [ ] 性能基准测试
- [ ] 视觉回归测试
- [ ] 文档更新

**交付物**:
- 完整测试套件
- 性能报告
- 用户文档

**总计**: 6 周

## 验收标准和检查清单

### 功能验收标准

#### 菜单系统功能
- [ ] **下拉菜单展示**: 点击顶部菜单项能正确展开下拉菜单
- [ ] **菜单分类完整**: 包含所有 10 个菜单分类（文件/编辑/图像/图层/文字/选择/滤镜/视图/窗口/帮助）
- [ ] **菜单项功能**: 所有菜单项点击后能触发对应功能
- [ ] **子菜单支持**: 支持多级子菜单展开
- [ ] **菜单关闭**: 点击其他区域或按 ESC 键能关闭菜单

#### 交互体验标准
- [ ] **键盘导航**: 支持 Tab、方向键、Enter、ESC 键导航
- [ ] **快捷键显示**: 菜单项显示对应快捷键
- [ ] **悬停效果**: 鼠标悬停有明显视觉反馈
- [ ] **点击反馈**: 菜单项点击有视觉反馈
- [ ] **响应速度**: 菜单展开/收起响应时间 < 200ms

#### 视觉效果标准
- [ ] **主题一致性**: 菜单样式与整体深色主题一致
- [ ] **科技蓝强调**: 激活状态使用 #00BFFF 强调色
- [ ] **动画流畅**: 展开/收起动画流畅无卡顿
- [ ] **磨砂效果**: 右侧面板实现磨砂玻璃背景
- [ ] **阴影效果**: 下拉菜单有适当阴影层次

#### 国际化标准
- [ ] **中文支持**: 所有菜单文本支持中文显示
- [ ] **英文支持**: 所有菜单文本支持英文显示
- [ ] **语言切换**: 运行时可切换界面语言
- [ ] **文本适配**: 不同语言文本长度自适应

### 技术验收标准

#### 代码质量
- [ ] **测试覆盖率**: 单元测试覆盖率 ≥ 80%
- [ ] **集成测试**: 所有主要功能有集成测试
- [ ] **代码规范**: 通过 clippy 和 rustfmt 检查
- [ ] **文档完整**: 所有公共 API 有文档注释

#### 性能标准
- [ ] **启动时间**: 应用启动时间增加 < 10%
- [ ] **内存使用**: 内存使用增加 < 20%
- [ ] **渲染性能**: 菜单渲染帧率 ≥ 60fps
- [ ] **响应时间**: 菜单操作响应时间 < 100ms

#### 兼容性标准
- [ ] **Windows 兼容**: 在 Windows 10/11 正常运行
- [ ] **macOS 兼容**: 在 macOS 10.15+ 正常运行
- [ ] **Linux 兼容**: 在主流 Linux 发行版正常运行
- [ ] **高 DPI 支持**: 支持高 DPI 显示器

### 回归测试检查清单

#### 现有功能保持
- [ ] **文件操作**: 新建、打开、保存功能正常
- [ ] **编辑功能**: 撤销、重做、复制、粘贴功能正常
- [ ] **图像处理**: 所有图像调整和滤镜功能正常
- [ ] **图层管理**: 图层创建、删除、编辑功能正常
- [ ] **工具使用**: 所有绘图工具功能正常

#### 界面布局保持
- [ ] **工具面板**: 左侧工具面板布局和功能正常
- [ ] **图层面板**: 图层面板显示和操作正常
- [ ] **属性面板**: 右侧属性面板功能正常
- [ ] **画布区域**: 画布显示和交互正常
- [ ] **状态栏**: 状态栏信息显示正常

## 风险缓解措施

### 开发风险
- **技术难点**: 提前进行技术验证和原型开发
- **时间延期**: 采用敏捷开发，每周评估进度
- **质量问题**: 持续集成和自动化测试

### 用户体验风险
- **学习成本**: 保持菜单结构与用户习惯一致
- **功能缺失**: 确保所有现有功能在新菜单中可访问
- **性能下降**: 持续性能监控和优化

## 技术实现细节

### 菜单状态管理

```rust
// 在 PsocApp 中新增菜单状态
pub struct MenuState {
    pub active_menu: Option<MenuCategory>,
    pub menu_positions: HashMap<MenuCategory, (f32, f32)>,
    pub animation_states: HashMap<MenuCategory, AnimationState>,
}

// 菜单消息类型
#[derive(Debug, Clone)]
pub enum MenuMessage {
    OpenMenu(MenuCategory),
    CloseMenu,
    SelectItem(MenuItem),
    HoverItem(usize),
    KeyboardNavigation(keyboard::Key),
}
```

### 样式实现

```rust
// 菜单样式函数
impl MenuStyle {
    pub fn appearance(&self, theme: &PsocTheme) -> container::Style {
        let palette = theme.palette();
        match self {
            MenuStyle::TopLevel => container::Style {
                background: Some(palette.dark_bg.into()),
                border: Border::with_color(palette.border),
                ..Default::default()
            },
            MenuStyle::Dropdown => container::Style {
                background: Some(palette.glass_bg.into()),
                border: Border::with_radius(8.0),
                shadow: Shadow {
                    color: palette.shadow,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                ..Default::default()
            },
            // 其他样式...
        }
    }
}
```

### 动画系统

```rust
// 菜单动画状态
#[derive(Debug, Clone)]
pub enum AnimationState {
    Closed,
    Opening(f32), // 0.0 到 1.0 的进度
    Open,
    Closing(f32),
}

// 动画更新逻辑
impl PsocApp {
    fn update_menu_animations(&mut self) -> Task<Message> {
        // 实现菜单展开/收起动画
        // 使用 iced::time 模块进行时间管理
    }
}
```

## 测试实现

### 菜单系统测试示例

```rust
// tests/ui/menu_system_tests.rs
#[cfg(test)]
mod menu_tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let menu = DropdownMenu::new("File", vec![
            MenuItem::new("New", Message::NewDocument),
            MenuItem::new("Open", Message::OpenDocument),
        ]);
        assert_eq!(menu.title, "File");
        assert_eq!(menu.items.len(), 2);
    }

    #[test]
    fn test_menu_interaction() {
        let mut app = PsocApp::default();
        let message = MenuMessage::OpenMenu(MenuCategory::File);
        app.update(Message::Menu(message));
        assert!(app.menu_state.active_menu.is_some());
    }

    #[test]
    fn test_keyboard_navigation() {
        // 测试键盘导航功能
    }
}
```

## 性能优化策略

### 渲染优化
- 菜单项虚拟化（大量菜单项时）
- 动画帧率控制
- 不必要的重绘避免

### 内存管理
- 菜单状态缓存
- 图标资源复用
- 及时释放未使用的菜单实例

## 兼容性考虑

### 平台差异
- **Windows**: 原生菜单栏集成
- **macOS**: 系统菜单栏适配
- **Linux**: 桌面环境兼容

### 屏幕适配
- 高 DPI 显示支持
- 不同分辨率适配
- 触摸屏交互支持

## 后续优化

- 自定义主题支持
- 更多动画效果
- 插件菜单集成
- 用户自定义菜单布局
- 菜单搜索功能
- 最近使用项目快速访问
- 工作区模板系统
