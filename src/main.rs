use structopt::StructOpt;
use std::{
    error::Error,
};

mod cli;
mod split;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::PrestoParser::from_args();
    match opt {
        cli::PrestoParser::Split(e) => {
            split::split(&e.path_file, &e.dst)?;
        }
    }

    Ok(())
}
