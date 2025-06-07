# PSOC 图像编辑器 - 简体中文本地化

# 应用程序
app-title = PSOC 图像编辑器
app-description = 基于 Rust 构建的类 Photoshop 图像编辑器

# 文件菜单
menu-file = 文件
menu-file-new = 新建
menu-file-open = 打开
menu-file-save = 保存
menu-file-save-as = 另存为
menu-file-export = 导出
menu-file-import = 导入
menu-file-recent = 最近文件
menu-file-exit = 退出

# 编辑菜单
menu-edit = 编辑
menu-edit-undo = 撤销
menu-edit-redo = 重做
menu-edit-cut = 剪切
menu-edit-copy = 复制
menu-edit-paste = 粘贴
menu-edit-delete = 删除
menu-edit-select-all = 全选
menu-edit-deselect = 取消选择
menu-edit-preferences = 首选项

# 视图菜单
menu-view = 视图
menu-view-zoom-in = 放大
menu-view-zoom-out = 缩小
menu-view-zoom-reset = 重置缩放
menu-view-zoom-fit = 适合窗口
menu-view-fullscreen = 全屏
menu-view-rulers = 显示标尺
menu-view-grid = 显示网格
menu-view-guides = 显示参考线

# 图层菜单
menu-layer = 图层
menu-layer-new = 新建图层
menu-layer-duplicate = 复制图层
menu-layer-delete = 删除图层
menu-layer-merge-down = 向下合并
menu-layer-flatten = 拼合图像
menu-layer-add-mask = 添加图层蒙版
menu-layer-remove-mask = 移除图层蒙版
menu-layer-add-adjustment = 添加调整图层

# 图像菜单
menu-image = 图像
menu-image-adjustments = 调整
menu-image-brightness-contrast = 亮度/对比度
menu-image-hsl = 色相/饱和度/明度
menu-image-curves = 曲线
menu-image-levels = 色阶
menu-image-color-balance = 色彩平衡
menu-image-grayscale = 灰度化

# 滤镜菜单
menu-filter = 滤镜
menu-filter-blur = 模糊
menu-filter-gaussian-blur = 高斯模糊
menu-filter-motion-blur = 运动模糊
menu-filter-sharpen = 锐化
menu-filter-unsharp-mask = 反锐化蒙版
menu-filter-noise = 噪点
menu-filter-add-noise = 添加噪点
menu-filter-reduce-noise = 降噪

# 工具菜单
menu-tools = 工具
menu-tools-color-picker = 颜色选择器
menu-tools-color-palette = 调色板

# 帮助菜单
menu-help = 帮助
menu-help-about = 关于

# 工具栏工具
tool-select = 选择
tool-ellipse-select = 椭圆选择
tool-lasso = 套索
tool-magic-wand = 魔棒
tool-move = 移动
tool-brush = 画笔
tool-eraser = 橡皮擦
tool-text = 文字
tool-gradient = 渐变
tool-shape = 形状
tool-crop = 裁剪
tool-eyedropper = 吸管
tool-transform = 变换

# 工具选项
tool-options = 工具选项
tool-option-size = 大小
tool-option-hardness = 硬度
tool-option-opacity = 不透明度
tool-option-color = 颜色
tool-option-font-family = 字体族
tool-option-font-size = 字体大小
tool-option-alignment = 对齐
tool-option-feather = 羽化
tool-option-anti-alias = 抗锯齿
tool-option-mode = 模式
tool-option-sample-size = 采样大小

# 对话框
dialog-ok = 确定
dialog-cancel = 取消
dialog-apply = 应用
dialog-reset = 重置
dialog-close = 关闭
dialog-yes = 是
dialog-no = 否

# 关于对话框
about-title = 关于 PSOC 图像编辑器
about-version = 版本 {$version}
about-description = 基于 Rust 和现代技术构建的专业图像编辑器。
about-copyright = 版权所有 © 2024 PSOC 开发团队
about-license = 基于 MIT 或 Apache-2.0 许可证

# 亮度/对比度对话框
brightness-contrast-title = 亮度/对比度
brightness-contrast-brightness = 亮度
brightness-contrast-contrast = 对比度
brightness-contrast-preview = 预览

# 颜色选择器对话框
color-picker-title = 颜色选择器
color-picker-red = 红色
color-picker-green = 绿色
color-picker-blue = 蓝色
color-picker-hue = 色相
color-picker-saturation = 饱和度
color-picker-lightness = 明度
color-picker-hex = 十六进制
color-picker-presets = 预设

# 状态栏
status-no-document = 无文档
status-document-saved = 已保存
status-document-unsaved = 未保存
status-zoom = 缩放: {$zoom}%
status-position = 位置: {$x}, {$y}
status-color = 颜色: {$color}
status-size = 大小: {$width} × {$height}
status-mode = 模式: {$mode}

# 图层面板
layer-panel-title = 图层
layer-panel-opacity = 不透明度
layer-panel-blend-mode = 混合模式
layer-panel-visible = 可见
layer-panel-locked = 锁定
layer-panel-add = 添加图层
layer-panel-delete = 删除图层
layer-panel-duplicate = 复制图层
layer-panel-move-up = 上移
layer-panel-move-down = 下移

