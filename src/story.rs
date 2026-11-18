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

  pub fn add_character(&mut self, character: &Character) {
    log::info!("Adding the character called {} to {}.",character.name,self.name);
    self.characters.insert(character.id,character.clone());
  }
}
