

use crate::collections::Table;

use crate::character::*;
use crate::story::*;
use crate::setting::*;

///////////// Tome

#[derive(Debug,Clone)]
pub struct Tome {
  pub name:       String,
  pub characters: Table<Character>,
  pub stories:    Table<Story>,
  pub settings:   Table<Setting>,
}

impl Tome {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name:       name.into(),
      characters: Table::new(),
      stories:    Table::new(),
      settings:   Table::new(),
    }
  }

  pub fn create_story(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new story called {}.",self.name,name);
    let id: u64 = self.stories.next();
    let story: Story = Story::new(name);
    self.stories.insert(id,story);
    id
  }

  pub fn create_character(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new character called {}.",self.name,name);
    let id: u64 = self.characters.next();
    let character: Character = Character::new(name);
    self.characters.insert(id,character);
    id
  }

  pub fn create_setting(&mut self, name: &String) -> u64 {
    log::info!("Tome {}: Creating new setting called {}.",self.name,name);
    let id: u64 = self.settings.next();
    let setting: Setting = Setting::new(name);
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

  pub fn list_characters_in_story(&self, story_id: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story_id) {
      Some(story) => {
        let characters: Vec<(u64,String)> = story.list_characters();
        log::debug!("Tome {}: Returning a list of {} characters from the story called {}.",self.name,characters.len(),story.name);
        Ok(characters)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
    }
  }

  pub fn list_settings_in_story(&self, story_id: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story_id) {
      Some(story) => {
        let settings: Vec<(u64,String)> = story.list_settings();
        log::debug!("Tome {}: Returning a list of {} settings from the story called {}.",self.name,settings.len(),story.name);
        Ok(settings)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
    }
  }

  pub fn list_characters_in_setting_in_story(&self, story_id: u64, setting_id: u64) -> Result<Vec<(u64,String)>,String> {
    match self.stories.get(story_id) {
      Some(story) => {
        let settings: Vec<(u64,String)> = match story.list_characters_in_setting(setting_id) {
          Ok(settings) => settings,
          Err(err) => {
            log::error!("Tome {}: Failed to get list of characters from setting in story: {}",self.name,err);
            return Err(format!("Tome {}: Failed to get list of characters from setting in story: {}",self.name,err));
          }
        };
        log::debug!("Tome {}: Returning a list of {} settings from the story called {}.",self.name,settings.len(),story.name);
        Ok(settings)
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
    }
  }

  pub fn add_character_to_story(&mut self, story_id: u64, character_id: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character_id) {
      Some(character) => match self.stories.get_mut(story_id) {
        Some(story) => {
          character.join_story(story);
          Ok(((character_id,character.name.clone()),(story_id,story.name.clone())))
        },
        None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
      },
      None => Err(format!("Tome {}: No character with id {} found",self.name,character_id)),
    }
  }

  pub fn add_setting_to_story(&mut self, story_id: u64, setting_id: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.settings.get(setting_id) {
      Some(setting) => match self.stories.get_mut(story_id) {
        Some(story) => {
          setting.join_story(story);
          Ok(((story_id,story.name.clone()),(setting_id,setting.name.clone())))
        },
        None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
      },
      None => Err(format!("Tome {}: No setting with id {} found",self.name,setting_id)),
    }
  }

  pub fn add_character_to_setting(&mut self, setting_id: u64, character_id: u64) -> Result<((u64,String),(u64,String)),String> {
    match self.characters.get(character_id) {
      Some(character) => match self.settings.get_mut(setting_id) {
        Some(setting) => {
          character.join_setting(setting);
          Ok(((character_id,character.name.clone()),(setting_id,setting.name.clone())))
        },
        None => Err(format!("Tome {}: No setting with id {} found",self.name,setting_id)),
      },
      None => Err(format!("Tome {}: No character with id {} found",self.name,character_id)),
    }
  }

  pub fn add_character_to_setting_in_story(&mut self, story_id: u64, setting_id: u64, character_id: u64) -> Result<((u64,String),(u64,String),(u64,String)),String> {
    match self.stories.get_mut(story_id) {
      Some(story) => {
        match story.add_character_to_setting(setting_id,character_id) {
          Ok(((_,setting_name),(_,character_name))) => Ok(((story_id,story.name.clone()),(setting_id,setting_name),(character_id,character_name))),
          Err(err) => Err(err),
        } 
      },
      None => Err(format!("Tome {}: No story with id {} found",self.name,story_id)),
    }
  }

  pub fn to_rust(&self) -> String {
    log::debug!("Tome {}: Converting to rust code.",self.name);
    let mut instructions: Vec<String> = Vec::new();
    let name = self.name.escape_default();
    instructions.push(format!("let mut tome: Tome = Tome::new(\"{}\");",name));
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
