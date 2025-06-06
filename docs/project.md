# PSOC 项目结构规划

## 项目概述

PSOC (Photoshop-like Software on Rust) 是一个基于Rust技术栈开发的图像编辑器，旨在提供类似Photoshop的功能。本文档详细规划了项目的目录结构、模块组织和架构设计。

## 项目根目录结构

```
psoc/
├── Cargo.toml                 # 主项目配置文件
├── Cargo.lock                 # 依赖锁定文件
├── README.md                   # 项目主要说明文档
├── LICENSE-MIT                 # MIT许可证
├── LICENSE-APACHE              # Apache 2.0许可证
├── CONTRIBUTING.md             # 贡献指南
├── CHANGELOG.md                # 版本更新日志
├── .gitignore                  # Git忽略文件配置
├── .github/                    # GitHub相关配置
│   ├── workflows/              # CI/CD工作流
│   │   ├── rust.yml           # Rust构建和测试
│   │   ├── release.yml        # 发布流程
│   │   └── docs.yml           # 文档构建
│   ├── ISSUE_TEMPLATE/         # Issue模板
│   └── PULL_REQUEST_TEMPLATE.md # PR模板
├── docs/                       # 项目文档
│   ├── README.md              # 开发任务列表
│   ├── project.md             # 项目结构文档(本文件)
│   ├── architecture.md        # 架构设计文档
│   ├── api/                   # API文档
│   ├── user-guide/            # 用户指南
│   └── dev-guide/             # 开发者指南
├── src/                        # 主要源代码
├── crates/                     # 子crate模块
├── tests/                      # 集成测试
├── benches/                    # 性能基准测试
├── examples/                   # 示例代码
├── assets/                     # 静态资源
├── scripts/                    # 构建和部署脚本
└── target/                     # 构建输出目录(git忽略)
```

## 核心模块结构 (src/)

