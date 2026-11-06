# Phi TK CLI Edition
![Winamin最強! Winamin最高!](arts/phi-tk-clilogo.png) 

Phi-TK 是一个基于 Tauri + Vue 3 的谱面渲染工具。
<!---Rust小螃蟹好可爱wwwwwwww Tauri何意味神秘图标--->
Phi-TK CLI 是一个基于 Phi-TK Tauri部分的命令行渲染工具，不保证和 Phi-TK 一样多功能，也不保证和 Phi-TK 一样渲染效率

<img src="arts/logos.svg" width="120" height="60"></img>

## 下载 / Download

<div style="transform: scale(1.7);transform-origin: 0 20%;">

[![Download](https://img.shields.io/badge/Download-Latest_Release-blue?style=for-the-badge)](https://github.com/CMDBookDevelope/Phi-TK-cli/releases)


</div>


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

`-r/--resolution`、`-R/--result` 这类简写遵循「更常用的参数用小写、同类里次常用的用大写」：
小写留给高频选项，大写留给同一字母的次要含义。

| 参数 | 说明 | 可用值 | 默认值 |
|------|--------|------|-----------|
| `-i, --input` | 谱面文件路径 | FilePath | 必填 |
| `-o, --output` | 输出视频路径 | FilePath | output.mp4 |
| `-r, --resolution` | 分辨率 | 宽x高 | 1920x1440 |
| `-c, --crf` | 渲染质量（CRF/QP） | 1-51 | 20 |
| `-f, --fps` | 帧率 | 整数 | 30 |
| `-d, --dim` | 背景压暗百分比 | 0-100 | 30 |
| `-R, --result` | 显示结算界面（原 `--finish`） | 布尔 | false |
| `-l, --load` | 显示加载界面 | 布尔 | false |
| `--ending` | 结算画面时长（秒） | 浮点 | 有 `--result` 时 3 |
| `-n, --name` | 玩家名 | 文本 | 空 |
| `--rks` | 玩家 rks | 浮点 | 6.16 |
| `--avatar` | 玩家头像路径 | FilePath | 无 |
| `--combo` | Combo 文本 | 文本 | PHI-TK POWERED |
| `-w, --watermark` | 水印文本 | 文本 | 空 |
| `--ui` | 显示的 UI 元素（逗号分隔；未列出的隐藏） | score,combo,name,level,line,pb,pause / all / none | 全部 |
| `--hide-ui` | 隐藏的 UI 元素（在 `--ui` 之后应用） | 同上 | 无 |
| `--bar` | 显示进度条 | 布尔 | false |
| `--score-aim` | 显示目标分数 | 布尔 | false |
| `--chart-debug` | 显示谱面调试信息 | 布尔 | false |
| `--flid-x` | 水平镜像谱面 | 布尔 | false |
| `--chart-ratio` | 谱面拉伸比例 | 浮点 | 1.0 |
| `--background` | 显示谱面插画背景 | 布尔 | false |
| `--respack` | 资源包路径 | DirectoryPath | 无 |
| `-s, --speed` | 谱面流速倍率 | 浮点 | 1.0 |
| `--note-scale` | 音符缩放 | 浮点 | 1.0 |
| `--volume-music` | 音乐音量 | 0-1 | 1.0 |
| `--volume-sfx` | 音效音量 | 0-1 | 1.0 |
| `--fxaa` | 开启 FXAA | 布尔 | false |
| `--no-particle` | 关闭粒子效果 | 布尔 | false |
| `--no-effect` | 关闭背景特效 | 布尔 | false |
| `--no-double-hint` | 关闭双押提示线 | 布尔 | false |
| `--aggressive` | Aggressive 模式 | 布尔 | false |
| `--hand-split` | 手序拆分 | 布尔 | false |
| `--sample-count` | MSAA 采样数 | 1 / 2 / 4 / 8 … | 4 |
| `--codec` | 视频编码 | h264 / hevc / av1 | h264 |
| `-e, --encoder` | 编码器 | auto / nvenc / qsv / amf / vulkan / dx12 / cpu | auto |
| `-p, --preset` | FFmpeg 预设 | veryfast…veryslow | medium |
| `--format` | 封装格式 | mp4 / mov | mp4 |
| `--bitrate-control` | 码率控制 | CRF / CBR | CRF |
| `-b, --bitrate` | 显式码率（覆盖 `--crf`） | 如 8000k | 无 |
| `--no-hw-accel` | 关闭硬件加速 | 布尔 | false |
| `--ffmpeg-thread` | FFmpeg 多线程 | 布尔 | false |
| `--target-audio` | 音频采样率 | 整数 | 48000 |
| `--audio-format` | 音频格式 | flac / aac / opus … | flac |
| `--audio-bit` | 音频位深 | 16 / 24 / 32 | 无 |
| `--audio-delay-frames` | 音频延迟（帧） | 整数 | 0 |
| `--assets` | 资源文件位置 | DirectoryPath | 自动探测（旧版安装位置为 /usr/lib/ptkc-assets） |

### 常用示例

```bash
# 1080p60，显示结算界面
phi-tk-cli -i song.pez -o song.mp4 -r 1920x1080 -f 60 -R

# 只保留分数和连击，隐藏其余 UI，压暗 50%
phi-tk-cli -i song.pez --dim 50 --ui score,combo

# 指定玩家信息与手序拆分，走 NVENC
phi-tk-cli -i song.pez -n "Winamin" --rks 16.2 --hand-split -e nvenc -p slow

# 无硬件加速，CBR 8Mbps
phi-tk-cli -i song.pez --no-hw-accel --bitrate-control CBR -b 8000k

# 不显示加载动画、显示结算 5 秒
phi-tk-cli -i song.pez --result --ending 5
```

~~*摆烂了。。。申请查看[源代码](src-tauri/src/render.rs#L1939-L1975 "1939工作室快让美芬死一死。")*~~

### 运行
```bash
./phi-tk-cli #后面添加渲染参数
```

> **一个进程只渲染一个任务。** 没有内置队列/任务管理器，跑完即退出；
> 要并行渲染多个谱面，直接起多个 `phi-tk-cli` 进程即可。

## 项目结构

```
Phi-TK-cli/
├── src-tauri/             # Tauri 后端源码
│   ├── src/               # Rust 源码
│   ├── assets/            # 应用资源
│   └── icons/             # 应用图标
└── arts/                  # 文档图片
```

## 致谢 / Acknowledgements

- **噪域（`blockAreaList`）支持**来自 [Ameiro-sudo/Phi-TK](https://github.com/Ameiro-sudo/Phi-TK)（`noise-build` 分支），
  经 [Phi-TKC](https://github.com/CMDBookDevelope/Phi-TKC) 移植为后端无关（Vulkan 兼容）的实现。
  感谢该 fork 的修改者对 Phigros 9th-chapter 噪域所做的逆向与实现工作。

## 许可证

本项目采用 GNU v3.0 开源许可证，详见 LICENSE 文件。

## 相关链接
*请给这些仓库点一个免费的Star✨!*

- [Phi-TK](https://github.com/Winamin/Phi-TK "🐲")
- [Phi-TK-render-lib](https://github.com/Winamin/Phi-TK-render-lib "龍龍龍")
- [Phi-TKC-render-lib (CNMDBook Ver.)](https://github.com/CMDBookDevelope/Phi-TKC-render-lib "🥬")
- [Macroquad CNMDBookVer.](https://github.com/CMDBookDevelope/prpr-macroquad "FULL HEADLESS SUPPORT.")
- [Miniquad CNMDBookVer.](https://github.com/CMDBookDevelope "HEADLESS nb")
- [Tauri](https://tauri.app/ "😱")
