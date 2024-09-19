use std::{fs::File, io::Write, path::Path};

use orgora::{parse, parse_file, Rule};
use pest::iterators::Pair;
use urlencoding::encode;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let filepath = &args[1];
    let p = Path::new(filepath);
    let parent = p.parent().unwrap().to_str().unwrap();
    let stem = p.file_stem().unwrap().to_str().unwrap();
    let html_filepath = format!("{}/{}.html", parent, stem);
    let mut f = File::create(html_filepath).unwrap();

    let content = std::fs::read_to_string(filepath).unwrap();
    let output = parse_file(content);
    f.write(output.as_bytes()).unwrap();
}
