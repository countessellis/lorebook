

use crate::collections::Table;

use crate::character::*;
use crate::story::*;
use crate::setting::*;

///////////// Tome

#[derive(Debug,Clone)]
pub struct Tome {
  pub id:         u64,
  pub name:       String,
  pub characters: Table<Character>,
  pub stories:    Table<Story>,
  pub settings:   Table<Setting>,
}

impl Tome {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:         id,
      name:       name.clone(),
      characters: Table::new(),
      stories:    Table::new(),
      settings:   Table::new(),
    }
  }

  pub fn create_story(&mut self, name: &String) -> u64 {
    log::info!("Creating new story called {}.",name);
    let id: u64 = self.stories.next();
    let story: Story = Story::new(id,name);
    self.stories.insert(id,story);
    id
  }

  pub fn create_character(&mut self, name: &String) -> u64 {
    log::info!("Creating new character called {}.",name);
    let id: u64 = self.characters.next();
    let character: Character = Character::new(id,name);
    self.characters.insert(id,character);
    id
  }

  pub fn create_setting(&mut self, name: &String) -> u64 {
    log::info!("Creating new setting called {}.",name);
    let id: u64 = self.settings.next();
    let setting: Setting = Setting::new(id,name);
    self.settings.insert(id,setting);
    id
  }

  pub fn list_stories(&self) -> Vec<(u64,String)> {
    let stories: Vec<(u64,String)> = self.stories.iter().map(|(id,story)| (id,story.name.clone())).collect();
    log::debug!("Returning a list of {} stories.",stories.len());
    stories
  }

  pub fn list_characters(&self) -> Vec<(u64,String)> {
    let characters: Vec<(u64,String)> = self.characters.iter().map(|(id,character)| (id,character.name.clone())).collect();
    log::debug!("Returning a list of {} characters.",characters.len());
    characters
  }

  pub fn list_settings(&self) -> Vec<(u64,String)> {
    let settings: Vec<(u64,String)> = self.settings.iter().map(|(id,setting)| (id,setting.name.clone())).collect();
    log::debug!("Returning a list of {} settings.",settings.len());
    settings
  }

  pub fn add_character_to_story(&mut self, story: u64, character: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character) {
      Some(character) => match self.stories.get_mut(story) {
        Some(story) => {
          character.join_story(story);
          Ok(((character.id,character.name.clone()),(story.id,story.name.clone())))
        },
        None => Err(format!("No story with id {} found",story)),
      },
      None => Err(format!("No character with id {} found",character)),
    }
  }

  pub fn add_setting_to_story(&mut self, story: u64, setting: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.settings.get(setting) {
      Some(setting) => match self.stories.get_mut(story) {
        Some(story) => {
          setting.join_story(story);
          Ok(((story.id,story.name.clone()),(setting.id,setting.name.clone())))
        },
        None => Err(format!("No story with id {} found",story)),
      },
      None => Err(format!("No setting with id {} found",setting)),
    }
  }

  pub fn add_character_to_setting(&mut self, setting: u64, character: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character) {
      Some(character) => match self.settings.get_mut(setting) {
        Some(setting) => {
          character.join_setting(setting);
          Ok(((character.id,character.name.clone()),(setting.id,setting.name.clone())))
        },
        None => Err(format!("No setting with id {} found",setting)),
      },
      None => Err(format!("No character with id {} found",character)),
    }
  }

  pub fn add_character_to_setting_in_story(&mut self, story: u64, setting: u64, character: u64) -> Result<((u64,String),(u64,String),(u64,String)),String> {
    match self.stories.get_mut(story) {
      Some(story) => story.add_character_to_setting(setting,character),
      None => Err(format!("No story with id {} found",story)),
    }
  }

  pub fn to_rust(&self) -> String {
    let mut instructions: Vec<String> = Vec::new();
    instructions.push(format!("let mut tome: Tome = Tome::new({},&\"{}\".to_string());",self.id,self.name));
    for (id,character) in &self.characters {
      instructions.push(character.to_rust());
      instructions.push(format!("tome.characters.insert({},character);",id));
    }
    for (id,setting) in &self.settings {
      instructions.push(setting.to_rust());
      instructions.push(format!("tome.settings.insert({},setting);",id));
    }
    for (id,story) in &self.stories {
      instructions.push(story.to_rust());
      instructions.push(format!("tome.stories.insert({},story);",id));
    }
    instructions.join("\n")
  }
}
