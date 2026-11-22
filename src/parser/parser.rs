use std::{
};

use crate::collections::Config;
use crate::elements::*;

///////////// Parser

#[derive(Clone)]
pub struct Parser {
  pub config:  Config,
}

impl Parser {
  pub fn new(config: &Config) -> Self {
    Self {
     config:  config.clone(),
    }
  }

  pub fn parse(&self, code: &String) -> Result<Element,String> {
    let statements: Vec<&str> = Self::split_statements(code.as_str());
    for statement in statements {
      // Do the parsing.
    }
    Ok(Element::Library(Library::new()))
  }

  pub fn split_statements(input: &str) -> Vec<&str> {
    let mut statements = Vec::new();
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
      if ch == ';' && !in_quotes {
        let stmt = input[start..idx].trim();
        if !stmt.is_empty() {
          statements.push(stmt);
        }
        start = idx + 1; // ';' is 1 byte in UTF-8
      }
    }
    let tail = input[start..].trim();
    if !tail.is_empty() { statements.push(tail); }
    statements
  }
}
