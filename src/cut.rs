use regex::Regex;
use std::{
    error::Error,
    fs,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

pub fn cut(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    let presto_file = File::open(src)?;
    let f = BufReader::new(presto_file);

    fs::create_dir_all(dst)?;

    let re = Regex::new(r"^(.+)\.conll\txxx\tXXX\tO\tO\tO\tO\t_$").unwrap();

    let mut lines = f.lines();
    lines.next();
    let line = match lines.next() {
        None => panic!("File does not Follow Presto format"),
        Some(line) => line,
    };
    let line = line.unwrap();
    let cap = re.captures(&line).unwrap();
    let mut out = match File::create(format!("{}/{}.conll", dst.display(), &cap[1])) {
        Err(why) => panic!("couldn't create {}: {}", &line, why),
        Ok(file) => file,
    };
    let mut buf_out = BufWriter::new(out);

    for line in lines {
        let line = line.unwrap();
        if re.is_match(&line) {
            buf_out.flush()?;
            let cap = re.captures(&line).unwrap();
            out = match File::create(format!("{}/{}.conll", dst.display(), &cap[1])) {
                Err(why) => panic!("couldn't create {}: {}", &line, why),
                Ok(file) => file,
            };
            buf_out = BufWriter::new(out);
        } else {
            writeln!(buf_out, "{}", &line)?;
        }
    }
    buf_out.flush()?;

    Ok(())
}
