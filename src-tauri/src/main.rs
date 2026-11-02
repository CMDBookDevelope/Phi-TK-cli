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

fn main() {
    // Parse before creating the window so `--help` / `--version` work over SSH and
    // on machines without a display.
    let args = CliArgs::parse();

    macroquad::Window::from_config(window_conf(), async move {
        macroquad::file::set_pc_assets_folder(&args.assets);

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = render_cli(args).await {
                eprintln!("(｀皿´＃). Error: {:?}", e);
                std::process::exit(1);
            }
            // Headless CLI: nothing will pump the window after the render ends.
            std::process::exit(0);
        });
    });
}
