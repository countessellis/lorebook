use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn process(&mut self, command: &str, args: &Vec<&str>) {
    match command {
      "compose" => {
        self.create("story",&args.join(" "));
      },
      "new"|"create" => {
        if args.len() > 1 {
          self.create(args[0],&args[1..].join(" "));
        } else {
          log::error!("To create new things, please specify the thing (story,character) and its name.");
          eprintln!("To create new things, please specify the thing (story,character) and its name.");
        }
      },
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
