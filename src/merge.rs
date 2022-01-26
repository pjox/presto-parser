use crate::correct;
use std::{
    error::Error,
    fs::File,
    io,
    io::{BufReader, BufWriter, Write},
    path::Path,
};
use walkdir::WalkDir;

pub fn merge(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    let out = match File::create(dst) {
        Err(why) => panic!("couldn't create {}: {}", dst.display(), why),
        Ok(file) => file,
    };

    let mut buf_out = BufWriter::new(out);

    for entry in WalkDir::new(src)
        .max_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
    {
        let entry = entry?;
        if entry.path().is_file() {
            correct::correct(&entry.path())?;
            let file = File::open(entry.path())?;
            let mut f = BufReader::new(file);
            writeln!(
                buf_out,
                "{}\txxx\tXXX\tO\tO\tO\tO\t_",
                entry.file_name().to_str().unwrap()
            )?;
            io::copy(&mut f, &mut buf_out)?;
        }
    }
    Ok(())
}