```
src/
├── main.rs                     # 应用程序入口点
├── lib.rs                      # 库入口点
├── app/                        # 应用程序主框架
│   ├── mod.rs                 # 模块声明
│   ├── application.rs         # 主应用程序结构
│   ├── config.rs              # 应用配置管理
│   ├── state.rs               # 应用状态管理
│   └── events.rs              # 事件系统
├── ui/                         # 用户界面模块
│   ├── mod.rs
│   ├── window.rs              # 主窗口
│   ├── canvas.rs              # 画布组件
│   ├── panels/                # 各种面板
│   │   ├── mod.rs
│   │   ├── layers.rs          # 图层面板
│   │   ├── tools.rs           # 工具面板
│   │   ├── properties.rs      # 属性面板
│   │   ├── history.rs         # 历史记录面板
│   │   └── color.rs           # 颜色选择器
│   ├── dialogs/               # 对话框
│   │   ├── mod.rs
│   │   ├── file.rs            # 文件对话框
│   │   ├── about.rs           # 关于对话框
│   │   └── preferences.rs     # 首选项对话框
│   ├── widgets/               # 自定义UI组件
│   │   ├── mod.rs
│   │   ├── slider.rs          # 滑块组件
│   │   ├── color_picker.rs    # 颜色选择器
│   │   └── toolbar.rs         # 工具栏
│   └── theme.rs               # UI主题系统
├── core/                       # 核心逻辑模块
│   ├── mod.rs
│   ├── document.rs            # 文档数据结构
│   ├── layer.rs               # 图层系统
│   ├── selection.rs           # 选区系统
│   ├── history.rs             # 撤销/重做系统
│   ├── color.rs               # 颜色管理
│   └── math.rs                # 数学工具函数
├── tools/                      # 工具系统
│   ├── mod.rs
│   ├── tool_trait.rs          # 工具接口定义
│   ├── tool_manager.rs        # 工具管理器
│   ├── brush.rs               # 画笔工具
│   ├── eraser.rs              # 橡皮擦工具
│   ├── selection/             # 选区工具
│   │   ├── mod.rs
│   │   ├── rectangle.rs       # 矩形选区
│   │   ├── ellipse.rs         # 椭圆选区
│   │   └── lasso.rs           # 套索工具
│   ├── move_tool.rs           # 移动工具
│   └── crop.rs                # 裁剪工具
├── rendering/                  # 渲染系统
│   ├── mod.rs
│   ├── renderer.rs            # 主渲染器
│   ├── canvas_renderer.rs     # 画布渲染
│   ├── layer_renderer.rs      # 图层渲染
│   ├── blend_modes.rs         # 混合模式
│   └── gpu/                   # GPU加速渲染
│       ├── mod.rs
│       ├── shaders/           # 着色器
│       └── buffers.rs         # 缓冲区管理
├── image_processing/           # 图像处理模块
│   ├── mod.rs
│   ├── filters/               # 滤镜系统
│   │   ├── mod.rs
│   │   ├── blur.rs            # 模糊滤镜
│   │   ├── sharpen.rs         # 锐化滤镜
│   │   └── noise.rs           # 噪点滤镜
│   ├── adjustments/           # 图像调整
│   │   ├── mod.rs
│   │   ├── brightness.rs      # 亮度调整
│   │   ├── contrast.rs        # 对比度调整
│   │   ├── hsl.rs             # 色相/饱和度/明度
│   │   └── curves.rs          # 曲线调整
│   ├── transforms/            # 图像变换
│   │   ├── mod.rs
│   │   ├── resize.rs          # 缩放
│   │   ├── rotate.rs          # 旋转
│   │   └── perspective.rs     # 透视变换
│   └── algorithms/            # 图像处理算法
│       ├── mod.rs
│       ├── convolution.rs     # 卷积运算
│       └── interpolation.rs   # 插值算法
├── file_io/                    # 文件输入输出
│   ├── mod.rs
│   ├── formats/               # 文件格式支持
│   │   ├── mod.rs
│   │   ├── png.rs             # PNG格式
│   │   ├── jpeg.rs            # JPEG格式
│   │   ├── tiff.rs            # TIFF格式
│   │   ├── psd.rs             # PSD格式(读取)
│   │   └── psoc.rs            # 自定义项目格式
│   ├── import.rs              # 导入功能
│   ├── export.rs              # 导出功能
│   └── project.rs             # 项目文件管理
├── plugins/                    # 插件系统(高级功能)
│   ├── mod.rs
│   ├── plugin_trait.rs        # 插件接口
│   ├── manager.rs             # 插件管理器
│   └── scripting/             # 脚本支持
│       ├── mod.rs
│       ├── lua.rs             # Lua脚本
│       └── wasm.rs            # WebAssembly插件
├── utils/                      # 工具函数
│   ├── mod.rs
│   ├── error.rs               # 错误处理
│   ├── logging.rs             # 日志系统
│   ├── config.rs              # 配置管理
│   └── platform.rs           # 平台相关功能
└── prelude.rs                  # 常用导入预设
```

## 子Crate模块结构 (crates/)

为了更好的模块化和代码复用，将一些独立的功能模块拆分为独立的crate：

```
crates/
├── psoc-core/                  # 核心数据结构和算法
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── color.rs           # 颜色空间和管理
│   │   ├── geometry.rs        # 几何计算
│   │   ├── image.rs           # 图像数据结构
│   │   └── math.rs            # 数学工具
│   └── tests/
├── psoc-image-processing/      # 图像处理算法库
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── filters/           # 滤镜算法
│   │   ├── transforms/        # 变换算法
│   │   └── algorithms/        # 基础算法
│   ├── tests/
│   └── benches/               # 性能测试
├── psoc-file-formats/          # 文件格式支持
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── png.rs
│   │   ├── jpeg.rs
│   │   ├── tiff.rs
│   │   └── psd.rs
│   └── tests/
├── psoc-ui-toolkit/            # UI组件库
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── widgets/
│   │   ├── themes/
│   │   └── layouts/
│   └── examples/
└── psoc-plugins/               # 插件系统
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── api.rs             # 插件API定义
    │   ├── manager.rs         # 插件管理
    │   └── scripting/
    └── examples/
```

## 测试结构 (tests/)

```
tests/
├── integration/                # 集成测试
│   ├── ui_tests.rs            # UI集成测试
│   ├── file_io_tests.rs       # 文件IO测试
│   ├── image_processing_tests.rs # 图像处理测试
│   └── performance_tests.rs   # 性能测试
├── fixtures/                   # 测试数据
│   ├── images/                # 测试图像
│   │   ├── test_rgb.png
│   │   ├── test_cmyk.tiff
│   │   └── test_layers.psd
│   └── projects/              # 测试项目文件
└── common/                     # 测试工具函数
    ├── mod.rs
    ├── test_utils.rs
    └── mock_data.rs
```

