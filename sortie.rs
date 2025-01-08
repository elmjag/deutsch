use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const NOUNS_RS_FILE: &str = "./src/db/nouns.rs";

fn parse_line(line: &String) -> Option<(String, String)> {
    if !line.starts_with("    (\"d") {
        return None;
    }

    let split = line.rsplit_once("\", \"");
    if split.is_none() {
        return None;
    }

    let (article, word) = split.unwrap();

    let article = String::from(&article[6..article.len()]);
    let word = String::from(&word[0..word.len() - 3]);

    Some((article, word))
}

fn parse_source_file() -> Vec<(String, String)> {
    let reader = BufReader::new(File::open(NOUNS_RS_FILE).unwrap());
    let mut nouns: Vec<(String, String)> = vec![];

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let res = parse_line(&line);
        if res.is_none() {
            continue;
        }

        let (article, word) = res.unwrap();
        nouns.push((article, word));
    }

    nouns
}

fn sort_nouns(nouns: &mut Vec<(String, String)>) {
    fn compare_nouns(a: &(String, String), b: &(String, String)) -> Ordering {
        let article_order = a.0.cmp(&b.0);

        if article_order.is_eq() {
            return a.1.cmp(&b.1);
        }

        article_order
    }

    nouns.sort_by(compare_nouns);
}

fn write_source_file(nouns: &Vec<(String, String)>) {
    let mut file = File::create(NOUNS_RS_FILE).unwrap();

    writeln!(file, r#"pub const NOUNS: &[(&str, &str)] = &["#).unwrap();

    for (article, word) in nouns {
        writeln!(file, r#"    ("{article}", "{word}"),"#).unwrap();
    }

    writeln!(file, "];").unwrap();
}

fn main() {
    let mut nouns: Vec<(String, String)> = parse_source_file();
    sort_nouns(&mut nouns);
    write_source_file(&nouns);
}
