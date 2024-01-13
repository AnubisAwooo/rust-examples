use clap::Parser;

/// 从文件中过滤关键词
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 关键词, 支持正则
    // #[arg(short, long)]
    pub word: String,

    /// 目标文件
    // #[arg(short, long)]
    pub files: Option<Vec<String>>,
}
