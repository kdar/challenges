#![feature(core)]
extern crate core;

use std::collections::HashMap;
use core::iter::FromIterator;

pub struct Info {
  data: HashMap<&'static str, &'static str>,
}

impl Info {
  pub fn name_for(&self, s: &str) -> Result<&str, &str> {
    let code: String = s.chars().map(|c| match c {
      'W' | 'M' | 'R' | 'D' | 'H' | 'V' | 'N' => 'A',
      'S' | 'Y' => 'C',
      'K' => 'G',
      _ => c
    }).collect();

    match self.data.get(&*code) {
      Some(name) => Ok(name),
      None => Err("error")
    }
  }
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Info {
  Info {data: HashMap::from_iter(pairs)}
}
