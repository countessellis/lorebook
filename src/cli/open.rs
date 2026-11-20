use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn open(&mut self, id: &str) {
    match id.parse::<u64>() {
      Ok(id) => {
        if let Some(tome) = self.library.set_active(id) {
          log::info!("Tome {} called {} is now open.",id,tome.name);
          println!("Tome {} called {} is now open.",id,tome.name);
        } else {
          log::warn!("Unable to open Tome {} which doesn't exist.",id);
          eprintln!("Unable to open Tome {} which doesn't exist.",id);
        }
      },
      Err(_) => {
        log::error!("Unable to parse {} into a tome id, please provide the tome id.",id);
        eprint!("Unable to parse {} into a tome id, please provide the tome id.",id);
      },
    }
      
  }
}
