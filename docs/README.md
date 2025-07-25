基于Rust技术栈，这里是一个开发类似Photoshop软件的任务列表，包含一个相对完备的开发流程。这将是一个非常庞大的项目，所以列表会按阶段划分，每个阶段都有其核心目标。

**核心理念：**

*   **迭代开发 (Agile/Scrum-like)：** 将大功能分解为小任务，定期交付可用版本。
*   **测试驱动 (TDD/BDD where applicable)：** 尤其是核心算法和数据结构。
*   **模块化设计：** 利用Rust的模块系统和Trait系统实现高内聚低耦合。
*   **性能优先：** Rust的优势之一，在图像处理中至关重要。

---

**开发流程概览**

1.  **阶段 0: 项目奠基与规划 (Foundation & Planning)**
2.  **阶段 1: 核心引擎与MVP (Minimum Viable Product)**
3.  **阶段 2: 基础图层系统 (Basic Layer System)**
4.  **阶段 3: 核心编辑工具 (Essential Editing Tools)**
5.  **阶段 4: 基础图像调整与滤镜 (Basic Adjustments & Filters)**
6.  **阶段 5: UI/UX 完善与进阶核心功能 (UI/UX Refinement & Advanced Core)**
7.  **阶段 6: 高级工具与功能 (Advanced Tools & Features)**
8.  **阶段 7: 非破坏性编辑探索 (Non-Destructive Editing Exploration - Highly Advanced)**
9.  **阶段 8: 扩展性、优化与发布准备 (Extensibility, Optimization & Release Prep)**

---

**详细任务列表**

**阶段 0: 项目奠基与规划**

*   **P0.1: 需求分析与范围定义**
    *   [ ] 明确项目的长期目标和短期目标（例如，第一版实现哪些功能）。
    *   [ ] 确定目标用户群体和核心用例。
    *   [ ] 调研竞品（Photoshop, GIMP, Krita, Affinity Photo）的核心特性。
*   **P0.2: 技术选型最终确认**
    *   [ ] GUI框架选型 (`iced`, `egui`, `gtk-rs`, `Tauri` - 做出最终决定，或确定组合策略)。
    *   [ ] 渲染后端确认 (e.g., `wgpu` via `iced`, or `tiny-skia` on `wgpu`).
    *   [ ] 关键图像处理库确认 (`image`, `imageproc`, `ndarray`).
*   **P0.3: 开发环境与工具链搭建**
    *   [x] 安装最新稳定版Rust工具链 (rustc, cargo)。
    *   [x] 配置IDE (VS Code with rust-analyzer, IntelliJ Rust, etc.)。
    *   [x] 初始化Git仓库，制定分支策略 (e.g., Gitflow)。
    *   [x] 搭建基础CI/CD流程 (e.g., GitHub Actions for automated builds and tests)。
*   **P0.4: 项目结构设计**
    *   [x] `Cargo.toml` 初始化，添加基础依赖。
    *   [x] 定义初步的模块结构 (e.g., `core_logic`, `ui`, `rendering`, `file_io`, `image_processing`).
*   **P0.5: 制定编码规范与代码审查流程**
    *   [x] 统一代码格式化 (`rustfmt`).
    *   [x] 静态分析工具 (`clippy`).
    *   [x] 制定代码审查清单和流程。
*   **P0.6: 基础错误处理和日志系统**
    *   [x] 选择错误处理库 (e.g., `anyhow`, `thiserror`).
    *   [x] 集成日志库 (e.g., `log`, `tracing` with `env_logger` or `tracing-subscriber`).

---

**阶段 1: 核心引擎与MVP (Minimum Viable Product)**

*   **P1.1: 应用程序窗口与主框架**
    *   [x] 使用选定的GUI框架创建基本窗口 (`iced`)。
    *   [x] 设计基本布局 (菜单栏、工具栏区域、主画布区域、图层面板区域、属性面板区域)。
*   **P1.2: 画布 (Canvas) 实现** ✅ **已完成**
    *   [x] 实现一个可交互的2D画布组件。
    *   [x] 图像数据显示：将`image::DynamicImage` 或 `ndarray` 数据渲染到画布。
    *   [x] 基础画布交互：缩放 (Zoom) 和平移 (Pan)。
    *   [x] 渲染引擎集成 (`wgpu` / `tiny-skia` / GUI框架内置渲染)。
    *   [x] 鼠标事件处理：点击、移动、滚轮缩放。
    *   [x] 状态同步：应用程序状态与画布状态的双向同步。
