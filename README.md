# Phi TK CLI Edition
![Winamin最強! Winamin最高!](arts/phi-tk-clilogo.png) 

Phi-TK 是一个基于 Tauri + Vue 3 的谱面渲染工具。
<!---Rust小螃蟹好可爱wwwwwwww Tauri何意味神秘图标--->
Phi-TK CLI 是一个基于 Phi-TK Tauri部分的命令行渲染工具，不保证和 Phi-TK 一样多功能，也不保证和 Phi-TK 一样渲染效率

<img src="arts/logos.svg" width="120" height="60"></img>

<small>*部分代码由AI编写/修改*</small>

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

**command usage:**
<!---
```bash
phi-tk-cli # or wrapper.sh \
--input your_chart.pez [required] \
--output output.mp4 # or .mov|.mkv... [optional] \
--resolution 1920x1440 # default [optional] \
--crf 20 # default [optional] \
--fps 30 # dafault [optional] \
--dark 40 # default Background picture brightness [optional] \
--load # default=off loading screen [optional] \
--finish # default=off result screen [optional] \
--assets # assets folder [optional] \
```
--->
### 自定义参数
| 参数 | 说明 | 可用值 | 默认值 |
|------|--------|------|-----------|
| `-i, --input` | 谱面文件路径 | FilePath | 自己填。。。我怎么知道你要渲染什么，，， |
| `-o, --output` | 输出视频路径 | FilePath | output.mp4 |
| `-r, --resolution` | 分辨率 | 整数x整数 | 1920x1440 |
| `-c, --crf` | 渲染质量 | 整数 | 20 |
| `--fps` | 帧率 | 整数 | 30 |
| `-d, --dark` | 背景亮度 | 0-100 | 30 |
| `-l, --load` | 显示加载界面 | 布尔 | False |
| `--finish` | 显示结算界面 | 布尔 | False |
| `-a, --assets` | 资源文件位置 | DirectoryPath | /usr/lib/ptkc-assets |

~~*摆烂了。。。申请查看[源代码](src-tauri/src/render.rs#L1939-L1975 "1939工作室快让美芬死一死。")*~~

###常规运行###
```bash
./phi-tk-cli #后面添加渲染染变量
```

## 项目结构

```
Phi-TK-cli/
├── src-tauri/             # Tauri 后端源码
│   ├── src/               # Rust 源码
│   ├── assets/            # 应用资源
│   └── icons/             # 应用图标
└── arts/                  # 文档图片
```

## 许可证

本项目采用 GNU v3.0 开源许可证，详见 LICENSE 文件。

## 相关链接
*请给这些仓库点一个免费的Star✨!*

- [Phi-TK](https://github.com/Winamin/Phi-TK "🐲")
- [Phi-TK-render-lib](https://github.com/Winamin/Phi-TK-render-lib "龍龍龍")
- [Tauri](https://tauri.app/ "😱")
