# PSOC项目架构设计文档

## 📋 文档概述

本文档详细描述了PSOC（Photoshop-like image editor）项目的整体架构设计、代码结构和模块关系。PSOC是一个用Rust构建的专业级图像编辑器，采用现代化的软件架构和设计模式。

## 🏗️ 整体架构

### 架构原则
- **模块化设计：** 清晰的模块分离和职责划分
- **类型安全：** 利用Rust类型系统保证内存和线程安全
- **可扩展性：** 支持插件系统和功能扩展
- **高性能：** 并行处理和缓存优化
- **跨平台：** 原生跨平台支持

### 架构层次
```
┌─────────────────────────────────────────────────────────────┐
│                    用户界面层 (UI Layer)                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   应用程序   │ │    画布     │ │   对话框    │ │  主题   │ │
│  │ Application │ │   Canvas    │ │  Dialogs    │ │ Theme   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                   业务逻辑层 (Business Layer)                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   工具系统   │ │   命令系统   │ │   快捷键    │ │  插件   │ │
│  │    Tools    │ │  Commands   │ │ Shortcuts   │ │Plugins  │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                    核心层 (Core Layer)                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   文档模型   │ │   渲染引擎   │ │  图像处理   │ │ 文件IO  │ │
│  │  Document   │ │  Rendering  │ │Image Process│ │File I/O │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                   基础设施层 (Infrastructure)                 │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   数据结构   │ │   几何计算   │ │   颜色管理   │ │  工具   │ │
│  │    Data     │ │  Geometry   │ │    Color    │ │ Utils   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 📦 项目结构

### Workspace组织
```
psoc/
├── Cargo.toml                 # 主项目配置
├── src/                       # 主应用程序代码
├── crates/                    # 子crate库
│   ├── psoc-core/            # 核心数据结构和算法
│   ├── psoc-image-processing/ # 图像处理算法
│   ├── psoc-file-formats/     # 文件格式支持
│   ├── psoc-ui-toolkit/       # UI组件库
│   └── psoc-plugins/          # 插件系统
├── tests/                     # 集成测试
├── examples/                  # 示例代码
├── docs/                      # 项目文档
├── scripts/                   # 构建和打包脚本
└── resources/                 # 资源文件
```

### 主应用程序结构
```
src/
├── main.rs                    # 程序入口点
├── lib.rs                     # 库入口点
├── app/                       # 应用程序核心
├── ui/                        # 用户界面
│   ├── application.rs         # 主应用程序
│   ├── canvas.rs             # 画布组件
│   ├── dialogs/              # 对话框
│   ├── components.rs         # UI组件
│   ├── icons.rs              # 图标系统
│   └── theme.rs              # 主题系统
├── tools/                     # 工具系统
│   ├── tool_trait.rs         # 工具接口
│   ├── tool_manager.rs       # 工具管理器
│   └── tools.rs              # 具体工具实现
├── commands/                  # 命令系统
├── shortcuts/                 # 快捷键系统
├── rendering/                 # 渲染模块
├── file_io/                   # 文件IO
├── image_processing/          # 图像处理
├── preferences/               # 用户偏好
├── i18n/                      # 国际化
├── plugins/                   # 插件接口
└── utils/                     # 工具函数
```

## 🔧 核心模块设计

### 1. 文档模型 (Document Model)

**位置：** `crates/psoc-core/src/document.rs`

**核心数据结构：**
```rust
pub struct Document {
    pub id: Uuid,                    // 文档唯一标识
    pub metadata: DocumentMetadata,  // 文档元数据
    pub size: Size,                  // 文档尺寸
    pub resolution: Resolution,      // 分辨率
    pub color_mode: ColorMode,       // 颜色模式
    pub color_space: DocumentColorSpace, // 颜色空间
    pub icc_profile: Option<IccProfile>, // ICC配置文件
    pub background_color: RgbaPixel, // 背景色
    pub layers: Vec<Layer>,          // 图层列表
    pub active_layer_index: Option<usize>, // 活动图层
    pub canvas_bounds: Rect,         // 画布边界
    pub selection: Selection,        // 当前选区
    pub command_history: CommandHistory, // 命令历史
}
```

**设计特点：**
- 支持多图层结构
- 完整的颜色管理
- 撤销/重做系统
- 选区管理
- 元数据支持

### 2. 图层系统 (Layer System)

**位置：** `crates/psoc-core/src/layer.rs`

**图层类型：**
```rust
pub enum LayerType {
    Pixel,                          // 像素图层
    Adjustment(AdjustmentType),     // 调整图层
    SmartObject(SmartObjectContent), // 智能对象
    Text(TextData),                 // 文本图层
    Shape(ShapeData),               // 形状图层
    Group,                          // 图层组
}
```

**图层结构：**
```rust
pub struct Layer {
    pub id: Uuid,                   // 图层ID
    pub name: String,               // 图层名称
    pub layer_type: LayerType,      // 图层类型
    pub pixel_data: Option<PixelData>, // 像素数据
    pub visible: bool,              // 可见性
    pub opacity: f32,               // 不透明度
    pub blend_mode: BlendMode,      // 混合模式
    pub offset: Point,              // 位置偏移
    pub transform: Transform,       // 变换矩阵
    pub bounds: Rect,               // 图层边界
    pub locked: bool,               // 锁定状态
    pub mask: Option<PixelData>,    // 图层蒙版
}
```

**设计特点：**
- 支持16种混合模式
- 非破坏性编辑
- 智能对象支持
- 图层蒙版
- 变换系统

### 3. 工具系统 (Tool System)

**位置：** `src/tools/`

**工具接口：**
```rust
pub trait Tool: Debug + Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn activate(&mut self) -> ToolResult<()>;
    fn deactivate(&mut self) -> ToolResult<()>;
    fn handle_event(&mut self, event: ToolEvent, document: &mut Document, state: &mut ToolState) -> ToolResult<()>;
    fn cursor(&self) -> ToolCursor;
    fn options(&self) -> Vec<ToolOption>;
    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()>;
    fn get_option(&self, name: &str) -> Option<ToolOptionValue>;
}
```

**工具类型：**
- **选择工具：** Select, EllipseSelect, LassoSelect, MagicWand
- **绘画工具：** Brush, Eraser
- **编辑工具：** Move, Transform, Crop
- **形状工具：** Rectangle, Ellipse, Line, Polygon
- **其他工具：** Text, Gradient, Eyedropper

**设计特点：**
- 统一的工具接口
- 事件驱动架构
- 动态选项配置
- 状态管理

### 4. 命令系统 (Command System)

**位置：** `src/commands/` 和 `crates/psoc-core/src/command.rs`

**命令接口：**
```rust
pub trait Command: Debug + Send + Sync {
    fn id(&self) -> Uuid;
    fn description(&self) -> &str;
    fn execute(&self, document: &mut Document) -> Result<()>;
    fn undo(&self, document: &mut Document) -> Result<()>;
    fn timestamp(&self) -> SystemTime;
    fn modifies_document(&self) -> bool;
}
```

**命令类型：**
- **图层命令：** AddLayer, DeleteLayer, MoveLayer
- **绘画命令：** BrushStroke, EraseStroke
- **调整命令：** BrightnessContrast, HSLAdjustment
- **变换命令：** Scale, Rotate, Translate
- **选择命令：** CreateSelection, ModifySelection
- **智能对象命令：** CreateSmartObject, UpdateSmartObject

**设计特点：**
- 完整的撤销/重做支持
- 命令组合和批处理
- 历史记录管理
- 内存优化

### 5. 渲染引擎 (Rendering Engine)

**位置：** `crates/psoc-core/src/rendering.rs`

**渲染器结构：**
```rust
pub struct RenderEngine {
    parallel_enabled: bool,              // 并行处理
    tile_size: u32,                     // 瓦片大小
    adjustment_registry: AdjustmentRegistry, // 调整注册表
    smart_object_manager: SmartObjectManager, // 智能对象管理器
}
```

**渲染流程：**
1. **图层收集：** 收集可见图层
2. **预处理：** 应用变换和调整
3. **合成：** 按混合模式合成
4. **后处理：** 应用滤镜和效果
5. **输出：** 生成最终图像

**设计特点：**
- 并行渲染支持
- 智能缓存机制
- 16种混合模式
- 高质量插值

### 6. 用户界面 (User Interface)

**位置：** `src/ui/`

**主应用程序：**
```rust
pub struct PsocApp {
    state: AppState,                    // 应用状态
    document: Option<Document>,         // 当前文档
    tool_manager: ToolManager,          // 工具管理器
    theme: PsocTheme,                   // 主题
    dialogs: DialogManager,             // 对话框管理器
}
```

**UI组件：**
- **菜单栏：** 文件、编辑、图像、图层、选择、滤镜等
- **工具栏：** 工具选择和快速操作
- **画布：** 图像显示和编辑区域
- **面板：** 图层、工具选项、颜色、历史记录等
- **状态栏：** 状态信息和进度显示

**设计特点：**
- 基于iced GUI框架
- 响应式设计
- 主题系统
- 国际化支持

## 🔄 模块关系和数据流

### 核心数据流
```
用户输入 → UI事件 → 工具处理 → 命令生成 → 文档修改 → 渲染更新 → UI显示
```

### 模块依赖关系
```
UI Layer
├── 依赖 Business Layer
├── 依赖 Core Layer (间接)
└── 依赖 Infrastructure (间接)

