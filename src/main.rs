extern crate onig;

use onig::{Syntax, RegexOptions, Regex};
use std::ops::BitOr;
use onig::{MatchParam, Region, SearchOptions};

fn find_no_panic(regex: &Regex, value: &str) -> Option<(usize, usize)> {
    let match_param = MatchParam::default();
    let mut region = Region::new();
    let result = regex.search_with_param(value, 0, value.len(), SearchOptions::SEARCH_OPTION_NONE, Some(&mut region), match_param);
    result.ok().and_then(|_| region.pos(0))
}

fn main() {
    let r = onig::Regex::with_options("(?:<(script|iframe|embed|frame|frameset|object|img|applet|body|html|style|layer|link|ilayer|meta|bgsound))", RegexOptions::REGEX_OPTION_MULTILINE
        .bitor(RegexOptions::REGEX_OPTION_IGNORECASE)
        .bitor(RegexOptions::REGEX_OPTION_NEGATE_SINGLELINE), Syntax::java()).unwrap();
    let mut sum: usize = 0;
    let len: usize = 1000000000000;
    for _ in 1..len {
        if find_no_panic(&r, "someting something something something").is_some() {
            sum += 1;
        }
    }
    println!("Found {}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
