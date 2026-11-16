use std::{
  io::{stdin,stdout,Write},
  thread,
  time::Duration,
};

use crate::config::*;
use crate::defaults::*;
use crate::tome::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
  pub(crate) tome:   Tome,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> Self {
    Self {
     config: config.clone(),
     tome:   Tome::new(),
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
          "new"|"compose" => {
            self.tome.compose(&args[0].to_string());
            println!("Starting to compose {}.",args[0]);
          },
          "contents"|"list" => {
            self.tome.list().iter().for_each(|name| println!("{}",name));
          },
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
