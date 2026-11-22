use std::{env::{args,Args},fmt,str::FromStr};

use crate::defaults::*;

use super::*;
//use super::{Library,Tome,Story,Setting,Character};

///////////// Element

#[derive(Debug,Clone)]
pub enum Element {
  Library(Library),
  Tome(Tome),
  Story(Story),
  Setting(Setting),
  Character(Character),
}
