# epub-gen

一个将图像压缩包转换为 EPUB 电子书的 Rust 工具。

## 项目概述

`epub-gen` 是一个命令行工具，可以将包含图像的压缩包（如 ZIP 文件）转换为 EPUB 格式的电子书。该工具支持批量处理图像、图像压缩和自定义输出质量。

## 功能特性

- 从 ZIP 压缩包中提取图像
- 支持多种图像格式（JPG、PNG、WebP 等）
- 图像批量压缩，支持自定义质量
- 生成符合 EPUB 标准的电子书
- 显示处理进度
- 可选择是否保持原始文件名顺序

## 依赖库

本项目使用了以下 Rust 库：

- [clap](https://crates.io/crates/clap) - 命令行参数解析
- [zip](https://crates.io/crates/zip) - ZIP 文件处理
- [tempfile](https://crates.io/crates/tempfile) - 临时文件管理
- [rust-embed](https://crates.io/crates/rust-embed) - 静态资源嵌入
- [rayon](https://crates.io/crates/rayon) - 并行计算
- [epub-builder](https://crates.io/crates/epub-builder) - EPUB 文件生成
- [image](https://crates.io/crates/image) - 图像处理
- [indicatif](https://crates.io/crates/indicatif) - 进度条显示

## 构建方法

### 环境要求
- Rust 1.70 或更高版本
- Cargo

### 构建步骤
```bash
# 克隆或下载项目
git clone <repository-url>
cd epub-gen

# 构建项目
cargo build --release

# 运行测试
cargo test
```

## 使用方法

### 基本用法
```bash
# 基本转换
cargo run -- -i input.zip -o output.epub

# 指定图像质量（1-100，默认为80）
cargo run -- -i input.zip -o output.epub --quality 90

# 保持原始文件名顺序
cargo run -- -i input.zip -o output.epub --preserve-order

# 同时使用多个选项
cargo run -- -i input.zip -o output.epub --quality 85 --preserve-order
```

### 命令行参数

| 参数 | 描述 |
|------|------|
| `-i, --input` | 输入的压缩包路径（如 images.zip） |
| `-o, --output` | 输出的 EPUB 文件路径（默认为 output.epub） |
| `--quality` | 图片质量（1-100，默认为80） |
| `--preserve-order` | 是否保留原始文件名顺序（默认为 false） |

### 示例
```bash
# 将 images.zip 转换为 my_book.epub，质量为 85
cargo run -- -i images.zip -o my_book.epub --quality 85

# 处理完成后保留原始文件名顺序
cargo run -- -i comic.zip -o comic.epub --preserve-order
```

## 项目结构

```
src/
├── cli.rs          # 命令行参数解析
├── compress.rs     # 图像压缩功能
├── epub.rs         # EPUB 生成功能
├── extract.rs      # 图像提取功能
├── main.rs         # 程序入口
└── pipeline.rs     # 主处理流程
```

## 工作流程

1. **提取** - 从 ZIP 压缩包中提取图像文件
2. **压缩** - 将图像转换为 JPG 格式并按指定质量压缩
3. **生成** - 将处理后的图像生成 EPUB 电子书

## 技术特点

- **并行处理** - 使用 rayon 实现图像压缩的并行处理
- **进度显示** - 实时显示提取和压缩进度
- **临时文件管理** - 使用 tempfile 确保临时文件正确清理
- **模板嵌入** - EPUB 模板文件直接嵌入到二进制文件中

## 免责协议

本软件按"现状"提供，不提供任何明示或暗示的保证。作者不对使用本软件造成的任何直接或间接损失承担责任，包括但不限于数据丢失、系统故障或任何其他问题。

使用本软件即表示您接受以下条款：
1. 软件仅供学习和参考使用
2. 使用者应对自己的使用行为负责
3. 作者不承担任何因使用本软件而产生的风险和责任
4. 在任何情况下，作者均不对因使用或无法使用本软件而造成的任何损害承担责任

## 许可证

 [MIT License](./LICENSE)
