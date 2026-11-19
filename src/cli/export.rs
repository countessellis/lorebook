use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn export(&mut self, args: &Vec<&str>) {
    if args.len() > 1 {
      match args[0] {
        "as" => {
          match args[1] {
            "rust" => {
              println!("{}",self.tome.to_rust());
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
      log::error!("To export provide either 'as FORMAT' or 'THING ID as FORMAT' where FORMAT is (rust) or 'THING' is (tome,story,character,setting) and ID is the id of the thing.");
      eprintln!("To export provide either 'as FORMAT' or 'THING ID as FORMAT' where FORMAT is (rust) or 'THING' is (tome,story,character,setting) and ID is the id of the thing.");
    }
  }
}
