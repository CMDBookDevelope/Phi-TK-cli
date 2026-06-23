# Phi TK CLI Edition
![render](arts/phi-tklogo.png)

Phi-TK 是一个基于 Tauri + Vue 3 的谱面渲染工具。
Phi-TK CLI 是一个基于 Phi-TK 的命令行渲染工具，不保证和 Phi-TK 一样多功能，也不保证和 Phi-TK 一样渲染效率
*部分代码由AI修改/编写*

## Phi-TK 功能特性: 详见[原仓库](https://github.com/Winamin/Phi-TK "？！强强 ！？")

## Phi-TK CLI 支持的功能
- 支持WAV无损音频
- CRF码率控制
- 支持Vulkan加速渲染

### 通用要求
- Rust 1.60+

### 平台特定要求
- **Linux**: 支持现代桌面发行版

## 安装步骤

### 1. 安装依赖

#### 安装 Rust
从 [Rust 官网](https://rustup.rs/) 下载并安装 Rust。

### 2. 克隆仓库主要代码 src-tauri 分支
```bash
git clone https://github.com/CMDBookDevelope/Phi-TK-cli/tree/main/src-tauri.git
cd src-tauri
```

### 3. 更新项目依赖
```bash
cargo update
```

## 编译构建

### 开发模式
```bash
pnpm tauri dev
```

### 生产构建
```bash
pnpm tauri build
```

构建完成后，可执行文件将位于：
- Windows: `src-tauri/target/release/Phi-TK.exe`
- Linux: `src-tauri/target/release/phi-tk`
- macOS: `src-tauri/target/release/bundle/macos/`

## 使用说明

1. 启动应用程序
2. 通过界面导入 Phigros 谱面文件
3. 配置渲染参数（音频质量、视频码率等）
4. 开始渲染过程
5. 导出渲染结果

## 项目结构

```
Phi-TK/
├── src/                    # Vue 前端源码
│   ├── components/         # Vue 组件
│   ├── router/            # 路由配置
│   └── assets/            # 静态资源
├── src-tauri/             # Tauri 后端源码
│   ├── src/               # Rust 源码
│   ├── assets/            # 应用资源
│   └── icons/             # 应用图标
└── arts/                  # 文档图片
```

## 开发脚本

- `pnpm dev` - 启动前端开发服务器
- `pnpm build` - 构建前端
- `pnpm type-check` - 类型检查
- `pnpm lint` - 代码检查和自动修复
- `pnpm prettier` - 代码格式化

## 许可证

本项目采用开源许可证，详见 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目。

## 相关链接

- [Phi-TK-render-lib](https://github.com/Winamin/Phi-TK-render-lib)
- [Tauri](https://tauri.app/)
- [Vue 3](https://vuejs.org/)
