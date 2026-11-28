use std::collections::HashMap;

use crate::collections::config::*;

use crate::util;

use crate::defaults::*;

///////////// Config

pub(crate) fn init() -> Config {
  let configurator = Configurator {
    defaults: HashMap::from([
      ("ranas".to_string(),util::bin_name()),
      ("intro".to_string(),DEFAULT_INTRO.to_string()),
      ("prompt".to_string(),DEFAULT_PROMPT.to_string()),
    ]),
    file: Some((DEFAULT_CONFIG_DIR.to_string(),DEFAULT_CONFIG_FILE.to_string())),
    ignore: vec![
      "--cli".to_string(),
      "--help".to_string(),
      "-h".to_string(),
      "--version".to_string(),
      "-V".to_string(),
    ],
    arguments: Vec::new(),
    flags: HashMap::new(),
  };
  configurator.to_config()
}

