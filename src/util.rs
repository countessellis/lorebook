use std::{
  env,
  io::{stdin,stdout,Write},
};

use crate::defaults::*;
use crate::collections::Fields;

pub(crate) fn bin_name() -> String {
  match env::current_exe() {
    Ok(path) => {
       let path = path.as_path();
       match path.file_name() {
         Some(file_name) => match file_name.to_str() {
           Some(file_name) => return file_name.to_string(),
           None => {},
         },
         None => {},
       }
    },
    Err(_) => {},
  }
  BUILD_NAME.to_string()
}

pub fn prompt(prompt: String, default: String) -> String {
  print!("{} ",prompt);
  stdout().flush().ok();
  let mut response = String::new();
  let _ = stdin().read_line(&mut response);
  if response.trim().is_empty() { response = default };
  response.trim_end().to_string()
}

pub fn render(template: &str, fields: &Fields) -> String {
  let mut prompt: String = template.to_string();
  for (key,value) in &fields.fields {
    prompt = prompt.replace(&format!("{{{{{}}}}}",key.to_uppercase()),&value.trim());
  }
  prompt
}

pub fn get_article(text: &str) -> &'static str {
  let clean = text.trim();
  if clean.is_empty() { return "a" }
  let lower = clean.to_lowercase();
  if lower.starts_with("uni") || lower.starts_with("use") || lower.starts_with("euro") || lower.starts_with("util") || lower.starts_with("ukr") || lower.starts_with("uga") || lower.starts_with("one") || lower.starts_with('1') { return "a" }
  if lower.starts_with("hour") || lower.starts_with("honor") || lower.starts_with("honest") || lower.starts_with('8') || lower.starts_with("11") { return "an" }
  if let Some(first_char) = lower.chars().next() {
    match first_char {
      'a' | 'e' | 'i' | 'o' | 'u' => "an",
      _ => "a", 
    }
  } else {
    "a"
  }
}