# 历史记录面板
history-panel-title = 历史记录
history-panel-clear = 清除历史记录
history-panel-no-history = 无历史记录

# 文档信息
document-info-title = 文档
document-info-status = 状态
document-info-file = 文件
document-info-size = 大小
document-info-resolution = 分辨率
document-info-color-mode = 颜色模式

# 错误消息
error-file-not-found = 文件未找到: {$path}
error-file-read = 读取文件失败: {$path}
error-file-write = 写入文件失败: {$path}
error-invalid-format = 无效文件格式: {$format}
error-out-of-memory = 内存不足
error-unknown = 发生未知错误

# 成功消息
success-file-saved = 文件保存成功
success-file-opened = 文件打开成功
success-operation-completed = 操作完成

# 混合模式
blend-mode-normal = 正常
blend-mode-multiply = 正片叠底
blend-mode-screen = 滤色
blend-mode-overlay = 叠加
blend-mode-soft-light = 柔光
blend-mode-hard-light = 强光
blend-mode-color-dodge = 颜色减淡
blend-mode-color-burn = 颜色加深
blend-mode-darken = 变暗
blend-mode-lighten = 变亮
blend-mode-difference = 差值
blend-mode-exclusion = 排除
blend-mode-hue = 色相
blend-mode-saturation = 饱和度
blend-mode-color = 颜色
blend-mode-luminosity = 明度

# 调整类型
adjustment-brightness-contrast = 亮度/对比度
adjustment-hsl = HSL
adjustment-curves = 曲线
adjustment-levels = 色阶
adjustment-color-balance = 色彩平衡
adjustment-grayscale = 灰度化

# 形状工具
shape-rectangle = 矩形
shape-ellipse = 椭圆
shape-line = 直线
shape-polygon = 多边形

# 文字对齐
text-align-left = 左对齐
text-align-center = 居中
text-align-right = 右对齐

# 裁剪模式
crop-mode-free = 自由
crop-mode-fixed-ratio = 固定比例
crop-mode-square = 正方形

# 采样大小
sample-size-1x1 = 1×1 像素
sample-size-3x3 = 3×3 像素
sample-size-5x5 = 5×5 像素

# 单位
unit-pixels = 像素
unit-percent = %
unit-degrees = 度
unit-points = 点

# 常用操作
action-new = 新建
action-open = 打开
action-save = 保存
action-delete = 删除
action-copy = 复制
action-paste = 粘贴
action-undo = 撤销
action-redo = 重做
action-apply = 应用
action-cancel = 取消
action-reset = 重置
action-clear = 清除
action-add = 添加
action-remove = 移除
action-edit = 编辑
action-create = 创建
action-load = 加载
action-export = 导出
action-import = 导入

# 文件类型
file-type-png = PNG 图像
file-type-jpeg = JPEG 图像
file-type-psoc = PSOC 项目
file-type-all-images = 所有图像
file-type-all-files = 所有文件

# 语言选择器
language-selector-title = 语言
language-selector-placeholder = 选择语言

# 首选项对话框
preferences-title = 首选项
preferences-categories = 分类
preferences-category-interface = 界面
preferences-category-performance = 性能
preferences-category-defaults = 默认设置
preferences-category-advanced = 高级

# 界面首选项
preferences-interface-title = 界面设置
preferences-theme = 主题
preferences-language = 语言
preferences-ui-scale = 界面缩放
preferences-font-size = 字体大小
preferences-font-size-placeholder = 12
preferences-show-tooltips = 显示工具提示
preferences-show-rulers = 显示标尺
preferences-show-grid = 显示网格
preferences-show-status-bar = 显示状态栏

# 性能首选项
preferences-performance-title = 性能设置
preferences-memory-limit = 内存限制
preferences-cache-size = 缓存大小
preferences-worker-threads = 工作线程数
preferences-tile-size = 瓦片大小
preferences-gpu-acceleration = GPU 加速
preferences-multithreaded-rendering = 多线程渲染

# 默认首选项
preferences-defaults-title = 默认设置
preferences-default-tool = 默认工具
preferences-default-format = 默认图像格式
preferences-auto-save = 自动保存间隔
preferences-max-undo = 最大撤销历史
preferences-confirm-close = 关闭未保存文档前确认
preferences-remember-window = 记住窗口状态
preferences-disabled = 已禁用

# 高级首选项
preferences-advanced-title = 高级设置
preferences-debug-mode = 调试模式
preferences-log-level = 日志级别
preferences-experimental = 实验性功能
preferences-plugin-dir = 插件目录
preferences-plugin-dir-placeholder = /插件/路径
preferences-no-plugin-dir = 未设置插件目录
preferences-crash-reporting = 崩溃报告
preferences-telemetry = 遥测

# 首选项对话框按钮
preferences-apply = 应用
preferences-cancel = 取消
preferences-reset = 重置为默认值

# 常用术语
untitled = 未命名

# 画布消息
canvas-no-document = 无文档打开
canvas-click-open = 点击"打开"加载图像
