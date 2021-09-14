use super::*;


pub fn parse_key_action_with_mods_py(raw: &str) -> Result<ParsedKeyAction> {
    let from = key_action_with_flags(raw).expect("failed to parse mapping trigger");
    if !from.0.is_empty() { return Err(anyhow!("failed to parse mapping trigger")); }
    let from = from.1;
    Ok(from.0)
}


fn key_sequence_or_action_with_flags(input: &str) -> ResNew<&str, Vec<ParsedKeyAction>> {
    alt((
        map(key_action_with_flags, |v| (vec![v.0], None)),
        key_sequence,
    ))(input)
}

pub fn parse_key_sequence_py(raw: &str) -> Result<Vec<ParsedKeyAction>> {
    // let raw = format!("\"{}\"", raw);
    let from = key_sequence_or_action_with_flags(raw).expect("failed to parse mapping trigger");
    if !from.0.is_empty() { return Err(anyhow!("failed to parse mapping trigger")); }
    let from = from.1;
    Ok(from.0)
}