Business Layer
├── 依赖 Core Layer
└── 依赖 Infrastructure

Core Layer
└── 依赖 Infrastructure

Infrastructure
└── 无外部依赖
```

### 关键接口
1. **Tool ↔ Document：** 工具通过命令修改文档
2. **Command ↔ Document：** 命令直接操作文档数据
3. **RenderEngine ↔ Document：** 渲染引擎读取文档生成图像
4. **UI ↔ Tool：** UI将事件传递给工具，获取工具状态
5. **UI ↔ Document：** UI显示文档内容，响应文档变化

## 🎯 设计模式应用

### 1. 命令模式 (Command Pattern)
- **应用：** 撤销/重做系统
- **实现：** Command trait和CommandHistory
- **优势：** 操作可逆、批处理、宏录制

### 2. 策略模式 (Strategy Pattern)
- **应用：** 工具系统、调整系统
- **实现：** Tool trait、Adjustment trait
- **优势：** 算法可替换、易于扩展

### 3. 观察者模式 (Observer Pattern)
- **应用：** UI事件处理
- **实现：** iced消息系统
- **优势：** 松耦合、事件驱动

### 4. 工厂模式 (Factory Pattern)
- **应用：** 图层创建、工具创建
- **实现：** LayerFactory、ToolManager
- **优势：** 对象创建统一管理

### 5. 单例模式 (Singleton Pattern)
- **应用：** 应用程序状态、配置管理
- **实现：** 全局状态管理
- **优势：** 状态一致性

## 🚀 性能优化设计

### 1. 并行处理
- **图像处理：** 使用rayon进行并行像素处理
- **渲染：** 瓦片化并行渲染
- **文件IO：** 异步文件操作

### 2. 缓存机制
- **智能对象：** LRU缓存策略
- **渲染结果：** 图层渲染缓存
- **调整预览：** 实时预览缓存

### 3. 内存管理
- **零拷贝：** 尽可能避免数据拷贝
- **引用计数：** 共享数据使用Arc
- **内存池：** 大对象内存复用

### 4. 算法优化
- **SIMD：** 向量化图像处理
- **查找表：** 预计算常用函数
- **空间索引：** 快速碰撞检测

## 🔧 扩展性设计

### 1. 插件系统
- **接口：** 标准化插件API
- **加载：** 动态库加载
- **沙箱：** WASM安全执行环境
- **脚本：** Lua脚本支持

### 2. 文件格式
- **模块化：** 独立的格式处理crate
- **可扩展：** 新格式易于添加
- **标准化：** 统一的格式接口

### 3. 工具扩展
- **插件工具：** 第三方工具支持
- **自定义工具：** 用户自定义工具
- **工具组合：** 复合工具支持

## 📋 总结

PSOC项目采用了现代化的软件架构设计，具有以下特点：

### 技术优势
- **类型安全：** Rust类型系统保证内存和线程安全
- **高性能：** 并行处理和缓存优化
- **模块化：** 清晰的模块分离和职责划分
- **可扩展：** 插件系统和标准化接口

### 架构优势
- **分层设计：** 清晰的架构层次
- **松耦合：** 模块间依赖最小化
- **高内聚：** 模块内功能紧密相关
- **可测试：** 完整的测试覆盖

### 维护优势
- **代码质量：** 严格的代码规范
- **文档完整：** 详细的设计和API文档
- **测试覆盖：** 95%+的测试覆盖率
- **持续集成：** 自动化测试和构建

PSOC项目的架构设计为构建专业级图像编辑软件提供了坚实的技术基础，同时保持了良好的可维护性和可扩展性。

## 🔍 详细模块分析

### Crate依赖关系

#### psoc-core (核心库)
**职责：** 提供基础数据结构和算法
**主要模块：**
- `document.rs` - 文档数据结构
- `layer.rs` - 图层系统
- `pixel.rs` - 像素数据处理
- `geometry.rs` - 几何计算
- `color.rs` - 颜色管理
- `command.rs` - 命令系统
- `rendering.rs` - 渲染引擎
- `smart_object.rs` - 智能对象
- `selection.rs` - 选区系统
- `adjustment.rs` - 调整系统

**依赖：** 仅依赖基础库（serde, uuid, anyhow等）

#### psoc-image-processing (图像处理库)
**职责：** 图像处理算法实现
**主要功能：**
- 滤镜算法（高斯模糊、锐化、噪点等）
- 变换算法（缩放、旋转、透视等）
- 颜色调整算法（亮度、对比度、HSL等）
- 选区算法（魔棒、边缘检测等）

**依赖：** psoc-core, image, rayon

#### psoc-file-formats (文件格式库)
**职责：** 文件格式支持
**支持格式：**
- PNG - 完整支持，ICC配置文件
- JPEG - 完整支持，EXIF数据
- TIFF - 基础支持
- WebP - 基础支持
- .psoc - 项目文件格式（RON）

**依赖：** psoc-core, image, serde

#### psoc-ui-toolkit (UI组件库)
**职责：** 可复用UI组件
**主要组件：**
- 颜色选择器
- 滑块控件
- 数值输入框
- 图层面板
- 工具选项面板

**依赖：** iced, psoc-core

#### psoc-plugins (插件系统)
**职责：** 插件架构和脚本支持
**功能：**
- 插件API定义
- 动态库加载
- Lua脚本引擎
- WASM运行时
- 插件管理器

**依赖：** psoc-core, mlua, wasmtime

### 核心算法实现

#### 图层合成算法
```rust
// 简化的图层合成流程
fn composite_layers(layers: &[Layer], bounds: Rect) -> PixelData {
    let mut result = PixelData::new(bounds.width(), bounds.height());

    for layer in layers.iter().filter(|l| l.visible) {
        let layer_data = render_layer(layer, bounds);
        blend_layer(&mut result, &layer_data, layer.blend_mode, layer.opacity);
    }

    result
}
```

#### 智能对象缓存
```rust
// LRU缓存实现
pub struct SmartObjectManager {
    cache: LruCache<ContentHash, RenderedContent>,
    max_cache_size: usize,
}