## 示例代码结构 (examples/)

```
examples/
├── basic_usage.rs              # 基本使用示例
├── custom_filter.rs            # 自定义滤镜示例
├── plugin_development.rs       # 插件开发示例
├── batch_processing.rs         # 批处理示例
└── headless_processing.rs      # 无头图像处理
```

## 静态资源结构 (assets/)

```
assets/
├── icons/                      # 应用图标
│   ├── app_icon.ico           # Windows图标
│   ├── app_icon.icns          # macOS图标
│   └── app_icon.png           # Linux图标
├── ui/                         # UI资源
│   ├── icons/                 # 工具图标
│   │   ├── brush.svg
│   │   ├── eraser.svg
│   │   ├── selection.svg
│   │   └── ...
│   ├── cursors/               # 自定义光标
│   └── themes/                # 主题资源
│       ├── dark.toml
│       ├── light.toml
│       └── high_contrast.toml
├── fonts/                      # 字体文件
├── shaders/                    # GPU着色器
│   ├── vertex/
│   ├── fragment/
│   └── compute/
└── localization/               # 国际化资源
    ├── en.ftl                 # 英文
    ├── zh-CN.ftl              # 简体中文
    ├── zh-TW.ftl              # 繁体中文
    └── ja.ftl                 # 日文
```

## 脚本和工具 (scripts/)

```
scripts/
├── build.sh                    # 构建脚本
├── test.sh                     # 测试脚本
├── release.sh                  # 发布脚本
├── setup_dev.sh               # 开发环境设置
├── generate_docs.sh            # 文档生成
├── package/                    # 打包脚本
│   ├── windows.ps1            # Windows打包
│   ├── macos.sh               # macOS打包
│   └── linux.sh               # Linux打包
└── tools/                      # 开发工具
    ├── icon_generator.py       # 图标生成工具
    ├── shader_compiler.rs      # 着色器编译工具
    └── asset_optimizer.py      # 资源优化工具
```

## 架构设计原则

### 1. 模块化设计
- **高内聚低耦合**: 每个模块专注于特定功能，模块间依赖最小化
- **接口驱动**: 使用Trait定义清晰的接口，便于测试和扩展
- **分层架构**: UI层、业务逻辑层、数据层清晰分离

### 2. 性能优化
- **零成本抽象**: 充分利用Rust的零成本抽象特性
- **内存管理**: 合理使用所有权系统，避免不必要的内存分配
- **并行处理**: 使用rayon等库进行图像处理的并行化
- **GPU加速**: 关键渲染和计算使用GPU加速

### 3. 可扩展性
- **插件系统**: 支持动态加载插件扩展功能
- **事件驱动**: 使用事件系统解耦组件间通信
- **配置化**: 支持运行时配置和主题切换

### 4. 跨平台兼容
- **平台抽象**: 将平台相关代码封装在独立模块中
- **统一API**: 提供统一的跨平台API接口
- **资源管理**: 平台相关资源的统一管理

## 技术选型

### GUI框架选择
基于项目需求分析，推荐使用 **iced** 作为主要GUI框架：

**优势:**
- 现代化的响应式UI框架
- 基于wgpu的高性能渲染
- 良好的跨平台支持
- 活跃的社区和持续更新
- 适合复杂图形应用

**备选方案:**
- **egui**: 轻量级，适合快速原型
- **Tauri**: Web技术栈，开发效率高
- **gtk-rs**: 成熟稳定，但学习曲线陡峭

### 渲染后端
- **主要选择**: wgpu (通过iced集成)
- **备选**: tiny-skia (CPU渲染)
- **GPU计算**: wgpu compute shaders

### 核心依赖库

