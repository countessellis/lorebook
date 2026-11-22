use crate::collections::Table;

use crate::character::*;
use crate::setting::*;

///////////// Story

#[derive(Debug,Clone)]
pub struct Story {
  pub name:       String,
  pub characters: Table<Character>,
  pub settings:   Table<Setting>,
}

impl Story {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name:       name.into(),
      characters: Table::new(),
      settings:   Table::new(),
    }
  }

  pub fn add_character(&mut self, character: &Character) -> u64 {
    let id: u64 = self.characters.next();
    log::info!("Story {}: Adding the character called {}.",self.name,character.name);
    self.characters.insert(id,character.clone());
    id
  }

  pub fn add_setting(&mut self, setting: &Setting) -> u64 {
    let id: u64 = self.settings.next();
    log::info!("Story {}: Adding the setting called {}.",self.name,setting.name);
    self.settings.insert(id,setting.clone());
    log::info!("Story {}: Importing {} characters from setting.",self.name,setting.characters.len());
    for (_,character) in &setting.characters {
      self.add_character(character);
    }
    id
  }

  pub fn add_character_to_setting(&mut self, setting_id: u64, character_id: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character_id) {
      Some(character) => match self.settings.get_mut(setting_id) {
        Some(setting) => {
          character.join_setting(setting);
          Ok(((setting_id,setting.name.clone()),(character_id,character.name.clone())))
        },
        None => Err(format!("Story {}: No setting with id {} found",self.name,setting_id)),
      },
      None => Err(format!("Story {}: No character with id {} found",self.name,character_id)),
    }
  }

  pub fn list_characters(&self) -> Vec<(u64,String)> {
    let characters: Vec<(u64,String)> = self.characters.iter().map(|(id,character)| (id,character.name.clone())).collect();
    log::debug!("Story {}: Returning a list of {} characters.",self.name,characters.len());
    characters
  }

  pub fn list_settings(&self) -> Vec<(u64,String)> {
    let settings: Vec<(u64,String)> = self.settings.iter().map(|(id,setting)| (id,setting.name.clone())).collect();
    log::debug!("Story {}: Returning a list of {} settings.",self.name,settings.len());
    settings
  }

  pub fn list_characters_in_setting(&self, setting: u64) -> Result<Vec<(u64,String)>,String> {
    match self.settings.get(setting) {
      Some(setting) => {
        let characters: Vec<(u64,String)> = setting.list_characters();
        log::debug!("Story {}: Returning a list of {} characters from the setting called {}.",self.name,characters.len(),setting.name);
        Ok(characters)
      },
      None => Err(format!("Story {}: No setting with id {} found",self.name,setting)),
    }
  }

  pub fn to_rust(&self) -> String {
    log::debug!("Story {}: Converting to rust code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let mut story: Story = Story::new(\"{}\");",name));
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

  pub fn to_lore(&self) -> String {
    log::debug!("Story {}: Converting to lorebook code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let story = new Story {{ name = \"{}\" }};",name));
    for (id,character) in &self.characters {
      instructions.push(character.to_lore());
      instructions.push(format!("setting.characters.insert({},character);",id));
    }
    for (id,setting) in &self.settings {
      instructions.push(setting.to_lore());
      instructions.push(format!("story.settings.insert({},setting);",id));
    }
    instructions.join("\n")
  }
}
