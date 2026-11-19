use crate::version::*;
use crate::compose;

use lorebook::Story;

///////////// Run

pub fn run() {
  println!("{}",version());
  log::info!("{}",version());
  let mut tome: Tome = compose::start();
}
