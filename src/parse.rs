use regex::Regex;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    usize,
};

#[derive(Debug, Clone)]
pub struct Token {
    pub id: usize,
    pub form: String,
    pub lemma: String,
    pub pos: String,
    pub coarse: String,
    pub fine: String,
    pub comp: String,
    pub nested: String,
    pub wikidata: String,
}

#[derive(Debug, Clone)]
pub struct Sentence {
    pub id: usize,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: usize,
    pub name: String,
    pub sentences: Vec<Sentence>,
}

pub fn parse(src: &Path) -> Result<(), Box<dyn Error>> {
    let presto_file = File::open(src)?;
    let f = BufReader::new(presto_file);

    // let re_title = Regex::new(r"^(.+)\.tsv\txxx\tXXX\tO\tO\tO\tO\t_$").unwrap();
    let re_tok = Regex::new(r"^(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)\t(.+)$").unwrap();

    let mut lines = f.lines().enumerate();
    lines.next();
    // let (_, line) = match lines.next() {
    //     None => panic!("File does not Follow Presto format"),
    //     Some(line) => line,
    // };

    // let mut presto: Vec<Document> = Vec::new();

    // let line = line.unwrap();
    // let cap = re_title.captures(&line).unwrap();

    // let mut doc_id: usize;
    // let mut sent_id: usize;
    // doc_id = 0;
    // sent_id = 0;

    // let mut doc = Document {
    //     id: doc_id,
    //     name: String::from(&cap[1]),
    //     sentences: Vec::new(),
    // };
    lines.next();

    // doc_id += 1;

    // let mut sent = Sentence {
    //     id: sent_id,
    //     tokens: Vec::new(),
    // };

    // let mut tok = Token::default();
    let mut pos = HashSet::new();
    let mut coarse = HashSet::new();
    let mut fine = HashSet::new();
    let mut comp = HashSet::new();
    let mut nested = HashSet::new();

    for (line_num, line) in lines {
        let line = line.unwrap();
        if line.is_empty() {
            // doc.sentences.push(sent.clone());
            // sent_id += 1;
            // sent = Sentence {
            //     id: sent_id,
            //     tokens: Vec::new(),
            // };
            continue;
        }
        // if re_title.is_match(&line) {
        //     presto.push(doc.clone());
        //     let cap = re_title.captures(&line).unwrap();
        //     doc = Document {
        //         id: doc_id,
        //         name: String::from(&cap[1]),
        //         sentences: Vec::new(),
        //     };
        //     doc_id += 1;
        // }
        if line.eq("form\tlemma\tPOS\tO\tO\tO\tO\t_") {
            continue;
        }
        if line.matches('\t').count() != 7 {
            println!("Line num: {} \nLine: {}\n", line_num + 1, line);
            continue;
        }
        let cap = re_tok.captures(&line).unwrap();

        pos.insert(String::from(&cap[3]));
        coarse.insert(String::from(&cap[4]));
        fine.insert(String::from(&cap[5]));
        comp.insert(String::from(&cap[6]));
        nested.insert(String::from(&cap[7]));

        // let tok = Token {
        //     id: line_num,
        //     form: String::from(&cap[1]),
        //     lemma: String::from(&cap[2]),
        //     pos: String::from(&cap[3]),
        //     coarse: String::from(&cap[4]),
        //     fine: String::from(&cap[5]),
        //     comp: String::from(&cap[6]),
        //     nested: String::from(&cap[7]),
        //     wikidata: String::from(&cap[8]),
        // };
        // sent.tokens.push(tok.clone());
    }

    let mut pos: Vec<_> = pos.into_iter().collect();
    pos.sort();
    let mut coarse: Vec<_> = coarse.into_iter().collect();
    coarse.sort();
    let mut fine: Vec<_> = fine.into_iter().collect();
    fine.sort();
    let mut comp: Vec<_> = comp.into_iter().collect();
    comp.sort();
    let mut nested: Vec<_> = nested.into_iter().collect();
    nested.sort();

    println!("POS:");
    for x in pos {
        println!("{:?}", x);
    }

    println!("\nCOARSE:");
    for x in coarse {
        println!("{:?}", x);
    }
    println!("\nFINE:");
    for x in fine {
        println!("{:?}", x);
    }
    println!("\nCOMP:");
    for x in comp {
        println!("{:?}", x);
    }
    println!("\nNESTED:");
    for x in nested {
        println!("{:?}", x);
    }

    Ok(())
}
