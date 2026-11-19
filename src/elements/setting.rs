use crate::collections::Table;

use crate::character::*;
use crate::story::*;

///////////// Setting

#[derive(Debug,Clone)]
pub struct Setting {
  pub id:         u64,
  pub name:       String,
  pub characters: Table<Character>,
}

impl Setting {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:         id,
      name:       name.clone(),
      characters: Table::new(),
    }
  }

  pub fn join_story(&self, story: &mut Story) {
    log::info!("Setting: {}: Joining the story {}.",self.name,story.name);
    story.add_setting(&self);
  }

  pub fn add_character(&mut self, character: &Character) {
    log::info!("Setting {}: Welcoming the character called {} into setting.",self.name,character.name);
    self.characters.insert(character.id,character.clone());
  }

  pub fn to_rust(&self) -> String {
    let mut instructions: Vec<String> = Vec::new();
    instructions.push(format!("let mut setting: Setting = Setting::new({},&\"{}\".to_string());",self.id,self.name));
    for (id,character) in &self.characters {
      instructions.push(character.to_rust());
      instructions.push(format!("setting.characters.insert({},character);",id));
    }
    instructions.join("\n")
  }
}
