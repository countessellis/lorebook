use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use ::lorebook::init;
use ::lorebook::logger;

///////////// Main

fn main() {
  logger::init();
  init::run();
}