impl SmartObjectManager {
    fn get_rendered_content(&mut self, content: &SmartObjectContent) -> &RenderedContent {
        let hash = content.hash();

        if !self.cache.contains(&hash) {
            let rendered = self.render_content(content);
            self.cache.put(hash, rendered);
        }

        self.cache.get(&hash).unwrap()
    }
}
```

#### 并行渲染
```rust
// 瓦片化并行渲染
fn render_parallel(document: &Document, bounds: Rect) -> PixelData {
    let tile_size = 256;
    let tiles: Vec<_> = bounds.tiles(tile_size).collect();

    let rendered_tiles: Vec<_> = tiles
        .par_iter()
        .map(|tile_bounds| render_tile(document, *tile_bounds))
        .collect();

    combine_tiles(rendered_tiles, bounds)
}
```

## 🎨 用户界面架构

### Iced应用程序结构
```rust
// 主应用程序状态
pub struct PsocApp {
    // 核心状态
    state: AppState,
    document: Option<Document>,
    tool_manager: ToolManager,

    // UI状态
    theme: PsocTheme,
    canvas_state: CanvasState,
    panel_states: PanelStates,

    // 对话框
    dialogs: DialogManager,

    // 国际化
    i18n: I18nManager,
}

// 消息系统
#[derive(Debug, Clone)]
pub enum Message {
    // 文件操作
    FileNew,
    FileOpen,
    FileSave,

