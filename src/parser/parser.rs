use std::{
};

use crate::collections::Config;
use crate::elements::*;
use super::*;
use super::helpers::*;

///////////// Parser

pub enum LoreValue {
  Str(String),
  Num(i64),
  Bool(bool),
}

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
    let statements: Vec<&str> = Self::split_outside_quotes(code.as_str(),';');
    for statement in statements {
      // Do the parsing.
    }
    Ok(Element::Library(Library::new()))
  }

  pub fn parse_cli(&self, library: &mut Library, line: &str) -> Result<String,String> {
    if !line.trim().is_empty() {
      let mut args: Vec<&str> = line.trim().split(" ").collect();
      match args[0].to_lowercase().as_str() {
        "exit"|"quit" => Ok(String::new()),
        "compose" => {
          if args.len() > 1 {
            args.insert(1,"tome");
            Self::create_cli(library,&args)
          } else {
            Err(format!("To compose a new tome, please include a title."))
          }
        },
        "new"|"create" => {
          if args.len() > 2 {
            Self::create_cli(library,&args)
          } else {
            Err(format!("To create a new element, please include the type and a name ('new TYPE NAME')."))
          }
        },
        _ => Err(format!("Invalid statement: {}",line)),
      }
    } else {
      Ok(String::new())
    }
  }
}
