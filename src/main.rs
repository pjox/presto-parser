use std::error::Error;
use structopt::StructOpt;

mod cli;
mod cut;
mod parse;
mod correct;
mod tags;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = cli::PrestoParser::from_args();
    match opt {
        cli::PrestoParser::Cut(e) => {
            cut::cut(&e.path_file, &e.dst)?;
        }
        cli::PrestoParser::Parse(e) => {
            parse::parse(&e.path_file)?;
        }
        cli::PrestoParser::Correct(e) => {
            correct::correct(&e.path_file)?;
        }
    }

    Ok(())
}
