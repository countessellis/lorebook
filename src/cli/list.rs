use super::cli::CLI;

///////////// CLI

impl CLI {
  pub(crate) fn list(&mut self, element: &str) {
    match element {
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
      "settings" => {
        let list: Vec<(u64,String)> = self.tome.list_settings();
        let id_width: usize = list.iter().map(|(id,_)| id).max().map(|id| id.to_string().len()).unwrap_or(0);
        self.tome.list_settings().iter().for_each(|(id,name)| println!("{:<width$} {}",id,name,width=id_width));
      },
      _ => {
        log::error!("Listing {} not implemented.",element);
        eprintln!("Listing {} not implemented.",element);
      },
    }
  }
}
