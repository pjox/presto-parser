use crate::tags;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn correct(src: &Path) -> Result<(), Box<dyn Error>> {
    let presto_file = File::open(src)?;
    let f = BufReader::new(presto_file);

    let re_tok = Regex::new(r"^(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)$").unwrap();

    for (line_num, line) in f.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.eq("form\tlemma\tPOS\tO\tO\tO\tO\t_") {
            continue;
        }
        if line.matches('\t').count() != 7 {
            println!(
                "In file {}\nProblem with FORMAT \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                line_num + 1,
                line
            );
            continue;
        }
        let cap = re_tok.captures(&line).unwrap();
        if !tags::POS.contains(&cap[3]) {
            println!(
                "In file {}\nProblem with POS tag, {} is not allowed! \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                &cap[3],
                line_num + 1,
                line
            );
            continue;
        }
        if !tags::COARSE.contains(&cap[4]) {
            println!(
                "In file {}\nProblem with COARSE tag, {} is not allowed! \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                &cap[4],
                line_num + 1,
                line
            );
            continue;
        }
        if !tags::FINE.contains(&cap[5]) {
            println!(
                "In file {}\nProblem with FINE tag, {} is not allowed! \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                &cap[5],
                line_num + 1,
                line
            );
            continue;
        }
        if !tags::COMP.contains(&cap[6]) {
            println!(
                "In file {}\nProblem with COMP tag, {} is not allowed! \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                &cap[6],
                line_num + 1,
                line
            );
            continue;
        }
        if !tags::NESTED.contains(&cap[7]) {
            println!(
                "In file {}\nProblem with NESTED tag, {} is not allowed! \nLine num: {} \nLine: {}\n",
                src.file_name().unwrap().to_str().unwrap(),
                &cap[7],
                line_num + 1,
                line
            );
            continue;
        }
    }

    Ok(())
}
