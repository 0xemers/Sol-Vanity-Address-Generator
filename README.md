# Solana Vanity Address Generator 🔍
（傻逼被三线 bot 盗取生成定制钱包里的所有 sol 后决定不能偷懒自己写一个本地的 T.T） 

**生成定制化 Solana 地址的安全工具 | 支持多线程/GPU加速**

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 安装指南 📦

### 前置要求
- Rust 1.65+ (`rustup install stable`)
- Cargo 包管理工具
- Apple Silicon 设备推荐 macOS 13+

### 快速安装
```bash
git clone https://github.com/solana-labs/solana.git

# 编译优化版 (Apple Silicon)
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=yes" cargo build --release

# 验证安装
./target/release/solana-vanity --version
```

## 🚀 使用方法

### 基础命令
```bash
cargo run --release -- --prefix YOU --suffix XX
```
