use std::{
  io::{stdin,stdout,Write},
  thread,
  time::Duration,
};

use super::cli::CLI;

use crate::defaults::*;

///////////// CLI

impl CLI {
  pub(crate) fn interact(&mut self) {
    println!("
{}
------------------------
  Type 'exit' to quit.
------------------------
",self.config.get("intro"));
    loop {
      let tome = self.library.active();
      let prompt = self.config.get("prompt");
      let template = if prompt.is_empty() { "{}> " } else { &prompt };
      if let Some(tome) = tome {
        print!("{}",template.replace("{}", &tome.name));
      } else {
        print!("{}",prompt);
      }
      stdout().flush().ok();
      let mut request = String::new();
      let _ = stdin().read_line(&mut request);
      if !request.trim().is_empty() {
        let mut args: Vec<&str> = request.trim().split(" ").collect();
        let command: &str = args.remove(0);
        let command: String = command.to_lowercase();
        if command.as_str() == "exit" || command.as_str() == "quit" { break }
        self.process(command.as_str(),&args);
      }
      thread::sleep(Duration::from_secs(DEFAULT_WAIT));
    }
  }
}
