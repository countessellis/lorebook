use std::{
  io::{stdin,stdout,Write},
};

use crate::config::*;
use crate::tome::*;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config: Config,
  pub(crate) tome:   Tome,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> Self {
    print!("Provide a name for this Lorebook tome: ");
    stdout().flush().ok();
    let mut name = String::new();
    let _ = stdin().read_line(&mut name);
    Self {
     config: config.clone(),
     tome:   Tome::new(0,&name.trim().to_string()),
    }
  }
}
