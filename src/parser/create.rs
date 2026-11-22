use super::*;
use crate::elements::*;

///////////// Parser

impl Parser {
  pub fn create(id: u64, statement: &str) -> Result<Element,String> {
    let details = match statement.trim().strip_prefix("new") {
      Some(details) => details.trim(),
      None => return Err(format!("Expected 'new' expression, got: {}",statement)),
    };
    let (element_type, block) = match details.find('{') {
      Some(idx) => (details[..idx].trim(),Some(details[idx..].trim())),
      None      => (details, None),
    };
    let mut properties = match Self::parse_property_block(block) {
      Ok(properties) => properties,
      Err(err)       => return Err(err),
    };
    let name = match Self::take_string(&mut properties,"name") {
      Ok(name) => name,
      Err(err) => return Err(err),
    };
    match element_type {
      "Library" => Ok(Element::Library(Library::new())),
      "Tome" => {
        let name = match Self::take_string(&mut properties, "name") {
          Ok(name) => name,
          Err(err) => return Err(err),
        };
        Ok(Element::Tome(Tome::new(&name)))
      },
      "Story" => {
        let name = match Self::take_string(&mut properties, "name") {
          Ok(name) => name,
          Err(err) => return Err(err),
        };
        Ok(Element::Story(Story::new(&name)))
      },
      "Setting" => {
        let name = match Self::take_string(&mut properties, "name") {
          Ok(name) => name,
          Err(err) => return Err(err),
        };
        Ok(Element::Setting(Setting::new(&name)))
      },
      "Character" => {
        let name = match Self::take_string(&mut properties, "name") {
          Ok(name) => name,
          Err(err) => return Err(err),
        };
        Ok(Element::Character(Character::new(&name)))
      },
      other => Err(format!("Unknown ElementType: '{}'", other)),
    }
  }

  pub fn create_cli(library: &mut Library, statement: &Vec<&str>) -> Result<String,String> {
    let element: String = statement[1].trim().to_lowercase();
    let name:    String = statement[2..].join(" ").trim().to_string();
    if element.as_str() == "tome" {
      let id: u64 = library.create_tome(&name);
      Ok(format!("Starting composing a new tome called {}. (id: {})",name,id))
    } else if let Some(tome) = library.active() {
      match element.as_str() {
        "story" => {
          let id: u64 = tome.create_story(&name);
          Ok(format!("Starting to compose a story called {}. (id: {})",name,id))
        },
        "character" => {
          let id: u64 = tome.create_character(&name);
          Ok(format!("Creating new character {}. (id: {})",name,id))
        },
        "setting" => {
          let id: u64 = tome.create_setting(&name);
          Ok(format!("Creating new setting {}. (id: {})",name,id))
        },
        _ => {
          Err(format!("Creating a new {} not implemented.",element))
        },
      }
    } else {
      Err(format!("No active tome was found, so can't list {}.",element))
    }
  }
}
