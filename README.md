# Phi TK CLI Edition
![Winamin最強! Winamin最高!](arts/phi-tk-clilogo.png) 

Phi-TK 是一个基于 Tauri + Vue 3 的谱面渲染工具。<img src="https://rust-lang.org/static/images/rust-logo-blk.svg" width="50" height="50" /><svg width="50" height="50" viewBox="0 0 432 137" fill="none" class="lp-tauri-logo dark"><path d="M85.76 49.92A13.1 13.1 0 0 1 72.62 63a13.1 13.1 0 0 1-13.14-13.08 13.1 13.1 0 0 1 13.14-13.07 13.1 13.1 0 0 1 13.14 13.07Z" fill="#FFC131"></path><path d="M63.74 86.66A13.1 13.1 0 0 1 50.6 99.73a13.1 13.1 0 0 1-13.15-13.07A13.1 13.1 0 0 1 50.6 73.58a13.1 13.1 0 0 1 13.14 13.08Z" fill="#24C8DB"></path><path d="M40.58 38.15C17.9 42.77.84 62.73.84 86.65c0 27.35 22.28 49.51 49.76 49.51a49.75 49.75 0 0 0 47.51-34.78 57.88 57.88 0 0 1-19.03 5.55A35.02 35.02 0 0 1 50.6 121.5a34.93 34.93 0 0 1-35.02-34.84 34.87 34.87 0 0 1 22.97-32.73 34.8 34.8 0 0 1 2.03-15.78Z" fill="#24C8DB"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M82.64 98.43c22.67-4.61 39.74-24.58 39.74-48.5C122.38 22.57 100.1.41 72.62.41A49.75 49.75 0 0 0 25.11 35.2a57.87 57.87 0 0 1 19.03-5.55 35.03 35.03 0 0 1 28.48-14.57 34.93 34.93 0 0 1 35.02 34.84 34.87 34.87 0 0 1-22.97 32.73 34.8 34.8 0 0 1-2.03 15.78Z" fill="#FFC131"></path></svg>
<!---![Rust小螃蟹好可爱wwwwwwww](https://rust-lang.org/static/images/rust-logo-blk.svg) ![Tauri何意味神秘图标](https://tauri.app/_astro/logo.BQPqkdSq.svg)--->
Phi-TK CLI 是一个基于 Phi-TK Tauri部分的命令行渲染工具，不保证和 Phi-TK 一样多功能，也不保证和 Phi-TK 一样渲染效率
*部分代码由AI编写/修改*

## Phi-TK 功能特性: 详见[原仓库](https://github.com/Winamin/Phi-TK "？！强强 ！？")

## Phi-TK CLI 支持的功能
- 支持WAV无损音频
- CRF码率控制
- 支持Vulkan加速渲染

### 通用要求
- Rust 1.60+

### 平台特定要求
- **Linux**: Debian 已测试!
- **Windows** 未测试...?
- **MacOS** 未测试...?

## 安装步骤

### 1. 安装 Rust
从 [Rust 官网](https://rustup.rs/) 下载并安装 Rust。

### 2. 克隆仓库
```bash
git clone https://github.com/CMDBookDevelope/Phi-TK-cli.git
#镜像加速: git clone https://gh-proxy.org/https://github.com/CMDBookDevelope/Phi-TK-cli.git
cd Phi-TK-cli/src-tauri
```

### 3. 更新项目依赖
```bash
cargo update
```

### 4. 移动 assets 到 ptkc-assets
```bash
cp -r src-tauri/assets /usr/lib/ptkc-assets
```

## 编译构建

### 生产构建
```bash
cargo build --release --bin phi-tk-cli
```

构建完成后，可执行文件将位于：
- Linux: `src-tauri/target/release/phi-tk`
  
## 使用说明
*需要搭配修改过的，贴合实际用户环境的wrapper.sh确保在无头环境运行*

###首次运行###
```bash
cd .. #请在Phi-TK-cli目录执行
chmod +x wrapper.sh
```

###常规运行###
```bash
./wrapper.sh #后面添加渲染染变量
```

## 项目结构

```
Phi-TK-cli/
├── src-tauri/             # Tauri 后端源码
│   ├── src/               # Rust 源码
│   ├── assets/            # 应用资源
│   └── icons/             # 应用图标
├── arts/                  # 文档图片
└── wrapper.sh             # 运行脚本
```

## 许可证

本项目采用 GNU v3.0 开源许可证，详见 LICENSE 文件。

## 相关链接

- [Phi-TK](https://github.com/Winamin/Phi-TK "🐲")
- [Phi-TK-render-lib](https://github.com/Winamin/Phi-TK-render-lib "龍龍龍")
- [Tauri](https://tauri.app/ "😱")
