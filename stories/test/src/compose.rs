#![allow(unused_mut,unused_variables,unused_imports)]

use lorebook::{Character,Setting,Story,Tome};

pub fn start() -> Tome {
  let mut tome: Tome = Tome::new(0,&"Book of Lore".to_string());
  let character: Character = Character::new(1,&"Alladin".to_string());
  tome.characters.insert(1,character);
  let mut setting: Setting = Setting::new(1,&"The City of Brass".to_string());
  let character: Character = Character::new(1,&"Alladin".to_string());
  setting.characters.insert(1,character);
  tome.settings.insert(1,setting);
  let mut story: Story = Story::new(1,&"The City of Brass".to_string());
  let character: Character = Character::new(1,&"Alladin".to_string());
  story.characters.insert(1,character);
  let mut setting: Setting = Setting::new(1,&"The City of Brass".to_string());
  let character: Character = Character::new(1,&"Alladin".to_string());
  setting.characters.insert(1,character);
  story.settings.insert(1,setting);
  tome.stories.insert(1,story);
  tome
}
