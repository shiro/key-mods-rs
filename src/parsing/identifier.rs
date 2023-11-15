use unicode_xid::UnicodeXID;

use super::*;

pub(super) fn ident(input: &str) -> ResNew<&str, String> {
    let (rest, id) = match word(input) {
        Ok((rest, id)) => (rest, id),
        Err(_) => return Err(make_generic_nom_err_options(input, vec!["identifier".to_string()])),
    };

    match id.0.as_ref() {
        "break" | "continue" | "do" | "else" | "false" | "for" |
        "if" | "in" | "let" | "loop" | "return" | "true" | "while"
        => Err(make_generic_nom_err_new(input)),
        _ => Ok((rest, id)),
    }
}

pub(super) fn word(input: &str) -> ResNew<&str, String> {
    let (input, _) = ws0(input)?;

    let mut chars = input.char_indices();
    match chars.next() {
        Some((_, ch)) if UnicodeXID::is_xid_start(ch) || ch == '_' => {}
        _ => return Err(make_generic_nom_err_options(input, vec!["word".to_string()])),
    }

    while let Some((i, ch)) = chars.next() {
        if !UnicodeXID::is_xid_continue(ch) {
            return Ok((&input[i..], (input[..i].into(), None)));
        }
    }

    Ok(("", (input.into(), None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weird_names() {
        assert_eq!(ident("_foobar"), nom_ok("_foobar".to_string()));
        assert_eq!(ident("btn_forward"), nom_ok("btn_forward".to_string()));
        assert_eq!(ident("š"), nom_ok("š".to_string()));
        assert_eq!(ident("foo.bar"), nom_ok_rest(".bar", "foo".to_string()));
    }
}
