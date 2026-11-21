use std::{
  io::{stdin,stdout,Write},
};

use crate::collections::Config;
use crate::elements::Library;

///////////// CLI

#[derive(Clone)]
pub(crate) struct CLI {
  pub(crate) config:  Config,
  pub(crate) library: Library,
}

impl CLI {
  pub(crate) fn new(config: &Config) -> Self {
    print!("Provide a name for this Lorebook tome: ");
    stdout().flush().ok();
    let mut name = String::new();
    let _ = stdin().read_line(&mut name);
    let mut library: Library = Library::new();
    let id: u64 = library.create_tome(&name.trim().to_string());
    library.set_active(id);
    Self {
     config:  config.clone(),
     library: library,
    }
  }
}
