use crate::collections::Table;

use crate::character::*;

///////////// Story

#[derive(Debug,Clone)]
pub struct Story {
  pub id:         u64,
  pub name:       String,
  pub characters: Table<Character>,
}

impl Story {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:         id,
      name:       name.clone(),
      characters: Table::new(),
    }
  }
}
