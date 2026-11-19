use crate::collections::Table;

use crate::character::*;
use crate::setting::*;

///////////// Story

#[derive(Debug,Clone)]
pub struct Story {
  pub id:         u64,
  pub name:       String,
  pub characters: Table<Character>,
  pub settings:   Table<Setting>,
}

impl Story {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:         id,
      name:       name.clone(),
      characters: Table::new(),
      settings:   Table::new(),
    }
  }

  pub fn add_character(&mut self, character: &Character) {
    log::info!("Story {}: Adding the character called {}.",self.name,character.name);
    self.characters.insert(character.id,character.clone());
  }

  pub fn add_setting(&mut self, setting: &Setting) {
    log::info!("Story {}: Adding the setting called {}.",self.name,setting.name);
    self.settings.insert(setting.id,setting.clone());
  }

  pub fn add_character_to_setting(&mut self, setting: u64, character: u64) -> Result<((u64,String),(u64,String),(u64,String)),String> {
    match self.characters.get(character) {
      Some(character) => match self.settings.get_mut(setting) {
        Some(setting) => {
          character.join_setting(setting);
          Ok(((self.id,self.name.clone()),(setting.id,setting.name.clone()),(character.id,character.name.clone())))
        },
        None => Err(format!("No setting with id {} found",setting)),
      },
      None => Err(format!("No character with id {} found",character)),
    }
  }

  pub fn to_rust(&self) -> String {
    let mut instructions: Vec<String> = Vec::new();
    instructions.push(format!("let mut story: Story = Story::new({},&\"{}\".to_string());",self.id,self.name));
    for (id,character) in &self.characters {
      instructions.push(character.to_rust());
      instructions.push(format!("story.characters.insert({},character);",id));
    }
    for (id,setting) in &self.settings {
      instructions.push(setting.to_rust());
      instructions.push(format!("story.settings.insert({},setting);",id));
    }
    instructions.join("\n")
  }
}