```toml
[dependencies]
# GUI框架
iced = { version = "0.10", features = ["canvas", "image", "svg"] }
iced_wgpu = "0.11"
iced_winit = "0.10"

# 图像处理
image = { version = "0.24", features = ["png", "jpeg", "tiff", "webp"] }
imageproc = "0.23"
ndarray = "0.15"

# 颜色管理
lcms2 = "6.0"
palette = "0.7"

# 文件IO
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ron = "0.8"
rfd = "0.11"  # 文件对话框

# 错误处理和日志
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 并行处理
rayon = "1.7"

# 数学计算
nalgebra = "0.32"
glam = "0.24"

# 插件系统(高级功能)
mlua = { version = "0.9", optional = true }
wasmtime = { version = "13.0", optional = true }

# 平台相关
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.24"
objc = "0.2"

[target.'cfg(unix)'.dependencies]
nix = "0.27"
```

## 开发流程和规范

### 代码规范
```toml
# .rustfmt.toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
edition = "2021"
```

### Git工作流
- **主分支**: `main` - 稳定发布版本
- **开发分支**: `develop` - 开发集成分支
- **功能分支**: `feature/功能名` - 新功能开发
- **修复分支**: `hotfix/问题描述` - 紧急修复
- **发布分支**: `release/版本号` - 发布准备

### CI/CD流程
```yaml
# .github/workflows/rust.yml
name: Rust CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]

    steps:
    - uses: actions/checkout@v3
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy

    - name: Check formatting
      run: cargo fmt -- --check

    - name: Run clippy
      run: cargo clippy -- -D warnings

    - name: Run tests
      run: cargo test --verbose

    - name: Run benchmarks
      run: cargo bench
```

### 测试策略
1. **单元测试**: 每个模块的核心功能
2. **集成测试**: 模块间交互测试
3. **性能测试**: 关键算法的性能基准
4. **UI测试**: 用户界面交互测试
5. **端到端测试**: 完整工作流程测试

## 部署和分发

### 构建配置
```toml
# Cargo.toml 发布配置
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 0
debug = true
overflow-checks = true

[profile.bench]
opt-level = 3
debug = false
lto = true
```

### 平台特定构建
- **Windows**: 使用cargo-wix生成MSI安装包
- **macOS**: 创建.app bundle和.dmg分发包
- **Linux**: 提供AppImage和各发行版包

### 发布流程
1. 版本号更新 (遵循语义化版本)
2. 更新CHANGELOG.md
3. 创建release分支
4. 自动化测试通过
5. 构建各平台二进制文件
6. 创建GitHub Release
7. 发布到包管理器 (可选)

## 项目里程碑

### 阶段0: 项目基础 (1-2个月)
- [x] 项目结构设计
- [ ] 开发环境搭建
- [ ] 基础CI/CD配置
- [ ] 核心依赖选型确认

### 阶段1: MVP开发 (3-4个月)
- [ ] 基础窗口和画布
- [ ] 图像文件IO
- [ ] 简单图层系统
- [ ] 基础绘图工具

### 阶段2: 核心功能 (4-6个月)
- [ ] 完整图层系统
- [ ] 选区工具
- [ ] 撤销/重做系统
- [ ] 基础滤镜和调整

### 阶段3: 高级功能 (6-8个月)
- [ ] 高级工具集
- [ ] 非破坏性编辑
- [ ] 插件系统
- [ ] 性能优化

### 阶段4: 发布准备 (2-3个月)
- [ ] UI/UX完善
- [ ] 文档编写
- [ ] 多语言支持
- [ ] 最终测试和优化

## 风险评估和应对

### 技术风险
1. **性能问题**: 大图像处理性能瓶颈
   - 应对: 早期性能测试，GPU加速，算法优化
2. **内存使用**: 大文件内存占用过高
   - 应对: 流式处理，内存池，智能缓存
3. **跨平台兼容**: 不同平台行为差异
   - 应对: 持续集成测试，平台抽象层

### 项目风险
1. **范围蔓延**: 功能需求不断增加
   - 应对: 严格的版本规划，MVP优先
2. **技术债务**: 快速开发导致代码质量下降
   - 应对: 代码审查，重构计划，测试覆盖

## 总结

本项目结构设计遵循现代软件工程最佳实践，充分利用Rust语言特性，为构建高性能、可扩展的图像编辑器奠定了坚实基础。通过模块化设计、清晰的架构分层和完善的开发流程，确保项目能够稳步推进并达到预期目标。