*   **P1.3: 图像文件IO** ✅ **已完成**
    *   [x] 使用 `image` crate 实现图像加载 (PNG, JPEG)。
    *   [x] 实现图像保存 (PNG, JPEG)。
    *   [x] 文件对话框集成 (使用 `rfd` - Rustic File Dialog)。
*   **P1.4: 核心数据结构设计** ✅ **已完成**
    *   [x] 定义项目文档结构 (e.g., 包含图像尺寸、分辨率、图层列表等)。
    *   [x] 定义图像像素数据表示 (e.g., `Vec<u8>`, `ndarray::Array3<u8>`)。
    *   [x] 实现完整的颜色管理系统 (RGB, HSL, HSV, 颜色调整)。
    *   [x] 实现几何计算工具 (点、矩形、变换矩阵)。
    *   [x] 实现图层系统 (像素图层、文本图层、调整图层、混合模式)。
    *   [x] 实现数学工具库 (插值、高斯函数、角度转换等)。
*   **P1.5: 简单的“关于”对话框**
*   **P1.6: 单元测试与集成测试 (基础)** ✅ **已完成**
    *   [x] 为核心数据结构和文件IO编写单元测试。
    *   [x] 实现完整的UI集成测试框架。
    *   [x] 建立性能基准测试。
    *   [x] 47个测试，100%通过率。
*   **MVP目标：** ✅ **已完成** - 用户可以打开一张图片，在画布上看到它，可以缩放平移，然后保存图片。

## 🎉 MVP 完成状态

PSOC 项目已成功完成 MVP 目标！用户现在可以：

- ✅ **打开图像**: 支持 PNG、JPEG 格式
- ✅ **画布显示**: 在交互式画布中查看图像
- ✅ **缩放操作**: 放大、缩小、重置缩放 (10%-1000%)
- ✅ **平移操作**: 鼠标拖拽和滚轮平移
- ✅ **保存图像**: 支持另存为功能
- ✅ **流畅交互**: 实时响应的用户界面

**测试状态**: 47个测试全部通过 (22个单元测试 + 25个集成测试)
**代码质量**: 零编译错误，符合 Rust 最佳实践

## 当前项目状态

- ✅ **P0: 项目初始化** - 完成
- ✅ **P1: 基础架构** - 完成
- ✅ **P2: 图层系统** - 完成
- ✅ **P3: 工具系统** - 完成
  - ✅ P3.1: 工具抽象与管理 - 完成
  - ✅ P3.2: 选区工具实现 - 完成
  - ✅ P3.3: 画笔工具实现 - 完成
  - ✅ P3.4: 橡皮擦工具实现 - 完成
  - ✅ P3.5: 移动工具实现 - 完成
  - ✅ P3.6: 撤销/重做系统架构 - 完成
- ✅ **P4: 调整和滤镜系统** - 完成
  - ✅ P4.1: 调整/滤镜框架 - 完成
  - ✅ P4.2: 亮度/对比度调整UI - 完成
  - ✅ P4.3: 更多调整类型 - 完成
  - ✅ P4.4: 高级调整功能 - 完成
  - ✅ P4.5: 高斯模糊UI - 完成
  - ✅ P4.6: 图像变换功能 - 完成
- ✅ **P5: UI/UX完善与进阶功能** - 完成
  - ✅ P5.1: 工具选项面板 - 完成
  - ✅ P5.2: 状态栏与信息面板 - 完成
  - ✅ P5.3: 颜色选择器与调色板 - 完成
  - ✅ P5.4: 标尺、网格与参考线 - 完成
  - ✅ P5.5: 键盘快捷键系统 - 完成
  - ✅ P5.6: 颜色管理系统(CMS)初步集成 - 完成
  - ✅ P5.7: 历史记录面板 - 完成

**当前测试状态**: 193个测试全部通过 (包含8个新增吸管工具测试)

---

**阶段 2: 基础图层系统**

*   **P2.1: 图层数据结构** ✅
    *   [x] 定义`Layer`结构体 (包含像素数据, 可见性, 不透明度, 混合模式-初期仅Normal)。
    *   [x] 项目文档结构中集成图层列表 `Vec<Layer>`。
*   **P2.2: 图层UI面板**
    *   [x] 显示图层列表。
    *   [x] 允许用户选择当前活动图层。
    *   [x] 实现图层操作：添加新图层 (空白/从文件)、删除图层、复制图层。
    *   [x] 实现图层属性控制：可见性切换、不透明度调整。
    *   [x] 实现图层顺序调整 (上移/下移)。
*   **P2.3: 图层混合与渲染**
    *   [x] 实现图层从下到上依次混合渲染到主画布。
    *   [x] 实现基础的Alpha混合 (Normal blend mode)。
