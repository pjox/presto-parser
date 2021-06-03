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

    let mut lines = f.lines().enumerate();
    lines.next();

    lines.next();

    for (line_num, line) in lines {
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
            println!("Problem with POS tag \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        if !tags::COARSE.contains(&cap[4]) {
            println!("Problem with COARSE tag \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        if !tags::FINE.contains(&cap[5]) {
            println!("Problem with FINE tag \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        if !tags::COMP.contains(&cap[6]) {
            println!("Problem with COMP tag \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        if !tags::NESTED.contains(&cap[7]) {
            println!("Problem with NESTED tag \nLine num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
    }

    Ok(())
}