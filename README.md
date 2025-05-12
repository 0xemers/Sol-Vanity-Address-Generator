# Solana 自定义地址生成器
(ps. 懒b被三线bot盗完定制钱包里所有sol后决定自己写一个本地的 T.T）)
这是一个用 Rust 编写的 Solana 自定义地址生成器，可以生成以指定前缀开头和/或以指定后缀结尾的 Solana 地址。该工具利用多线程处理和 Apple GPU (Metal) 加速搜索过程。

## 功能特点

- 可定制地址前缀和后缀
- 多线程 CPU 并行计算
- Apple Metal GPU 加速支持
- 实时显示搜索速度和进度
- 支持所有 Base58 合法字符

## 适用环境

- **操作系统**：
  - macOS (支持 Apple Silicon/Intel)
  - Linux
  - Windows

- **硬件要求**：
  - 基本功能：任何支持 Rust 的计算机
  - GPU 加速：搭载 Apple M1/M2/M3 系列芯片或支持 Metal 的 Mac

## 安装要求

- Rust 和 Cargo (建议使用最新的稳定版)
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  source $HOME/.cargo/env
  rustup component add rustfmt
  rustup update
  ```

## 快速开始
1. 克隆 Solana 仓库
  ```bash
  git clone https://github.com/solana-labs/solana.git
  ```
2. 克隆本仓库
  ```bash
  git clone https://github.com/basedeliox/Sol-Vanity-Address-Generator.git
  cd solana-vanity
  ```
3. 编译和运行
  ```bash
  cargo clean
  cargo build --release
  cargo run --release -- --prefix YOU --suffix XX --threads XX
  ```

## 示例
```bash
git clone https://github.com/solana-labs/solana.git
git clone https://github.com/yourusername/solana-vanity.git
cd solana-vanity
cargo build --release
# 搜索以"ETH"开头且以"UP"结尾的地址，并使用24个线程搜索
cargo run --release -- -prefix ETH -suffix UP --threads 24
```







