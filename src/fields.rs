use std::collections::HashMap;

///////////// Fields

#[derive(Clone,Debug)]
pub struct Fields {
  pub fields: HashMap<String,String>,
}

impl Fields {
  pub fn new(source: &HashMap<&str,String>) -> Fields {
    let mut fields: HashMap<String,String> = HashMap::new();
    for (key,value) in source {
      fields.insert(key.to_string(),value.clone());
    }
    Fields { fields: fields }
  }

  pub fn insert(&mut self,key: &str, value: String) {
    self.fields.insert(key.to_string(),value);
  }

}
