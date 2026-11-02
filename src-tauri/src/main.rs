// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use phi_tk_mini_lib::{render_cli, CliArgs};
use macroquad::window::Conf;

// 定义返回 Conf 的函数，设置 headless 为 true
fn window_conf() -> Conf {
    Conf {
        window_title: "Phi-TK CLI看到我说明失败了。。。".to_string(),
        // Vulkan 渲染需要真实的 Vulkan 上下文，miniquad 的 headless 路径只有
        // EGL/OpenGL。服务器上可用 `xvfb-run` 提供虚拟显示。
        headless: false,
        platform: macroquad::miniquad::conf::Platform {
            rendering_backend: macroquad::miniquad::conf::RenderingBackend::Vulkan,
            ..Default::default()
        },
        ..Default::default()
    }
}

// 使用 window_conf 函数返回 Conf，设置 headless: true
#[macroquad::main(window_conf)]
async fn main() {
    let args = CliArgs::parse();
    macroquad::file::set_pc_assets_folder(&args.assets);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = render_cli(args).await {
            eprintln!("(｀皿´＃). Error: {:?}", e);
        }
    });
}
