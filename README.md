# aimvp - AI Markdown Visual Presentation

A comprehensive formatting tool specifically designed for .md and .txt files that provides intelligent Markdown document standardization and automatic text spacing optimization between CJK characters and alphanumeric symbols.

## Features

- 🎯 **Smart Formatting**: Automated Markdown document standardization processing
- ⚡ **High Performance**: Rust-based parallel text processing engine
- 🔤 **CJK Support**: Professional spacing adjustment between CJK characters and alphanumeric symbols
- 📝 **List Optimization**: Unified list symbols and spacing formatting
- 🔧 **Tool Integration**: Support for pangu, prettier and other tool chains
- 💪 **Batch Processing**: Efficient handling of large files

> **CJK 字符說明**  
> CJK 是 Chinese, Japanese, Korean 的縮寫，指中文、日文、韓文三種東亞語言：
> - **中文 (Chinese)**: 使用漢字（繁體字、簡體字）
> - **日文 (Japanese)**: 混合使用漢字、平假名（ひらがな）、片假名（カタカナ）
> - **韓文 (Korean)**: 主要使用韓文字母（한글/Hangul），偶爾使用漢字
>
> **Unicode 範圍**  
> - CJK 統一漢字：U+4E00-U+9FFF
> - CJK 擴展 A：U+3400-U+4DBF  
> - 韓文字母：U+AC00-U+D7AF
> - 日文假名：U+3040-U+309F, U+30A0-U+30FF

## Installation

### Homebrew (Recommended)

```bash
brew install monshakeys/tap/aimvp
```

### Alternative Methods

If you prefer not to use Homebrew, you can install using one of these methods:

#### Build from Source

```bash
# Clone the repository
git clone <repo-url>
cd aimvp

# For development - fast compilation
cargo build
./target/debug/aimvp

# For production - optimized performance
cargo build --release
./target/release/aimvp

# Or run directly without building executable
cargo run              # debug mode
cargo run --release    # release mode

# Install to system (optional)
cargo install --path .
```

#### Using Cargo

If you have Rust installed:

```bash
cargo install --git <repo-url>
```

#### Download Binary

Download the latest binary from the [releases page](https://github.com/monshakeys/aimvp/releases) and place it in your `$PATH`.

## Usage

```bash
# Show help message
aimvp --help

# Basic usage - format a single file
aimvp document.md

# Format multiple files
aimvp *.md
```

## Project Structure

```
aimvp/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── test_pangu.py
│   └── test_spacing.rs
├── benches/
│   └── spacing.rs
├── aimvp.sh
├── md-rules.md
├── validations.json
└── TODO.md
```