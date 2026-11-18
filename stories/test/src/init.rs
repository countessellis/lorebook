use crate::version::*;
use crate::story;

use lorebook::Tome;

///////////// Run

pub fn run() {
  println!("{}",version());
  log::info!("{}",version());
  let mut tome: Tome = story::start();
  println!(
"  Stories:

{}

  Characters:

{}

Settings:

{}
",
    tome.list_stories().iter().map(|(id,name)| format!("{}: {}",id,name)).collect::<Vec<String>>().join("\n"),
    tome.list_characters().iter().map(|(id,name)| format!("{}: {}",id,name)).collect::<Vec<String>>().join("\n"),
    tome.list_settings().iter().map(|(id,name)| format!("{}: {}",id,name)).collect::<Vec<String>>().join("\n"),
  );
}
