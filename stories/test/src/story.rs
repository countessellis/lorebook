use lorebook::{Character,Story,Tome};

pub fn start() -> Tome {
  let mut tome: Tome = Tome::new(0,&"Book of Lore".to_string());
  let mut character: Character = Character::new(1,&"Aladdin".to_string());
  tome.characters.insert(1,character);
  let mut story: Story = Story::new(1,&"The City of Brass".to_string());
  let mut character: Character = Character::new(1,&"Aladdin".to_string());
  story.characters.insert(1,character);
  tome.stories.insert(1,story);
  tome
}