    // 编辑操作
    Undo,
    Redo,
    Copy,
    Paste,

    // 工具操作
    ToolSelected(ToolType),
    ToolOption(ToolOptionMessage),

    // 图层操作
    Layer(LayerMessage),

    // 画布操作
    Canvas(CanvasMessage),

    // 对话框
    Dialog(DialogMessage),
}
```

### 响应式布局
```rust
fn view(&self) -> Element<Message> {
    column![
        self.menu_bar(),           // 菜单栏
        self.toolbar(),            // 工具栏
        row![
            self.tool_panel(),     // 工具面板
            self.canvas_area(),    // 画布区域
            self.properties_panel(), // 属性面板
        ].spacing(4),
        self.status_bar(),         // 状态栏
    ].spacing(0)
}
```

## 🔧 工具系统详细设计

### 工具生命周期
1. **注册：** 工具在ToolManager中注册
2. **激活：** 用户选择工具，调用activate()
3. **事件处理：** 处理鼠标、键盘事件
4. **选项更新：** 动态更新工具选项
5. **停用：** 切换到其他工具，调用deactivate()

### 工具事件流
```rust
// 事件处理流程
UI Event → ToolManager → Active Tool → Command Generation → Document Update → UI Refresh
```

### 工具选项系统
```rust
// 工具选项定义
pub struct ToolOption {
    pub name: String,              // 选项名称
    pub display_name: String,      // 显示名称
    pub description: String,       // 描述
    pub option_type: ToolOptionType, // 选项类型
    pub default_value: ToolOptionValue, // 默认值
}

