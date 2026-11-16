use std::collections::HashMap;

use crate::story::*;

///////////// Tome

#[derive(Debug,Clone)]
pub(crate) struct Tome {
  pub(crate) stories: HashMap<String,Story>,
}

impl Tome {
  pub(crate) fn new() -> Self {
    Self { stories: HashMap::new() }
  }

  pub(crate) fn compose(&mut self, name: &String) {
    log::info!("Creating new story called {}.",name);
    let story: Story = Story::new();
    self.stories.insert(name.clone(),story);
  }

  pub(crate) fn list(&self) -> Vec<String> {
    let stories: Vec<String> = self.stories.keys().cloned().collect();
    log::debug!("Returning a list of {} stories.",stories.len());
    stories
  }
}
