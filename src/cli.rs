use std::{
  io::{stdin,stdout,Write},
  thread,
  time::Duration,
};

use crate::util;

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
          "compose" => {
            self.tome.create_story(&args[0].to_string());
            println!("Starting to compose {}.",args[0]);
          },
          "new" => {
            if args.len() > 1 {
              match args[0] {
                "story" => {
                  let name: String = args[1..].join(" ");
                  self.tome.create_story(&name);
                  println!("Starting to compose a story called {}.",name);
                },
                "character" => {
                  let name: String = args[1..].join(" ");
                  self.tome.create_character(&name);
                  println!("Creating new character {}.",name);
                },
                _ => {
                  log::error!("Creating a new {} not implemented.",args[0]);
                  eprintln!("Creating a new {} not implemented.",args[0]);
                },
              }
            } else {
              log::error!("To create new things, please specify the thing (story,character) and its name.");
              eprintln!("To create new things, please specify the thing (story,character) and its name.");
            }
          },
          "contents" => {
            let list: Vec<(u64,String)> = self.tome.list_stories();
            let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
            self.tome.list_stories().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
          },
          "list" => {
            if args.len() > 0 {
              match args[0] {
                "stories" => {
                  let list: Vec<(u64,String)> = self.tome.list_stories();
                  let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
                  self.tome.list_stories().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
                },
                "characters" => {
                  let list: Vec<(u64,String)> = self.tome.list_characters();
                  let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
                  self.tome.list_characters().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
                },
                _ => {
                  log::error!("listing {} not implemented.",args[0]);
                  eprintln!("Creating {} not implemented.",args[0]);
                },
              }
            } else {
              log::error!("To list things, please specify the thing (stories,characters).");
              eprintln!("To list things, please specify the thing (stories,characters).");
            }
          },
          "add" => {
            if args.len() > 4 {
              match (args[0],args[1].parse::<u64>(),args[3],args[4].parse::<u64>()) {
                ("character",Ok(thing),"story",Ok(add_to)) => {
                  match self.tome.add_character_to_story(thing,add_to) {
                    Ok(((_,character),(_,story))) => {
                      log::info!("Added the {} called {} to the {} called {}",args[0],character,args[3],story);
                      println!("Added the {} called {} to the {} called {}",args[0],character,args[3],story);
                    },
                    Err(err) => {
                      log::error!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
                      eprintln!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
                    },
                  }
                },
                _ => {
                  log::error!("Adding {} {} to {} {} isn't implemented.",util::get_article(args[0]),args[0],util::get_article(args[3]),args[3]);
                  eprint!("Adding {} {} to {} {} isn't implemented.",util::get_article(args[0]),args[0],util::get_article(args[3]),args[3]);
                },
              }
            } else {
              log::error!("To add things, please specify the thing (character), which specific thing (number), to another thing (story), and which other thing (number).");
              eprintln!("To add things, please specify the thing (character), which specific thing (number), to another thing (story), and which other thing (number).");
            }
            
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
