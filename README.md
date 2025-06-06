# PSOC - 一款用Rust构建的图像编辑器 🎨🦀

[![Build Status](https://img.shields.io/github/actions/workflow/status/YOUR_USERNAME/YOUR_REPONAME/rust.yml?branch=main)](https://github.com/YOUR_USERNAME/YOUR_REPONAME/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.XX.X+-93450a.svg)](https://www.rust-lang.org)
<!-- [![Crates.io](https://img.shields.io/crates/v/your-crate-name.svg)](https://crates.io/crates/your-crate-name) -->

**PSOC** 是一个雄心勃勃的开源项目，旨在利用Rust编程语言的强大功能（性能、内存安全、并发性）从头开始构建一款功能丰富的桌面图像编辑应用程序，灵感来源于Photoshop、GIMP和Krita等优秀软件。

## ✨ 项目愿景与目标

*   **高性能：** 利用Rust的零成本抽象和对底层控制的能力，实现快速的图像处理和流畅的用户体验。
*   **内存安全：** 借助Rust的所有权和借用系统，从根本上减少内存相关的错误和安全漏洞。
*   **现代化：** 采用现代的UI/UX设计理念和软件架构。
*   **跨平台：** 目标是支持主流桌面操作系统 (Windows, macOS, Linux)。
*   **模块化与可扩展：** 设计易于维护和扩展的架构，未来可能支持插件系统。
*   **学习与探索：** 探索Rust在大型桌面应用程序和图形处理领域的潜力。

## 🚀 主要特性 (计划中与进行中)

这是一个长期项目，特性将逐步实现。以下是我们规划的一些核心功能：

*   **核心引擎：**
    *   [x] 高效的2D渲染画布 (基于 `iced` 和 `wgpu`)
    *   [x] 图像文件 I/O (PNG, JPEG, 自定义PSOC项目格式)
    *   [x] 颜色管理 (RGB, HSL, HSV颜色空间)
*   **图层系统：**
    *   [x] 基础图层操作 (创建, 删除, 复制, 显隐, 不透明度, 顺序调整)
    *   [x] 16种专业图层混合模式 (Normal, Multiply, Screen, Overlay等)
    *   [x] 完整的图层渲染引擎
    *   [ ] (计划) 调整图层
    *   [ ] (计划) 图层蒙版
*   **工具系统：**
    *   [x] 工具抽象与管理架构
    *   [x] 统一的工具事件处理系统
    *   [x] 工具选项配置系统
    *   [x] 工具切换和状态管理
*   **选择工具：**
    *   [x] 基础选择工具框架
    *   [ ] 矩形、椭圆选区实现
    *   [ ] (计划) 套索、多边形套索、魔棒工具
    *   [ ] (计划) 选区加减乘操作
*   **绘图与编辑工具：**
    *   [x] 画笔工具框架 (可调大小、硬度、颜色)
    *   [x] 橡皮擦工具框架
    *   [x] 移动工具框架
    *   [ ] 具体绘图功能实现
    *   [ ] (计划) 填充工具、渐变工具
    *   [ ] (计划) 文本工具
    *   [ ] (计划) 形状工具、裁剪工具
*   **图像调整与滤镜：**
    *   [ ] 亮度/对比度, 色相/饱和度/明度
    *   [ ] (计划) 色阶、曲线
    *   [ ] (计划) 常见滤镜 (模糊、锐化、降噪等)
*   **用户界面：**
    *   [x] 直观的图形用户界面 (使用 `iced`)
    *   [x] 图层面板和画布显示
    *   [x] 文件对话框集成
    *   [ ] 可定制的工具栏、面板布局
    *   [ ] 撤销/重做系统
    *   [ ] 历史记录面板
    *   [ ] 标尺、网格、参考线
*   **其他：**
    *   [ ] (高级) 非破坏性编辑工作流
    *   [ ] (高级) 脚本/插件支持 (可能通过 Lua/WASM)

## 🛠️ 技术栈

*   **语言：** Rust
*   **GUI框架：** `iced` (现代化的Rust GUI框架)
*   **渲染后端：** `wgpu` (通过`iced`) + `tiny-skia`
*   **图像处理：** `image`, `ndarray`, `rayon`
*   **颜色管理：** 自定义实现 (RGB, HSL, HSV)
*   **异步运行时：** `tokio`
*   **序列化：** `serde`, `ron` (项目文件格式)
*   **构建系统：** Cargo
*   **核心数据结构与算法：** 自定义实现，利用Rust标准库和生态系统中的优秀crates。

## 📊 项目状态与路线图

本项目目前处于 **Alpha开发阶段**。

### 已完成的里程碑：
*   ✅ **P0阶段**: 项目基础设施和架构设计
*   ✅ **P1阶段**: 核心数据结构、文件IO、错误处理、GUI基础
*   ✅ **P2.1**: 图层数据结构实现
*   ✅ **P2.2**: 图层UI面板开发
*   ✅ **P2.3**: 图层混合与渲染
*   ✅ **P2.4**: 多图层项目文件支持
*   ✅ **P3.1**: 工具抽象与管理系统
*   ✅ **P3.2**: 选区工具实现
*   ✅ **P3.3**: 画笔工具实现
*   ✅ **P3.4**: 橡皮擦工具实现

### 当前状态：
*   **158个单元测试** 全部通过（55个库测试 + 53个核心库测试 + 37个集成测试 + 13个其他测试）
*   **完整的图层系统** 支持16种混合模式
*   **项目文件格式** 支持保存/加载多图层文档
*   **工具系统架构** 完整的工具抽象与管理
*   **选区工具** 支持矩形选区和选区渲染
*   **画笔工具** 支持可配置大小、硬度、颜色的高质量绘画
*   **橡皮擦工具** 支持可配置大小、硬度的专业级Alpha通道擦除
*   **GUI应用程序** 基本功能完整
*   **代码质量** 所有clippy错误已修复

详细的开发文档可以在 `docs/` 目录中找到。

## 🏁 开始使用 (开发构建)

**先决条件：**

*   安装最新稳定版的 [Rust 工具链](https://www.rust-lang.org/tools/install) (rustc, cargo)。
*   (可选) 如果你选择的GUI框架有其他系统依赖（例如 `gtk-rs` 需要GTK开发库），请根据其文档安装。

**构建步骤：**

1.  克隆仓库：
    ```bash
    git clone https://github.com/YOUR_USERNAME/YOUR_REPONAME.git
    cd YOUR_REPONAME
    ```
2.  构建项目：
    *   调试构建：
        ```bash
        cargo build
        ```
    *   发行构建 (优化性能)：
        ```bash
        cargo build --release
        ```
3.  运行应用程序：
    *   调试模式：
        ```bash
        cargo run
        ```
    *   发行模式：
        ```bash
        cargo run --release
        # 或者直接运行target/release/[executable_name]
        ```

## 🖼️ 截图 (敬请期待)

*(当UI初具雏形时，在这里添加截图)*

## 🤝 贡献指南

我们热烈欢迎各种形式的贡献！无论是代码、文档、Bug报告还是功能建议。

1.  **报告Bug或建议功能：** 请通过 [GitHub Issues](https://github.com/YOUR_USERNAME/YOUR_REPONAME/issues) 提交。
2.  **参与开发：**
    *   Fork 本仓库。
    *   创建一个新的特性分支 (`git checkout -b feature/AmazingFeature`)。
    *   提交你的更改 (`git commit -m 'Add some AmazingFeature'`)。
    *   将分支推送到你的Fork (`git push origin feature/AmazingFeature`)。
    *   创建一个 Pull Request。
3.  **代码风格：** 请运行 `rustfmt` 和 `clippy` 以确保代码风格一致并通过静态检查。
    ```bash
    cargo fmt
    cargo clippy -- -D warnings
    ```

请查阅 `CONTRIBUTING.md` (如果创建了该文件) 获取更详细的贡献指南。

## 📜 开源许可

本项目采用双重许可： **MIT License** 和 **Apache License 2.0**。
你可以根据自己的需求选择其中任一许可。

*   [LICENSE-MIT](LICENSE-MIT)
*   [LICENSE-APACHE](LICENSE-APACHE)

(除非你另有说明，否则你的所有贡献都将默认采用此双重许可。)

## 🙏 致谢

*   感谢所有为Rust生态系统做出贡献的开发者。
*   感谢 [GIMP](https://www.gimp.org/), [Krita](https://krita.org/), [Photoshop](https://www.adobe.com/products/photoshop.html) 等优秀软件提供的灵感。
*   感谢以下关键crates的作者和维护者：
    *   `image`
    *   `wgpu` / `iced` / `egui` / `gtk-rs` / `tauri` (根据你的选择)
    *   `ndarray`
    *   `lcms2-rs`
    *   ... (其他你用到的重要库)

---

**免责声明：** 这是一个爱好者驱动的复杂项目，开发周期可能较长。请耐心等待并欢迎加入我们一起构建！