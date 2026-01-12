# Prism

现代化多媒体数据采集桌面应用，基于 **Rust + React + Tauri** 构建。

## 目标

实现一个支持不同媒体类型的资源聚合app

1. 支持不同媒体类型，如漫画, 小说, 电影, 音乐等
2. 支持用户自定义爬虫规则，可以爬取指定网站的资源
3. 支持资源的聚合搜索(多个相同资源类型的网站搜索)、资源发现功能
4. 支持资源的下载功能
5. 支持资源的详情展示, 列表展示, 内容展示即观看
6. 完善的爬虫实现支持在目前Tauri(webview) + rust + rs最大能支持的爬虫实现

## 环境要求

- Rust 1.82+ (需要 nightly格式化)
- Node.js 18+
- Bun 1.0+

## 快速启动

```bash
git clone https://github.com/Wuhy5/prism.git
cd prism
bun install
bun tauri dev
```

## 开发命令

```bash
# 开发模式
bun dev                # 前端开发服务器
bun tauri dev          # Tauri 开发模式

# 构建
bun tauri build        # 构建完整应用

# 代码质量
cargo +nightly fmt     # Rust 代码格式化
cargo clippy           # Rust 代码检查
bun lint               # 前端代码检查
```

## 文档

- [开发规范](AGENTS.md) - AI 开发和编码规范
- [Tauri 集成](docs/tauri.md)
- [UI 设计](docs/shadcn-ui.md)

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件
