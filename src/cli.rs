use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "presto-parser", about = "A collection of tools for the Presto corpus")]
/// Holds every command that is callable by the `presto-parser` command.
pub enum PrestoParser {
    #[structopt(about = "Split the main Presto file into sub-files")]
    Split(Split),
}

#[derive(StructOpt)]
pub struct Split {
    #[structopt(parse(from_os_str), help = "path to presto file")]
    pub path_file: PathBuf,
    #[structopt(parse(from_os_str), help = "destination folder")]
    pub dst: PathBuf,
}
