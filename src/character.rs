

///////////// Character

#[derive(Debug,Clone)]
pub struct Character {
  pub id:   u64,
  pub name: String
}

impl Character {
  pub fn new(id: u64, name: &String) -> Self {
    Self {
      id:   id,
      name: name.clone(),
    }
  }
}