*   **P2.4: 更新文件IO以支持多图层项目**
    *   [x] 设计自定义项目文件格式 (e.g.,基于 RON, JSON, or a binary format like MessagePack) 用于保存图层信息。
    *   [x] 实现自定义项目文件的加载与保存。
    *   [x] "导出为平面图像" 功能 (PNG, JPEG)。

---

**阶段 3: 核心编辑工具**

*   **P3.1: 工具抽象与管理**
    *   [x] 定义 `Tool` trait，包含激活、停用、鼠标事件处理等方法。
    *   [x] 实现工具管理器，用于切换当前活动工具。
    *   [x] 工具栏UI，用于选择工具。
*   **P3.2: 选区工具 (基础)** ✅ **已完成**
    *   [x] 实现矩形选区工具。
    *   [x] 在画布上绘制选区边框。
    *   [x] 选区数据结构 (e.g., 矩形坐标，未来可扩展为Mask)。
    *   [x] 约束操作在选区内（对于后续工具）。
*   **P3.3: 画笔工具 (Brush Tool)**
    *   [x] 基本画笔实现 (在活动图层上绘制)。
    *   [x] 工具选项：颜色选择 (使用 `iced`), 画笔大小。
    *   [x] （可选）画笔硬度/边缘羽化 (初期可简化)。
*   **P3.4: 橡皮擦工具 (Eraser Tool)** ✅ **已完成**
    *   [x] 类似画笔，但擦除像素 (设置Alpha为0)。
    *   [x] 工具选项：大小、硬度。
*   **P3.5: 移动工具 (Move Tool)** ✅ **已完成**
    *   [x] 移动当前活动图层的内容。
    *   [x] 移动选区内容（基础实现）。
*   **P3.6: 撤销/重做系统 (Undo/Redo)**
    *   [x] 实现命令模式 (Command Pattern)。
    *   [x] 每个可撤销操作封装为一个`Command`对象。
    *   [x] 维护撤销栈和重做栈。
    *   [x] UI集成 (菜单项、快捷键)。

---

**阶段 4: 基础图像调整与滤镜**

*   **P4.1: 调整/滤镜框架** ✅
    *   [x] 定义调整/滤镜应用的接口。
    *   [x] 考虑作用范围：整个图层或当前选区。
    *   [x] 实现亮度和对比度调整。
    *   [x] 集成到命令系统支持撤销/重做。
    *   [x] 添加GUI菜单集成。
*   **P4.2: 亮度/对比度调整**
    *   [x] 实现算法 (使用 `imageproc` 或手写)。
    *   [x] UI对话框或面板，带滑块控制。
    *   [x] 实时预览（可选，初期可先应用后查看）。
*   **P4.3: 更多调整类型** ✅
    *   [x] HSL调整（色相/饱和度/明度独立控制）。
    *   [x] 灰度化调整（4种算法：平均值、亮度、明度、自定义）。
    *   [x] 色彩平衡调整（阴影/中间调/高光独立控制）。
    *   [x] GUI菜单集成和扩展的调整消息系统。
*   **P4.4: 高级调整功能** ✅
    *   [x] 曲线调整（RGB和单通道曲线）。
    *   [x] 色阶调整（输入/输出色阶控制）。
*   **P4.5: 高斯模糊 (Gaussian Blur)** ✅
    *   [x] 实现算法 (可使用 `imageproc::filter::gaussian_blur_f32`)。
    *   [x] UI控制模糊半径。
*   **P4.6: 图像变换**
    *   [x] 自由变换（Free Transform）的初步实现：先实现图层/选区内容的缩放和旋转。
    *   [x] UI操作手柄。

---

**阶段 5: UI/UX 完善与进阶核心功能**

*   **P5.1: 工具选项面板** ✅
    *   [x] 为每个工具设计并实现上下文相关的选项面板。
    *   [x] 动态工具选项显示系统
    *   [x] 滑块、颜色选择器、复选框等UI控件
    *   [x] 实时选项值更新和同步
    *   [x] 选项重置功能
    *   [x] 所有工具的选项配置（画笔、橡皮擦、选择、移动、变换）
*   **P5.2: 状态栏与信息面板**
    *   [x] 显示当前图像信息 (尺寸、颜色模式、缩放级别)。
    *   [x] 显示鼠标坐标、颜色值。
*   **P5.3: 颜色选择器与调色板** ✅ **已完成**
    *   [x] 完整的颜色选择器对话框（RGB/HSL控制、十六进制输入、预设颜色）。
    *   [x] 调色板管理系统（创建/保存/加载调色板、内置调色板集合）。
    *   [x] 颜色历史记录组件（最近使用颜色、持久化存储）。
    *   [x] 应用程序集成（消息系统、菜单集成、工具颜色应用）。
