use crate::collections::Table;

use crate::tome::*;

///////////// Library

#[derive(Debug,Clone)]
pub struct Library {
  pub library: Table<Tome>,
}

impl Library {
  pub fn new() -> Self {
    Self {
      library: Table::new(),
    }
  }

  pub fn create_tome(&mut self, name: &String) -> u64 {
    log::info!("Creating new tome called {}.",name);
    let id: u64 = self.library.next();
    let tome: Tome = Tome::new(id,name);
    self.library.insert(id,tome);
    id
  }

  pub fn list_tomes(&self) -> Vec<(u64,String)> {
    let tomes: Vec<(u64,String)> = self.library.iter().map(|(id,tome)| (id,tome.name.clone())).collect();
    log::debug!("Returning a list of {} tomes.",tomes.len());
    tomes
  }

  pub fn active(&mut self) -> Option<&mut Tome> {
    self.library.active()
  }

  pub fn set_active(&mut self, tome: u64) -> Option<&mut Tome> {
    if self.library.set_active(tome) {
      self.library.active()
    } else { None }
  }
}
