use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn process(&mut self, command: &str, args: &Vec<&str>) {
    match command {
      "open" => {
        if args.len() > 0 {
          self.open(args[0]);
        } else {
          log::error!("To open a tome, please specify the id of the tome.");
          eprintln!("To open a tome, please specify the id of the tome.");
        }
      },
      "compose"|"new"|"create" => {},
      "contents" => {
        self.list("stories");
      },
      "list" => {
        if args.len() > 0 {
          self.list(args[0]);
        } else {
          log::error!("To list things, please specify the thing (stories,characters).");
          eprintln!("To list things, please specify the thing (stories,characters).");
        }
      },
      "add" => {
        if args.len() > 4 {
          self.join(&args);
        } else {
          log::error!("To add things, please specify the thing (character), which specific thing (number), to another thing (story), and which other thing (number).");
          eprintln!("To add things, please specify the thing (character), which specific thing (number), to another thing (story), and which other thing (number).");
        }
        
      },
      "export" => {
        self.export(&args);
      },
      _ => {
        log::error!("Command {} not implemented.",command);
        eprintln!("Command {} not implemented.",command);
      },
    }
  }
}
