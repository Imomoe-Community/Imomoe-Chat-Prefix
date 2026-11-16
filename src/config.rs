// Copyright (C) 2026 Mai Ooizumi
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::Deserialize;
use std::{fs, io::ErrorKind, path::Path};

// In CS2 chat control byte order, as used by S2SDK's ConvertColorStringToByte.
const COLORS: [&str; 16] = [
    "default", "darkred", "purple", "green", "olive", "lime", "red", "grey", "yellow", "bluegrey",
    "blue", "darkblue", "grey2", "orchid", "lightred", "gold",
];
const DEFAULT: &str = " [Community]\x01 ";
// Leave room for the message body within a caller's 190-byte per-message chat budget.
const MAX_PREFIX_BYTES: usize = 96;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    prefix: Prefix,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Prefix {
    Single(String),
    Multiple(Vec<String>),
}

pub(super) fn load(path: &Path, startup: bool) -> Result<Vec<String>, String> {
    let result = match fs::read_to_string(path) {
        Ok(source) => parse(&source),
        Err(error) if startup && error.kind() == ErrorKind::NotFound => Ok(vec![DEFAULT.into()]),
        Err(error) => Err(error.to_string()),
    };
    result.map_err(|error| format!("{}: {error}", path.display()))
}

fn parse(source: &str) -> Result<Vec<String>, String> {
    let config: Config = toml::from_str(source).map_err(|error| error.to_string())?;
    match config.prefix {
        Prefix::Single(text) => compile(&text).map(|value| vec![value]),
        Prefix::Multiple(entries) if entries.is_empty() => {
            Err("prefix array must not be empty".into())
        }
        Prefix::Multiple(entries) => entries
            .iter()
            .enumerate()
            .map(|(index, text)| {
                compile(text).map_err(|error| format!("prefix entry {}: {error}", index + 1))
            })
            .collect(),
    }
}

fn compile(text: &str) -> Result<String, String> {
    if text
        .chars()
        .any(|ch| ch.is_control() || matches!(ch, '\u{2028}' | '\u{2029}'))
    {
        return Err("prefix must be a single line without control characters".into());
    }
    let mut text = text.trim();
    let mut wire = String::from(" ");
    let mut visible = false;
    while !text.is_empty() {
        let end = text.find(['{', '}']).unwrap_or(text.len());
        let (plain, remaining) = text.split_at(end);
        visible |= !plain.trim().is_empty();
        wire.push_str(plain);
        if remaining.is_empty() {
            break;
        }
        let (color, after) = remaining
            .strip_prefix('{')
            .and_then(|tag| tag.split_once('}'))
            .ok_or("expected a color tag like {gold}")?;
        let index = COLORS
            .iter()
            .position(|&name| name == color)
            .ok_or_else(|| format!("unknown color {{{color}}}; use a named CS2 chat color"))?;
        wire.push((index as u8 + 1).into());
        text = after;
    }
    wire.push_str("\x01 ");
    if !visible || wire.len() > MAX_PREFIX_BYTES {
        return Err(format!(
            "prefix must contain visible text and fit in {MAX_PREFIX_BYTES} chat bytes including colors and spacing"
        ));
    }
    Ok(wire)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_example_compiles_with_segment_colors_and_a_reset() {
        let rendered = parse(include_str!(
            "../configs/imomoe_chat_prefix/prefix.example.toml"
        ))
        .unwrap();
        assert_eq!(
            rendered,
            [
                " \x08[\x10Community\x08]\x01 ",
                " \x0B[\x04Community\x0B]\x01 ",
                " \x07【\x09Community\x07】\x01 ",
            ]
        );
        assert_eq!(parse("prefix = '【社区】'").unwrap(), [" 【社区】\x01 "]);
        assert_eq!(parse("prefix = ['【社区】']").unwrap(), [" 【社区】\x01 "]);
    }

    #[test]
    fn rejects_wrong_colors_and_prefixes_that_cannot_be_displayed_as_one_line() {
        for invalid in [
            "prefix = '{ff0000}[Community]'",
            "prefix = '{gold}'",
            "prefix = '{gold[Community]'",
            "prefix = '[Community]}'",
            r#"prefix = "two\nlines""#,
            r#"prefix = "raw\u0004code""#,
            "prefix = 'test'\ncolour = 'red'",
            "prefix = []",
            "prefix = ['[Community]', 1]",
        ] {
            assert!(parse(invalid).is_err(), "{invalid}");
        }
        assert!(parse(&format!("prefix = '{}'", "社".repeat(32))).is_err());
        assert_eq!(compile(&"a".repeat(93)).unwrap().len(), MAX_PREFIX_BYTES);
        assert!(compile(&"a".repeat(94)).is_err());
        assert!(
            parse("prefix = ['[Community]', '{typo}[Community]']")
                .unwrap_err()
                .contains("prefix entry 2")
        );
    }
}
