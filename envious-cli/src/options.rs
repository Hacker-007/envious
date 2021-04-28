use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "envious")]
pub struct Options {
    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,
    #[structopt(short, long)]
    pub tui: bool
}
