use crate::story::*;

///////////// Character

#[derive(Debug,Clone)]
pub struct Character {
  pub id:   u64,
  pub name: String
}

impl Character {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:   id,
      name: name.clone(),
    }
  }

  pub fn join_story(&self, story: &mut Story) {
    log::info!("Adding the character called {} to {}.",self.name,story.name);
    story.add_character(&self);
  }
}