*   **P5.4: 标尺、网格与参考线**
    *   [x] 在画布边缘显示标尺。
    *   [x] 可选显示网格和可拖拽的参考线。
*   **P5.5: 键盘快捷键系统** ✅
    *   [x] 为常用操作添加快捷键。
    *   [ ] （可选）允许用户自定义快捷键。
*   **P5.6: 颜色管理系统 (CMS) 初步集成** ✅
    *   [x] 使用 `lcms2`。
    *   [x] 图像加载时读取嵌入的ICC Profile。
    *   [x] 显示时转换为显示器配置文件 (sRGB作为默认)。
    *   [x] 保存时可选择嵌入ICC Profile。
    *   [x] **注意:** 颜色管理非常复杂，初期目标是基本支持。
*   **P5.7: 历史记录面板 (History Panel)** ✅
    *   [x] 可视化显示撤销/重做栈中的操作。
    *   [x] 允许用户点击返回到历史状态。
    *   [x] 清除历史记录功能。
    *   [x] 当前位置高亮显示。

---

**阶段 6: 高级工具与功能**

*   **P6.1: 更多选区工具** ✅
    *   [x] 椭圆选区工具。
    *   [x] 套索工具 (Lasso Tool) - 自由手绘选区。
    *   [x] 魔棒工具 (Magic Wand) - 基于颜色相似性选择。
    *   [x] 选区操作：加选、减选、交叉选。
*   **P6.2: 文本工具 (Text Tool)** ✅
    *   [x] 添加文本图层。
    *   [x] 字体选择、大小、颜色、对齐。
    *   [x] 文本渲染 (e.g., 使用 `rusttype` 或 `ab_glyph`).
*   **P6.3: 渐变工具 (Gradient Tool)**
    *   [x] 线性、径向渐变。
    *   [x] 颜色停止点编辑。
*   **P6.4: 形状工具 (Shape Tools)**
    *   [x] 绘制矩形、椭圆、线条、多边形 (作为矢量形状图层或栅格化)。
*   **P6.5: 更多混合模式 (Blend Modes)** ✅
    *   [x] 实现常见的混合模式 (Multiply, Screen, Overlay, Soft Light等)。
    *   [x] 增强图层面板显示混合模式和不透明度信息。
    *   [x] 实现LayerMessage::ChangeLayerBlendMode消息处理。
    *   [x] 完整的16种专业级混合模式GUI支持。
*   **P6.6: 裁剪工具 (Crop Tool)** ✅
    *   [x] 实现完整的CropTool（支持自由裁剪、固定比例、正方形模式）。
    *   [x] 裁剪命令系统（CropDocumentCommand和CropLayerCommand）。
    *   [x] 工具选项配置（裁剪模式和预览开关）。
    *   [x] GUI集成（裁剪图标和工具栏集成）。
*   **P6.7: 吸管工具 (Eyedropper Tool)** ✅
    *   [x] 从画布拾取颜色到前景色/背景色。


**阶段 7: 非破坏性编辑探索 (Highly Advanced)**

*   **P7.1: 调整图层 (Adjustment Layers)** ✅
    *   [x] 概念引入：将亮度/对比度、HSL等调整作为一种特殊图层。
    *   [x] 调整图层影响其下的所有可见图层。
    *   [x] 调整图层本身不含像素数据，只含调整参数。
    *   [x] 渲染引擎集成调整图层处理逻辑。
    *   [x] GUI界面支持调整图层创建和显示。
    *   [x] 支持调整图层的不透明度控制。
    *   [x] 完整的单元测试和集成测试覆盖。
*   **P7.2: 图层蒙版 (Layer Masks)** ✅
    *   [x] 每个图层可以关联一个灰度蒙版，控制该图层的像素可见性。
    *   [x] 蒙版数据结构和API（创建、移除、编辑、反转）。
    *   [x] 渲染引擎蒙版集成，支持实时蒙版效果。
    *   [x] GUI蒙版界面（图层面板显示、蒙版操作消息）。
    *   [x] 蒙版命令系统（撤销/重做支持）。
    *   [x] 全面的单元测试（5个新增测试）。
    *   [ ] 允许在蒙版上使用画笔等工具进行编辑（计划P8.1）。
*   **P7.3: （可选）智能对象/图层 (Smart Objects/Layers)**
    *   [x] 允许嵌入其他图像或矢量内容，并进行非破坏性变换。这个非常复杂。

