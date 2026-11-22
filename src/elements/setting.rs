use crate::collections::Table;

use crate::character::*;
use crate::story::*;

///////////// Setting

#[derive(Debug,Clone)]
pub struct Setting {
  pub name:       String,
  pub characters: Table<Character>,
}

impl Setting {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name:       name.into(),
      characters: Table::new(),
    }
  }

  pub fn join_story(&self, story: &mut Story) -> u64 {
    log::info!("Setting: {}: Joining the story {}.",self.name,story.name);
    story.add_setting(&self)
  }

  pub fn add_character(&mut self, character: &Character) -> u64 {
    let id: u64 = self.characters.next();
    log::info!("Setting {}: Welcoming the character called {} into setting.",self.name,character.name);
    self.characters.insert(id,character.clone());
    id
  }

  pub fn list_characters(&self) -> Vec<(u64,String)> {
    let characters: Vec<(u64,String)> = self.characters.iter().map(|(id,character)| (id,character.name.clone())).collect();
    log::debug!("Setting {}: Returning a list of {} characters.",self.name,characters.len());
    characters
  }

  pub fn to_rust(&self) -> String {
    log::debug!("Setting {}: Converting to rust code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let mut setting: Setting = Setting::new(\"{}\");",name));
    for (id,character) in &self.characters {
      instructions.push(character.to_rust());
      instructions.push(format!("setting.characters.insert({},character);",id));
    }
    instructions.join("\n")
  }

  pub fn to_lore(&self) -> String {
    log::debug!("Setting {}: Converting to lorebook code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let setting = new Setting{{ name = \"{}\" }};",name));
    for (id,character) in &self.characters {
      instructions.push(character.to_lore());
      instructions.push(format!("setting.characters.insert({},character);",id));
    }
    instructions.join("\n")
  }

}
