

use crate::collections::Table;

use crate::character::*;
use crate::story::*;

///////////// Tome

#[derive(Debug,Clone)]
pub struct Tome {
  pub characters: Table<Character>,
  pub stories:    Table<Story>,
}

impl Tome {
  pub fn new() -> Self {
    Self {
      characters: Table::new(),
      stories:    Table::new(),
    }
  }

  pub fn create_story(&mut self, name: &String) {
    log::info!("Creating new story called {}.",name);
    let id: u64 = self.stories.next();
    let story: Story = Story::new(id,name);
    self.stories.insert(id,story);
  }

  pub fn create_character(&mut self, name: &String) {
    log::info!("Creating new character called {}.",name);
    let id: u64 = self.characters.next();
    let character: Character = Character::new(id,name);
    self.characters.insert(id,character);
  }

  pub fn list_stories(&self) -> Vec<(u64,String)> {
    let stories: Vec<(u64,String)> = self.stories.iter().map(|story| (story.id,story.name.clone())).collect();
    log::debug!("Returning a list of {} stories.",stories.len());
    stories
  }

  pub fn list_characters(&self) -> Vec<(u64,String)> {
    let characters: Vec<(u64,String)> = self.characters.iter().map(|character| (character.id,character.name.clone())).collect();
    log::debug!("Returning a list of {} characters.",characters.len());
    characters
  }
}
