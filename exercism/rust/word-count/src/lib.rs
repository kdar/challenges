use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
  let mut m: HashMap<String, u32> = HashMap::new();

  let parts = s.split(|c: char| !c.is_alphanumeric());
  for i in parts {
    let part = &i.to_lowercase();
    if i.is_empty() {
      continue;
    }

    let t = m.clone();
    match t.get(part) {
      Some(c) => {
        m.insert(part.to_owned(), c+1);
      },
      None => {
        m.insert(part.to_owned(), 1);
      },
    }
  }

  m
}
