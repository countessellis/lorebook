use super::Sequence;

#[derive(Debug,Clone)]
pub struct Table<T> {
  sequence: Sequence,
  items:    Vec<Option<T>>,
}

impl<T> Table<T> {
  pub fn new() -> Self {
    Self {
      sequence: Default::default(),
      items:    Vec::new(),
    }
  }

  pub fn insert(&mut self, id: u64, item: T) {
    let idx = id as usize;
    if idx >= self.items.len() {
      self.items.resize_with(idx + 1, || None);
    }
    self.items[idx] = Some(item);
  }

  pub fn get(&self, id: u64) -> Option<&T> {
    self.items.get(id as usize)?.as_ref()
  }

  pub fn remove(&mut self, id: u64) -> Option<T> {
    self.items.get_mut(id as usize)?.take()
  }

  pub fn next(&mut self) -> u64 {
    self.sequence.next()
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.items.iter().flatten()
  }
}

