// inspired by https://gitlab.com/davidbittner/ansi-parser/
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::combinator::opt;
use nom::multi::many0;
use nom::IResult;

use super::enums::AnsiSequence;

type Res<'a> = IResult<&'a str, AnsiSequence>;

macro_rules! tag_parser {
    ($sig:ident, $tag:expr, $ret:expr) => {
        fn $sig(input: &str) -> Res {
            let (input, _) = tag($tag)(input)?;

            Ok((input, $ret))
        }
    };
}

fn escape(input: &str) -> Res {
    let (input, _) = tag("\u{1b}")(input)?;

    Ok((input, AnsiSequence::Escape))
}

fn parse_number_or_default(input: &str, default: u32) -> u32 {
    match input.parse::<u32>() {
        Ok(n) => n,
        Err(_) => default,
    }
}

fn cursor_pos(input: &str) -> Res {
    let (input, _) = tag("[")(input)?;

    let (input, x) = digit0(input)?;

    let x = parse_number_or_default(x, 1);

    let (input, _) = opt(tag(";"))(input)?;

    let (input, y) = digit0(input)?;

    let y = parse_number_or_default(y, 1);

    let (input, _) = alt((tag("H"), tag("f")))(input)?;

    Ok((input, AnsiSequence::CursorPos(x as u16, y as u16)))
}

fn cursor_up(input: &str) -> Res {
    let (input, _) = tag("[")(input)?;

    let (input, n) = digit0(input)?;

    let n = parse_number_or_default(n, 1);

    let (input, _) = tag("A")(input)?;

    Ok((input, AnsiSequence::CursorUp(n as u16)))
}

fn cursor_down(input: &str) -> Res {
    let (input, _) = tag("[")(input)?;

    let (input, n) = digit0(input)?;

    let n = parse_number_or_default(n, 1);

    let (input, _) = tag("B")(input)?;

    Ok((input, AnsiSequence::CursorDown(n as u16)))
}

fn cursor_forward(input: &str) -> Res {
    let (input, _) = tag("[")(input)?;

    let (input, n) = digit0(input)?;

    let n = parse_number_or_default(n, 1);

    let (input, _) = tag("C")(input)?;

    Ok((input, AnsiSequence::CursorForward(n as u16)))
}

fn cursor_backward(input: &str) -> Res {
    let (input, _) = tag("[")(input)?;

    let (input, n) = digit0(input)?;

    let n = parse_number_or_default(n, 1);

    let (input, _) = tag("D")(input)?;

    Ok((input, AnsiSequence::CursorBackward(n as u16)))
}

fn graphics_mode(input: &str) -> Res {
    let mut v = vec![];

    let (mut input, _) = tag("[")(input)?;

    for i in 1..=5 {
        (input, _) = opt(tag(";"))(input)?;

        let x = digit0(input)?;
        input = x.0;
        let n = x.1;

        match n.parse::<u8>() {
            Ok(n) => v.push(n),
            // no graphics mode with 4 numbers
            // so if the fifth iteration gives an error
            // but not the fourth, the input is faulty
            Err(_) if i == 5 => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::NoneOf,
                )))
            }
            Err(_) => break,
        }
    }

    let (input, _) = tag("m")(input)?;

    Ok((input, AnsiSequence::SetGraphicsMode(v)))
}

fn combined(input: &str) -> Res {
    alt((
        escape,
        cursor_pos,
        cursor_up,
        cursor_down,
        cursor_forward,
        cursor_backward,
        graphics_mode,
    ))(input)
}

fn parse_escape(input: &str) -> Res {
    let (input, _) = tag("\u{1b}")(input)?;

    combined(input)
}

fn parse_char(input: &str) -> Res {
    let (input, c) = nom::character::complete::anychar(input)?;

    Ok((input, AnsiSequence::Character(c)))
}

pub fn parse(input: &str) -> IResult<&str, Vec<AnsiSequence>> {
    many0(alt((parse_escape, parse_char)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_escape() {
        assert_eq!(escape("\u{1b}"), Ok(("", AnsiSequence::Escape)));
    }

    #[test]
    pub fn test_cursor_pos() {
        assert_eq!(cursor_pos("[5;5H"), Ok(("", AnsiSequence::CursorPos(5, 5))));
        assert_eq!(cursor_pos("[;5f"), Ok(("", AnsiSequence::CursorPos(1, 5))));
        assert_eq!(cursor_pos("[5;H"), Ok(("", AnsiSequence::CursorPos(5, 1))));
    }

    #[test]
    pub fn test_cursor_up() {
        assert_eq!(cursor_up("[2A"), Ok(("", AnsiSequence::CursorUp(2))));
    }

    #[test]
    pub fn test_cursor_down() {
        assert_eq!(cursor_down("[2B"), Ok(("", AnsiSequence::CursorDown(2))));
    }

    #[test]
    pub fn test_cursor_backward() {
        assert_eq!(
            cursor_backward("[2D"),
            Ok(("", AnsiSequence::CursorBackward(2)))
        );
    }

    #[test]
    pub fn test_cursor_forward() {
        assert_eq!(
            cursor_forward("[2C"),
            Ok(("", AnsiSequence::CursorForward(2)))
        );
    }

    #[test]
    pub fn test_graphics_mode() {
        assert_eq!(
            graphics_mode("[m"),
            Ok(("", AnsiSequence::SetGraphicsMode(vec![])))
        );
        assert_eq!(
            graphics_mode("[1m"),
            Ok(("", AnsiSequence::SetGraphicsMode(vec![1])))
        );
        assert_eq!(
            graphics_mode("[1;2m"),
            Ok(("", AnsiSequence::SetGraphicsMode(vec![1, 2])))
        );
        assert_eq!(
            graphics_mode("[1;2;3m"),
            Ok(("", AnsiSequence::SetGraphicsMode(vec![1, 2, 3])))
        );
        assert_eq!(
            graphics_mode("[1;2;3;4m"),
            Err(nom::Err::Error(nom::error::Error::new(
                "m",
                nom::error::ErrorKind::NoneOf
            )))
        );
        assert_eq!(
            graphics_mode("[1;2;3;4;5m"),
            Ok(("", AnsiSequence::SetGraphicsMode(vec![1, 2, 3, 4, 5])))
        );
        assert_eq!(
            graphics_mode("[1;2;3;4;5;6m"),
            Err(nom::Err::Error(nom::error::Error::new(
                ";6m",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_parse_escape() {
        assert_eq!(parse_escape("\u{1b}\u{1b}"), Ok(("", AnsiSequence::Escape)));
        assert_eq!(
            parse_escape("\u{1b}[C"),
            Ok(("", AnsiSequence::CursorForward(1)))
        );
    }

    #[test]
    fn test_char() {
        assert_eq!(parse_char("a"), Ok(("", AnsiSequence::Character('a'))));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("\u{1b}[1;1mt"),
            Ok((
                "",
                vec![
                    AnsiSequence::SetGraphicsMode(vec![1, 1]),
                    AnsiSequence::Character('t')
                ]
            ))
        );
    }
}
