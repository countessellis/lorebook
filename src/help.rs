use std::fmt;
use std::collections::HashMap;

use crate::config::*;
use crate::defaults::*;
use crate::fields::*;

use crate::util;

///////////// Help

#[derive(Debug,Clone)]
pub(crate) struct Help {
  config: Config,
}

impl fmt::Display for Help {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let details: Vec<String> = vec![
      format!("  --mode <MODE>                        Mode to run in (default: {}, choices: {})",DEFAULT_MODE,MODE_LIST.join(", ")),
      format!("  --help                               Display this help message"),
    ];
    let fields: Fields = Fields::new(&HashMap::from([
      ("RUNAS",self.config.get("ranas")),
      ("DESCRIPTION",DESCRIPTION.to_string()),
      ("DETAILS",details.join("\n")),
      ("COPYRIGHT",COPYRIGHT.to_string()),
      ("AUTHORS",AUTHORS.to_string()),
    ]));
    let help: String = util::render(&HELP_TEMPLATE.to_string(),&fields);
    write!(f, "{}",help)
  }
}

impl Help {
  pub(crate) fn new(config: &Config) -> Help {
    Help { config: config.clone() }
  }
}

