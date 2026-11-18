use crate::version::*;
use crate::story;

use lorebook::Story;

///////////// Run

pub fn run() {
  println!("{}",version());
  log::info!("{}",version());
  let mut story: Story = story::start();
}