---

**阶段 8: 扩展性、优化与发布准备**

*   **P8.1: 性能分析与优化**
    *   [x] 使用分析工具 (e.g., `perf`, `flamegraph`) 找出性能瓶颈。
    *   [x] 优化渲染、图像处理算法 (利用 `rayon` 进行并行化, SIMD)。
    *   [x] 优化内存使用。
*   **P8.2: 插件/脚本系统架构设计 (非常高级)**
    *   [ ] 考虑使用 `mlua` (Lua) 或 WASM (via `wasmtime`/`wasmer`) 实现插件。
    *   [ ] 定义插件API。
*   **P8.3: 首选项/设置对话框**
    *   [x] 允许用户配置界面主题、性能选项、默认行为等。
*   **P8.4: 用户文档与教程**
    *   [ ] 编写用户手册。
    *   [ ] 制作入门教程。
*   **P8.5: 多语言支持 (Internationalization - i18n)**
    *   [x] 使用 `fluent-rs` 或类似库。
*   **P8.6: 应用程序打包与分发**
    *   [ ] 为 Windows, macOS, Linux 创建安装包/可执行文件。
    *   [ ] (若使用 Tauri) Tauri内置打包工具。
    *   [ ] 需要研究各平台的打包方法。
*   **P8.7: 最终测试与Bug修复**
    *   [ ] Alpha/Beta测试阶段。
    *   [ ] 收集用户反馈。

---

**贯穿所有阶段的任务：**

*   **代码审查：** 保证代码质量和知识共享。
*   **编写单元测试和集成测试：** 确保功能正确性和防止回归。
*   **文档编写：** 代码注释、架构文档、API文档（如果适用）。
*   **版本控制：** 频繁提交，清晰的提交信息。
*   **持续集成：** 自动化构建和测试。
*   **Bug跟踪：** 使用Issue Tracker (e.g., GitHub Issues)。

---

**重要提示：**

*   **从小处着手：** 这个列表非常庞大。务必从MVP开始，逐步迭代。不要试图一次性实现所有功能。
*   **专注核心：** 早期阶段专注于核心图像处理逻辑和稳定的画布。
*   **社区和Crates：** 积极利用Rust社区和crates.io上已有的库，避免重复造轮子（除非有明确的性能或功能需求）。
*   **这是一个马拉松，不是短跑。** 保持耐心和持续的努力。

---

## 🎯 当前项目状态 (P7.1 - 调整图层完成)

### 已完成阶段
- ✅ **P0**: 项目初始化和基础架构
- ✅ **P1.1-P1.6**: 完整的图像文件IO系统
- ✅ **P2.1-P2.4**: 完整的图层系统
- ✅ **P3.1-P3.6**: 完整的工具系统和撤销/重做架构
- ✅ **P4.1-P4.6**: 完整的调整/滤镜系统和图像变换
- ✅ **P5.1-P5.7**: 完整的高级UI功能
- ✅ **P6.1-P6.7**: 高级工具、混合模式、裁剪和吸管系统
- ✅ **P7.1**: 调整图层系统

### 核心功能特性
- **16种专业混合模式**: Normal, Multiply, Screen, Overlay, SoftLight, HardLight, ColorDodge, ColorBurn, Darken, Lighten, Difference, Exclusion, Hue, Saturation, Color, Luminosity
- **完整工具集**: 选择、椭圆选择、套索、魔棒、画笔、橡皮擦、移动、变换、文本、渐变、形状、裁剪、吸管工具
- **专业调整功能**: 亮度/对比度、HSL、曲线、色阶、灰度化、色彩平衡
- **高级滤镜**: 高斯模糊、运动模糊、反锐化蒙版、基础锐化、添加噪点、降噪
- **完整UI系统**: 工具选项、状态栏、颜色选择器、调色板、历史记录、标尺网格等
- **裁剪功能**: 自由裁剪、固定比例(16:9, 4:3, 3:2)、正方形模式
- **吸管工具**: 颜色拾取、多种采样大小(1x1/3x3/5x5)、前景/背景色选择
- **调整图层**: 非破坏性编辑、亮度/对比度/HSL/灰度化调整图层、不透明度控制

### 测试状态
- **总测试数**: 342个
- **通过率**: 100%
- **覆盖率**: 全面覆盖所有核心功能
- **新增**: 12个调整图层专项测试

### 下一步计划
- **P7**: 非破坏性编辑探索
- **P8**: 扩展性、优化与发布准备

项目已达到专业图像编辑软件的核心功能水准，具备完整的工具系统、图层管理、调整滤镜和用户界面。

