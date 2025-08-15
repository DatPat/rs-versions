//! Reusable parsers for the `versions` library.

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::{IResult, Parser};

/// Parse an unsigned integer.
///
/// Should yield either a zero on its own, or some other multi-digit number.
pub(crate) fn unsigned(input: &str) -> IResult<&str, u32> {
    let mut end = 0;
    for (idx, c) in input.char_indices() {
        if !c.is_ascii_digit() {
            break;
        }
        end = idx + c.len_utf8();
    }

    let (number_str, rest) = input.split_at(end);

    if number_str.is_empty() {
        return Ok((rest, 0));
    }

    let number = number_str.parse::<u32>().unwrap();

    Ok((rest, number))
}

#[test]
fn unsigned_test() {
    assert!(unsigned("0").is_ok());
    assert!(unsigned("123").is_ok());

    match unsigned("06") {
        Ok(("", 6)) => {}
        Ok(_) => panic!("Parsed 06, but gave wrong output"),
        Err(_) => panic!("Couldn't parse 06"),
    }
}

/// Some alphanumeric combination, possibly punctuated by `-` characters.
pub(crate) fn hyphenated_alphanums(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-')(i)
}

/// Some alphanumeric combination.
pub(crate) fn alphanums(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric())(i)
}

/// Parse metadata. As of SemVer 2.0, this can contain alphanumeric characters
/// as well as hyphens.
pub fn meta(i: &str) -> IResult<&str, String> {
    let (i, _) = char('+')(i)?;
    map(
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.'),
        |s: &str| s.to_owned(),
    )
    .parse(i)
}
