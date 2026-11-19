use crate::setting::*;
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
    log::info!("Character {}: Joining story called {}.",self.name,story.name);
    story.add_character(&self);
  }

  pub fn join_setting(&self, setting: &mut Setting) {
    log::info!("Character {}: Joining setting called {}.",self.name,setting.name);
    setting.add_character(&self);
  }

  pub fn to_rust(&self) -> String {
    let mut instructions: Vec<String> = Vec::new();
    instructions.push(format!("let character: Character = Character::new({},&\"{}\".to_string());",self.id,self.name));
    instructions.join("\n")
  }
}
