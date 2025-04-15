use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["url", "path"]),
))]
#[command(group(
    ArgGroup::new("delivery")
        .required(false)
        .args(&["buffer", "output"]),
))]
pub struct Args {
    #[arg(long)]
    pub url: Option<String>,

    #[arg(long)]
    pub path: Option<String>,

    #[arg(long)]
    pub output: Option<String>,

    #[arg(long, default_value_t = false)]
    pub buffer: bool,

    #[arg(long)]
    pub resize: bool,

    #[arg(long, requires = "resize")]
    pub width: Option<u32>,

    #[arg(long, requires = "resize")]
    pub height: Option<u32>,
}
