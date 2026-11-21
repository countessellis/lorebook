use std::collections::HashMap;

use crate::collections::Config;

use crate::util;

use crate::defaults::*;

///////////// Config

pub(crate) fn init() -> Config {
  Config::from_hashmap(&HashMap::from([
    ("ranas".to_string(),util::bin_name()),
    ("intro".to_string(),DEFAULT_INTRO.to_string()),
    ("prompt".to_string(),DEFAULT_PROMPT.to_string()),
  ]))
}
