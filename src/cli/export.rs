use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn export(&mut self, args: &Vec<&str>) {
    if args.len() > 1 {
      if let Some(tome) = self.library.active() {
        match args[0] {
          "as" => {
            match args[1] {
              "rust" => {
                println!("{}",tome.to_rust());
              },
              _ => {
                log::error!("Exporting to {} isn't implemented.",args[1]);
                eprint!("Exporting to {} isn't implemented.",args[1]);
              },
            }
          },
          _ => {
            log::error!("Exporting {} isn't implemented.",args[0]);
            eprint!("Exporting {} isn't implemented.",args[0]);
          },
        }
      } else {
        log::error!("No active tome was found, so can't export.");
        eprintln!("No active tome was found, so can't export.");
      }
    } else {
      log::error!("To export provide either 'as FORMAT' or 'THING ID as FORMAT' where FORMAT is (rust) or 'THING' is (tome,story,character,setting) and ID is the id of the thing.");
      eprintln!("To export provide either 'as FORMAT' or 'THING ID as FORMAT' where FORMAT is (rust) or 'THING' is (tome,story,character,setting) and ID is the id of the thing.");
    }
  }
}
