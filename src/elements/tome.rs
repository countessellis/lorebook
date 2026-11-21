

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
  pub fn new(id: u64, name: impl Into<String>) -> Self {
    Self {
      id:         id,
      name:       name.into(),
      characters: Table::new(),
      stories:    Table::new(),
      settings:   Table::new(),
    }
  }

  pub fn create_story(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new story called {}.",self.name,name);
    let id: u64 = self.stories.next();
    let story: Story = Story::new(id,name);
    self.stories.insert(id,story);
    id
  }

  pub fn create_character(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new character called {}.",self.name,name);
    let id: u64 = self.characters.next();
    let character: Character = Character::new(id,name);
    self.characters.insert(id,character);
    id
  }

  pub fn create_setting(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new setting called {}.",self.name,name);
    let id: u64 = self.settings.next();
    let setting: Setting = Setting::new(id,name);
    self.settings.insert(id,setting);
    id
  }

  pub fn list_stories(&self) -> Vec<(u64,String)> {
    let stories: Vec<(u64,String)> = self.stories.iter().map(|(id,story)| (id,story.name.clone())).collect();
    log::debug!("Tome {}: Returning a list of {} stories.",self.name,stories.len());
    stories
  }

  pub fn list_characters(&self) -> Vec<(u64,String)> {
    let characters: Vec<(u64,String)> = self.characters.iter().map(|(id,character)| (id,character.name.clone())).collect();
    log::debug!("Tome {}: Returning a list of {} characters.",self.name,characters.len());
    characters
  }

  pub fn list_settings(&self) -> Vec<(u64,String)> {
    let settings: Vec<(u64,String)> = self.settings.iter().map(|(id,setting)| (id,setting.name.clone())).collect();
    log::debug!("Tome {}: Returning a list of {} settings.",self.name,settings.len());
    settings
  }

  pub fn list_characters_in_story(&self, story: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story) {
      Some(story) => {
        let characters: Vec<(u64,String)> = story.list_characters();
        log::debug!("Tome {}: Returning a list of {} characters from the story called {}.",self.name,characters.len(),story.name);
        Ok(characters)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
    }
  }

  pub fn list_settings_in_story(&self, story: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story) {
      Some(story) => {
        let settings: Vec<(u64,String)> = story.list_settings();
        log::debug!("Tome {}: Returning a list of {} settings from the story called {}.",self.name,settings.len(),story.name);
        Ok(settings)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
    }
  }

  pub fn list_characters_in_setting_in_story(&self, story: u64, setting: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story) {
      Some(story) => {
        let settings: Vec<(u64,String)> = match story.list_characters_in_setting(setting) {
          Ok(settings) => settings,
          Err(err) => {
            log::error!("Tome {}: Failed to get list of characters from setting in story: {}",self.name,err);
            return Err(format!("Tome {}: Failed to get list of characters from setting in story: {}",self.name,err));
          }
        };
        log::debug!("Tome {}: Returning a list of {} settings from the story called {}.",self.name,settings.len(),story.name);
        Ok(settings)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
    }
  }

  pub fn add_character_to_story(&mut self, story: u64, character: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character) {
      Some(character) => match self.stories.get_mut(story) {
        Some(story) => {
          character.join_story(story);
          Ok(((character.id,character.name.clone()),(story.id,story.name.clone())))
        },
        None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
      },
      None => Err(format!("Tome {}: No character with id {} found",self.name,character)),
    }
  }

  pub fn add_setting_to_story(&mut self, story: u64, setting: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.settings.get(setting) {
      Some(setting) => match self.stories.get_mut(story) {
        Some(story) => {
          setting.join_story(story);
          Ok(((story.id,story.name.clone()),(setting.id,setting.name.clone())))
        },
        None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
      },
      None => Err(format!("Tome {}: No setting with id {} found",self.name,setting)),
    }
  }

  pub fn add_character_to_setting(&mut self, setting: u64, character: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character) {
      Some(character) => match self.settings.get_mut(setting) {
        Some(setting) => {
          character.join_setting(setting);
          Ok(((character.id,character.name.clone()),(setting.id,setting.name.clone())))
        },
        None => Err(format!("Tome {}: No setting with id {} found",self.name,setting)),
      },
      None => Err(format!("Tome {}: No character with id {} found",self.name,character)),
    }
  }

  pub fn add_character_to_setting_in_story(&mut self, story: u64, setting: u64, character: u64) -> Result<((u64,String),(u64,String),(u64,String)),String> {
    match self.stories.get_mut(story) {
      Some(story) => story.add_character_to_setting(setting,character),
      None => Err(format!("Tome {}: No story with id {} found",self.name,story)),
    }
  }

  pub fn to_rust(&self) -> String {
    log::debug!("Tome {}: Converting to rust code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let mut tome: Tome = Tome::new({},\"{}\");",self.id,name));
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

  pub fn to_lore(&self) -> String {
    log::debug!("Tome {}: Converting to lorebook code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let story = new Tome {{ name = \"{}\" }};",name));
    for (id,character) in &self.characters { 
      instructions.push(character.to_lore());
      instructions.push(format!("setting.characters.insert({},character);",id));
    }
    for (id,setting) in &self.settings {
      instructions.push(setting.to_lore());
      instructions.push(format!("story.settings.insert({},setting);",id)); 
    }
    for (id,story) in &self.stories {
      instructions.push(story.to_lore());
      instructions.push(format!("tome.stories.insert({},story);",id));
    }
    instructions.join("\n")
  }
}
