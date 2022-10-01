/// Grrs 命令行程序例子
/// 使用方法：
/// 
/// 
/// 
/// 
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = "命令描述：")]
struct Cli {
    #[clap(value_parser)]
    pattern: String,
    #[clap(short, long, value_parser, value_name = "FILE")]
    path: Option<std::path::PathBuf>,
    #[clap(short, long, action = clap::ArgAction::Count, help = "调试")]
    debug: u8,
}


fn main() {
    let args = Cli::parse();
}
