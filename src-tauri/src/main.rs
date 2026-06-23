mod render;
use clap::Parser;
use render::{render_cli, CliArgs};

#[macroquad::main("Phi-TK")]
async fn main() {
    // 先解析命令行参数
    let args = CliArgs::parse();

    // 设置资源目录
    macroquad::file::set_pc_assets_folder(&args.assets);

    // 手动创建一个 Tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 在 runtime 里运行异步渲染逻辑
    rt.block_on(async {
        if let Err(e) = render_cli(args).await {
            eprintln!("Error: {:?}", e);
        }
    });
}
