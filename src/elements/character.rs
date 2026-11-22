use crate::setting::*;
use crate::story::*;

///////////// Character

#[derive(Debug,Clone)]
pub struct Character {
  pub name: String
}

impl Character {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
    }
  }

  pub fn join_story(&self, story: &mut Story) -> u64 {
    log::info!("Character {}: Joining story called {}.",self.name,story.name);
    story.add_character(&self)
  }

  pub fn join_setting(&self, setting: &mut Setting) -> u64 {
    log::info!("Character {}: Joining setting called {}.",self.name,setting.name);
    setting.add_character(&self)
  }

  pub fn to_rust(&self) -> String {
    log::debug!("Character {}: Converting to rust code.",self.name);
    let name = self.name.escape_default();
    format!("let character: Character = Character::new(\"{}\");",name)
  }

  pub fn to_lore(&self) -> String {
    log::debug!("Character {}: Converting to lorebook code.",self.name);
    let name = self.name.escape_default();
    format!("let character = new Character {{ name = \"{}\" }};",name)
  }
}
