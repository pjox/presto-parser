use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "presto-parser",
    about = "A collection of tools for the Presto corpus"
)]
/// Holds every command that is callable by the `presto-parser` command.
pub enum PrestoParser {
    #[structopt(about = "Cut the main Presto file into sub-files")]
    Cut(Cut),
    #[structopt(about = "Parse main Presto file into sub-files")]
    Parse(Parse),
    #[structopt(about = "Correct main Presto file into sub-files")]
    Correct(Correct),
}

#[derive(StructOpt)]
pub struct Cut {
    #[structopt(parse(from_os_str), help = "path to presto file")]
    pub path_file: PathBuf,
    #[structopt(parse(from_os_str), help = "destination folder")]
    pub dst: PathBuf,
}

#[derive(StructOpt)]
pub struct Parse {
    #[structopt(parse(from_os_str), help = "path to presto file")]
    pub path_file: PathBuf,
}

#[derive(StructOpt)]
pub struct Correct {
    #[structopt(parse(from_os_str), help = "path to presto file")]
    pub path_file: PathBuf,
}
