mod render;
use clap::Parser;
use render::{render_cli, CliArgs};
use macroquad::window::Conf;

// 定义返回 Conf 的函数，设置 headless 为 true
fn window_conf() -> Conf {
    Conf {
        window_title: "Phi-TK CLI看到我说明失败了。。。".to_string(),
        headless: true,          // 关键：启用无头模式
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
