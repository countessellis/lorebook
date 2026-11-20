use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn create(&mut self, element: &str, name: &String) {
    if element == "tome" {
      let id: u64 = self.library.create_tome(&name);
      println!("Starting to a new tome called {}. (id: {})",name,id);
    } else if let Some(tome) = self.library.active() {
      match element {
        "story" => {
          let id: u64 = tome.create_story(&name);
          println!("Starting to compose a story called {}. (id: {})",name,id);
        },
        "character" => {
          let id: u64 = tome.create_character(&name);
          println!("Creating new character {}. (id: {})",name,id);
        },
        "setting" => {
          let id: u64 = tome.create_setting(&name);
          println!("Creating new setting {}. (id: {})",name,id);
        },
        _ => {
          log::error!("Creating a new {} not implemented.",element);
          eprintln!("Creating a new {} not implemented.",element);
        },
      }
    } else {
      log::error!("No active tome was found, so can't list {}.",element);
      eprintln!("No active tome was found, so can't list {}.",element);
    }
  }
}
