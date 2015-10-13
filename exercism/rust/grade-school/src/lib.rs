use std::collections::HashMap;

pub struct School {
  map: HashMap<u32, Vec<String>>,
}

impl School {
  pub fn new() -> School {
    School{
      map: HashMap::new(),
    }
  }

  pub fn add(&mut self, grade: u32, name: &str) {
    let entry = self.map.entry(grade).or_insert(Vec::<String>::new());
    entry.push(name.to_owned());
    entry.sort();
  }

  pub fn grades(&self) -> Vec<u32> {
    let v: Vec<u32> = self.map.keys().map(|s| *s).collect();
    v
  }

  pub fn grade(&self, n: u32) -> Option<&Vec<String>> {
    self.map.get(&n)
  }
}
