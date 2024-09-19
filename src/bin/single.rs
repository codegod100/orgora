use orgora::parse;

fn main() {
    let line = "* [[agora]]";
    println!("Line: {:#?}", line);
    parse(line);
}
