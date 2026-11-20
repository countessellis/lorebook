use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn list(&mut self, element: &str) {
    if element == "tomes" {
      let list: Vec<(u64,String)> = self.library.list_tomes();
      let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
      list.iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
    } else if let Some(tome) = self.library.active() {
      match element {
        "stories" => {
          let list: Vec<(u64,String)> = tome.list_stories();
          let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
          tome.list_stories().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
        },
        "characters" => {
          let list: Vec<(u64,String)> = tome.list_characters();
          let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
          tome.list_characters().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
        },
        "settings" => {
          let list: Vec<(u64,String)> = tome.list_settings();
          let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
          tome.list_settings().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
        },
        _ => {
          log::error!("Listing {} not implemented.",element);
          eprintln!("Listing {} not implemented.",element);
        },
      }
    } else {
      log::error!("No active tome was found, so can't list {}.",element);
      eprintln!("No active tome was found, so can't list {}.",element);
    }
  }
}
