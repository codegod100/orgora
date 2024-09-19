extern crate pest;
use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;
use urlencoding::encode;
#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to project `src`
struct OrgParser;
use extism_pdk::*;

pub fn parse_line(line: &str) -> Result<pest::iterators::Pair<Rule>, Error<Rule>> {
    let mut pairs = match OrgParser::parse(Rule::input, line) {
        Ok(pairs) => pairs,
        Err(e) => return Err(e),
    };

    let pair = pairs.next().unwrap();
    Ok(pair)
}

pub fn html(pair: Pair<Rule>) -> String {
    let inner = pair.into_inner().flatten();
    let mut text = "".to_string();
    let mut replace = "".to_string();
    let mut target = "".to_string();
    let mut display = "".to_string();
    for pair in inner {
        match pair.as_rule() {
            Rule::text => text.push_str(pair.as_str()),
            Rule::normal_link | Rule::weird_link | Rule::hashtag => {
                // println!("{:#?}", pair);
                replace = pair.as_str().to_string();
                if pair.as_rule() == Rule::hashtag {
                    display = pair.as_str().to_string();
                }
            }
            Rule::target | Rule::hashtag_target => target = pair.as_str().to_string(),
            Rule::display => display = pair.as_str().to_string(),
            _ => (),
        }
    }
    if target == "" {
        return text.to_string();
    }
    if display == "" {
        display = target.clone();
    }
    if !target.contains("http") {
        target = encode(&target).into_owned();
    }
    let link = format!("<a href='{}'>{}</a>", target, display);
    let out = text.replace(&replace, &link);
    out
}

pub fn parse(content: String) -> String {
    let lines = content.lines();
    let mut level = 0;
    let mut output = "<ul>".to_string();
    for line in lines {
        let pair = match parse_line(line) {
            Ok(pair) => pair,
            Err(_e) => {
                continue;
            }
        };
        match pair.as_rule() {
            Rule::l1 => {
                if level >= 1 {
                    output.push_str("</li>\n")
                }
                if level > 1 {
                    output.push_str("</ul>")
                }
                output.push_str("<li>");
                let out = html(pair);
                output.push_str(&out);
                level = 1;
            }
            Rule::l2 => {
                if level > 2 {
                    output.push_str("</ul>")
                }
                if level == 2 {
                    output.push_str("</li>")
                }
                if level < 2 {
                    output.push_str("<ul>")
                }
                output.push_str("<li>");
                let out = html(pair);
                output.push_str(&out);
                level = 2;
            }
            Rule::l3 => {
                if level >= 3 {
                    output.push_str("</li>")
                }
                if level < 3 {
                    output.push_str("<ul>")
                }
                output.push_str("<li>");
                let out = html(pair);
                output.push_str(&out);
                level = 3;
            }
            _ => (),
        }
    }
    output.push_str("\n</li></ul>");
    output
}


#[plugin_fn]
pub fn parse_with_wasm(content: String) -> FnResult<String>{
    Ok(parse(content))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_parse_file() {
        let content = fs::read_to_string("fixtures/sample1.org").unwrap();
        let html = fs::read_to_string("fixtures/sample1.html").unwrap();
        let result = crate::parse(content.clone());
        // println!("{result}");
        assert_eq!(result, html);
    }
}
