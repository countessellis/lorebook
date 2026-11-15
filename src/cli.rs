use std::{
  io::{stdin,stdout,Write},
  thread,
  time::Duration,
};

use crate::config::*;
use crate::defaults::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> Self {
    
    Self {
     config: config.clone(),
    }
  }

  pub(crate) fn interact(&mut self) {
    println!("
{}
------------------------
  Type 'exit' to quit.
------------------------
",self.config.get("intro"));
    loop {
      print!("{} ",self.config.get("prompt"));
      stdout().flush().ok();
      let mut request = String::new();
      let _ = stdin().read_line(&mut request);
      if !request.trim().is_empty() {
        let mut args: Vec<&str> = request.trim().split(" ").collect();
        let command: &str = args.remove(0);
        match command.to_lowercase().as_str() {
          "exit"|"quit" => {
            break;
          },
          _ => {},
        }
      }
      thread::sleep(Duration::from_secs(DEFAULT_WAIT));
    }
  }
}
