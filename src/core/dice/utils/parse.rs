//! Vocabulary die notation parsing: the exact `NdM` form.

use crate::core::dice::types::dice::Dice;

/// The one shape die notation may take, worded for error messages.
const DICE_SHAPE: &str =
    "die notation must be '<count>d<faces>' with both sides positive integers, e.g. \"2d2\"";

/// Parse the exact `NdM` form: a positive die count, a lowercase `d`,
/// and a positive face count, with nothing else on either side. Any
/// deviation — uppercase `D`, zero, a missing side, extra characters —
/// is an error stating the one legal shape.
pub fn parse_dice(text: &str) -> Result<Dice, String> {
    let (count, faces) = text.split_once('d').ok_or_else(|| die_error(text))?;
    Ok(Dice {
        n: positive_part(count, text)?,
        m: positive_part(faces, text)?,
    })
}

/// One side of a die expression: digits only, at least one digit, and
/// a value of at least one. The digit check keeps signs, whitespace,
/// and any other spelling from slipping through the integer parse.
fn positive_part(side: &str, whole: &str) -> Result<i32, String> {
    if side.is_empty() || !side.bytes().all(|b| b.is_ascii_digit()) {
        return Err(die_error(whole));
    }
    side.parse::<i32>()
        .ok()
        .filter(|value| *value >= 1)
        .ok_or_else(|| die_error(whole))
}

/// One error shape for every rejection: the message states the legal
/// form and quotes the offending text.
fn die_error(text: &str) -> String {
    format!("{DICE_SHAPE}, got '{text}'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_forms_parse() {
        assert_eq!(parse_dice("2d2"), Ok(Dice { n: 2, m: 2 }));
        assert_eq!(parse_dice("1d3"), Ok(Dice { n: 1, m: 3 }));
        assert_eq!(parse_dice("10d250"), Ok(Dice { n: 10, m: 250 }));
    }

    #[test]
    fn illegal_forms_are_rejected() {
        for text in [
            "0d2", "2d0", "2D2", "2d", "d6", "2d2x", "+2d2", " 2d2", "-1d2", "2d2d2", "",
        ] {
            let error = parse_dice(text).unwrap_err();
            assert!(error.contains("die notation"), "{text}: {error}");
            assert!(error.contains(text), "{text}: {error}");
        }
    }
}
