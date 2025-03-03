use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const NOUNS_INPUT_FILE: &str = "src/nouns.txt";
const NOUNS_OUTPUT_FILE: &str = "nouns";

fn parse_line(line: &String) -> (String, String) {
    let (article, word) = line.split_once(" ").unwrap();

    (String::from(article), String::from(word))
}

fn parse_nouns_file() -> impl Iterator<Item = (String, String)> {
    let reader = BufReader::new(File::open(NOUNS_INPUT_FILE).unwrap());

    reader.lines().map(|l| {
        let line = l.unwrap();
        parse_line(&line)
    })
}

fn write_nouns_list() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(NOUNS_OUTPUT_FILE);

    let f = File::create(dest_path).unwrap();
    {
        let mut writer = BufWriter::new(f);
        let mut lines = 0;

        writer
            .write(b"pub const NOUNS: [(&str, &str); NUM] = [\n")
            .unwrap();

        for (a, w) in parse_nouns_file() {
            lines += 1;
            let line = format!("    (\"{a}\", \"{w}\"),\n");
            writer.write(line.as_bytes()).unwrap();
        }

        let last_line = format!("];\nconst NUM: usize = {lines};");
        writer.write(last_line.as_bytes()).unwrap();
    }
}

fn main() {
    write_nouns_list();

    println!("cargo::rerun-if-changed={NOUNS_INPUT_FILE}");
    println!("cargo::rerun-if-changed=build.rs");
}
