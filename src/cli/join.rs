use super::cli::CLI;

use crate::util;

///////////// CLI

impl CLI {
  pub(crate) fn join(&mut self, args: &Vec<&str>) {
    if let Some(tome) = self.library.active() {
      match (args[0],args[1].parse::<u64>(),args[3],args[4].parse::<u64>()) {
        ("character",Ok(thing),"story",Ok(add_to)) => {
          match tome.add_character_to_story(thing,add_to) {
            Ok(((_,story),(_,character))) => {
              log::info!("Added the {} called {} to the {} called {}",args[0],character,args[3],story);
              println!("Added the {} called {} to the {} called {}",args[0],character,args[3],story);
            },
            Err(err) => {
              log::error!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
              eprintln!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
            },
          }
        },
        ("setting",Ok(thing),"story",Ok(add_to)) => {
          match tome.add_setting_to_story(thing,add_to) {
            Ok(((_,story),(_,setting))) => {
              log::info!("Added the {} called {} to the {} called {}",args[0],setting,args[3],story);
              println!("Added the {} called {} to the {} called {}",args[0],setting,args[3],story);
            },
            Err(err) => {
              log::error!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
              eprintln!("Failed to add the {} {} to the {} {}: {}",args[0],args[1],args[3],args[4],err);
            },
          }
        },
        ("character",Ok(thing),"setting",Ok(add_to)) => {
          match tome.add_character_to_setting(thing,add_to) {
            Ok(((_,setting),(_,character))) => {
              log::info!("Added the {} called {} to the {} called {}",args[0],character,args[3],setting);
              println!("Added the {} called {} to the {} called {}",args[0],character,args[3],setting);
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
      log::error!("No active tome was found, so can't join.");
      eprintln!("No active tome was found, so can't join.");
    }
  }
}