// 选项类型
pub enum ToolOptionType {
    Bool,                          // 布尔值
    Int { min: i32, max: i32 },   // 整数范围
    Float { min: f32, max: f32 }, // 浮点数范围
    Color,                         // 颜色
    Enum(Vec<String>),            // 枚举
    String,                        // 字符串
}
```

## 📊 性能监控和优化

### 性能指标
- **启动时间：** < 2秒
- **文件加载：** 大文件 < 5秒
- **工具响应：** < 100ms
- **渲染帧率：** 60fps
- **内存使用：** < 100MB基础占用

### 优化策略
1. **预加载：** 常用资源预加载
2. **懒加载：** 按需加载功能模块
3. **缓存：** 多级缓存策略
4. **并行：** CPU密集任务并行化
5. **SIMD：** 向量化图像处理

### 内存管理
```rust
// 智能指针使用
Arc<Document>          // 文档共享
Rc<RefCell<Layer>>     // 图层可变借用
Box<dyn Tool>          // 工具多态
Cow<'_, str>           // 字符串优化
```

## 🌐 国际化设计

### 多语言支持
- **框架：** Fluent国际化框架
- **语言：** 英文、简体中文
- **资源：** .ftl文件格式
- **动态切换：** 运行时语言切换

### 本地化内容
- UI文本翻译
- 错误消息本地化
- 日期时间格式
- 数字格式
- 文化相关功能

## 🔒 安全性设计

### 内存安全
- **Rust保证：** 无空指针、无缓冲区溢出
- **借用检查：** 编译时内存安全
- **线程安全：** Send/Sync trait保证

### 文件安全
- **路径验证：** 防止路径遍历攻击
- **格式验证：** 严格的文件格式检查
- **大小限制：** 防止内存耗尽攻击
- **权限检查：** 文件访问权限验证

### 插件安全
- **沙箱：** WASM安全执行环境
- **权限控制：** 细粒度权限管理
- **API限制：** 受限的插件API
- **代码签名：** 插件完整性验证

## 📈 可扩展性实现

### 插件架构
```rust
// 插件接口
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, api: &PluginApi) -> Result<()>;
    fn execute(&self, context: &PluginContext) -> Result<PluginResult>;
}

// 插件API
pub struct PluginApi {
    document_api: DocumentApi,
    tool_api: ToolApi,
    ui_api: UiApi,
}
```

### 功能扩展点
1. **工具扩展：** 自定义工具插件
2. **滤镜扩展：** 自定义滤镜算法
3. **格式扩展：** 新文件格式支持
4. **UI扩展：** 自定义面板和对话框
5. **脚本扩展：** Lua脚本自动化

## 🧪 测试架构

### 测试分层
1. **单元测试：** 模块级功能测试
2. **集成测试：** 模块间协作测试
3. **端到端测试：** 完整功能流程测试
4. **性能测试：** 基准测试和压力测试
5. **UI测试：** 用户界面交互测试

### 测试工具
- **Rust内置：** 标准测试框架
- **Criterion：** 性能基准测试
- **Proptest：** 属性测试
- **Mock：** 依赖模拟
- **Coverage：** 代码覆盖率

## 📋 开发工作流

### 代码质量保证
1. **静态分析：** Clippy检查
2. **格式化：** rustfmt自动格式化
3. **文档：** 强制API文档
4. **测试：** 95%+覆盖率要求
5. **审查：** 代码审查流程

### 持续集成
1. **构建：** 多平台自动构建
2. **测试：** 自动化测试执行
3. **质量：** 代码质量检查
4. **打包：** 自动化打包发布
5. **部署：** 自动化部署流程

这个架构设计确保了PSOC项目的技术先进性、代码质量和长期可维护性，为构建世界级的图像编辑软件奠定了坚实基础。
