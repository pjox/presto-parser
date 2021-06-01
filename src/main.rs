use std::error::Error;
use structopt::StructOpt;

mod cli;
mod cut;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::PrestoParser::from_args();
    match opt {
        cli::PrestoParser::Cut(e) => {
            cut::cut(&e.path_file, &e.dst)?;
        }
    }

    Ok(())
}
