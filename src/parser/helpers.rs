use std::{
  collections::HashMap,
};

use super::*;
use crate::elements::*;

///////////// Parser

impl Parser {
  pub(super) fn parse_property_block(block: Option<&str>) -> Result<HashMap<String,LoreValue>,String> {
    let mut properties = HashMap::new();
    let Some(block) = block else {
      return Ok(properties);
    };
    let content = match block.strip_prefix('{') {
      Some(content) => match content.strip_suffix('}') {
        Some(content) => content.trim(),
        None          => return Err(format!("Expected closing '}}' in property block, got: {}",block)),
      },
      None => return Err(format!("Expected opening '{{' in property block, got: {}",block)),
    };
    for item in Self::split_outside_quotes(content, ',') {
      let (key,value) = match item.split_once('=') {
        Some((key,value)) => match Self::parse_literal(value.trim()) {
          Ok(value) => (key,value),
          Err(err)  => return Err(err),
        },
        None => return Err(format!("Invalid property assignment: {}", item)),
      };
      properties.insert(key.trim().to_string(),value);
    }
    Ok(properties)
  }

  pub(super) fn parse_literal(raw: &str) -> Result<LoreValue, String> {
    if raw == "true" { return Ok(LoreValue::Bool(true)); }
    if raw == "false" { return Ok(LoreValue::Bool(false)); }
    if let Ok(n) = raw.parse::<i64>() { return Ok(LoreValue::Num(n)); }
    if let Some(inner) = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
      let mut out = String::with_capacity(inner.len());
      let mut chars = inner.chars();
      while let Some(ch) = chars.next() {
        if ch == '\\' {
          match chars.next() {
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => return Err("Trailing backslash in string".to_string()),
          }
        } else {
          out.push(ch);
        }
      }
      return Ok(LoreValue::Str(out));
    }
    Err(format!("Invalid literal value: {}", raw))
  }

  pub(super) fn split_outside_quotes(input: &str, delimiter: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let mut escaped = false;
    for (idx, ch) in input.char_indices() {
      if escaped {
       escaped = false;
       continue;
      }
      if ch == '\\' && in_quotes {
        escaped = true;
        continue;
      }
      if ch == '"' {
        in_quotes = !in_quotes;
        continue;
      }
      if ch == delimiter && !in_quotes {
        let part = input[start..idx].trim();
        if !part.is_empty() {
          parts.push(part);
        }
        start = idx + delimiter.len_utf8();
      }
    }
    let tail = input[start..].trim();
    if !tail.is_empty() {
      parts.push(tail);
    }
    parts
  }

  pub(super) fn take_string(properties: &mut HashMap<String,LoreValue>, key: &str) -> Result<String,String> {
    match properties.remove(key) {
      Some(LoreValue::Str(s)) => Ok(s),
      Some(_) => Err(format!("Property '{}' must be a string", key)),
      None => Err(format!("Missing required property '{}'", key)),
    }
  }
}
