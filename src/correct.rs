use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use crate::tags;

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
            println!("Problem with FORMAT \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        let cap = re_tok.captures(&line).unwrap();
        if !tags::POS.contains(&cap[3]) {
            println!("Problem with POS tag, {} is not allowed! \nLine num: {} \nLine: {}\n", &cap[3], line_num + 1, line);
            continue;
        }
        if !tags::COARSE.contains(&cap[4]) {
            println!("Problem with COARSE tag, {} is not allowed! \nLine num: {} \nLine: {}\n", &cap[4], line_num + 1, line);
            continue;
        }
        if !tags::FINE.contains(&cap[5]) {
            println!("Problem with FINE tag, {} is not allowed! \nLine num: {} \nLine: {}\n", &cap[5], line_num + 1, line);
            continue;
        }
        if !tags::COMP.contains(&cap[6]) {
            println!("Problem with COMP tag, {} is not allowed! \nLine num: {} \nLine: {}\n", &cap[6], line_num + 1, line);
            continue;
        }
        if !tags::NESTED.contains(&cap[7]) {
            println!("Problem with NESTED tag, {} is not allowed! \nLine num: {} \nLine: {}\n", &cap[7], line_num + 1, line);
            continue;
        }
    }

    Ok(())
}