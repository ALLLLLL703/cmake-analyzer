use std::path::PathBuf;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Inspect cmake lexer output")]
pub struct Args {
    pub file: Option<PathBuf>,

    #[arg(long, conflicts_with = "file")]
    pub text: Option<String>,

    #[arg(long)]
    pub color: bool,
}
