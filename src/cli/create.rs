use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn create(&mut self, element: &str, name: &String) {
    match element {
      "story" => {
        let id: u64 = self.tome.create_story(&name);
        println!("Starting to compose a story called {}. (id: {})",name,id);
      },
      "character" => {
        let id: u64 = self.tome.create_character(&name);
        println!("Creating new character {}. (id: {})",name,id);
      },
      "setting" => {
        let id: u64 = self.tome.create_setting(&name);
        println!("Creating new setting {}. (id: {})",name,id);
      },
      _ => {
        log::error!("Creating a new {} not implemented.",element);
        eprintln!("Creating a new {} not implemented.",element);
      },
    }
  }
}
