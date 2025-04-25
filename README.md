# Solana Vanity Address Generator 🔍
（被三线 bot 盗取生成定制钱包里的所有 sol 后决定自己写一个本地的 T.T） 

**生成定制化 Solana 地址的安全工具 | 支持多线程/GPU加速**

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 功能特性 🚀

- ​**定制地址格式**：自由指定地址前N位和后M位字符
- ​**极速生成**：Apple Silicon M3 Max 实测 1.4M keys/s
- ​**多层安全**：
  - 内存零残留技术
  - 离线生成模式
  - 私钥分段显示
- ​**跨平台支持**：macOS/Linux/Windows (WSL2)
- ​**智能优化**：
  - 自动CPU线程调度
  - Metal/GPU加速（macOS）
  - SIMD指令优化

## 安装指南 📦

### 前置要求
- Rust 1.65+ (`rustup install stable`)
- Cargo 包管理工具
- Apple Silicon 设备推荐 macOS 13+

### 快速安装
```bash
git clone https://github.com/yourusername/solana-vanity-generator.git
cd solana-vanity-generator

# 编译优化版 (Apple Silicon)
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release

# 验证安装
./target/release/solana-vanity --version

## 🚀 使用方法

### 基础命令
```bash
./solana-vanity --prefix SOL --suffix 88 --threads 14
```

### 参数说明
| 参数               | 必选 | 默认值 | 示例值       | 描述                     |
|--------------------|------|--------|--------------|--------------------------|
| `--prefix`         | 是   | 无     | `SOL`        | 地址开头字符 (1-4位)     |
| `--suffix`         | 否   | 无     | `88`         | 地址结尾字符 (1-2位)     |
| `--threads`        | 否   | 自动   | `14`         | CPU 线程数 (M3 Max用14)  |
| `--gpu`            | 否   | `off`  | `on`         | 启用 GPU 加速 (需NVIDIA) |
| `--timeout`        | 否   | `3600` | `7200`       | 最长运行时间 (秒)        |

### 完整示例
```bash
# 生成以 SOL 开头、88 结尾的地址，使用 16 线程
RUST_LOG=info ./solana-vanity \
  --prefix SOL \
  --suffix 88 \
  --threads 16 \
  --gpu on \
  --timeout 10800
```

## 📝 输出示例

### 成功匹配
```text
[2024-03-01 14:23:18] INFO  - 初始化完成 | 线程: 14 | GPU: Enabled (CUDA)
[2024-03-01 14:23:18] INFO  - 目标格式: SOL...88
[2024-03-01 14:26:42] SUCCESS - 找到匹配地址! 
═══════════════════════════════════════════
地址: SOLAr12BqMu4JqYdGn5YjKtY7Vb88
私钥: 3gh7z9VkF...Wx5k9j2 (已加密存储)
═══════════════════════════════════════════
总尝试次数: 238,420,119 | 耗时: 3m24s
平均速度: 1.17M keys/s
```

### 运行中状态
```text
[2024-03-01 14:23:19] STATUS - 尝试次数: 12.4M | 速度: 1.21M/s 
[2024-03-01 14:23:20] STATUS - 当前候选地址示例:
- SOL9kX...Lp88
- SOLqTw...mY88 
- SOLVrG...zN88
```

## ⚠️ 安全提示
```bash
# 生成后立即操作
shred -u secure_key.bin     # 安全删除密钥文件
systemctl restart systemd-journald  # 清除系统日志
```
