use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
  let mut b: BTreeMap<String, i32> = BTreeMap::new();

  for (key, value) in input.iter() {
    for x in value {
      b.insert(x.to_lowercase().to_owned(), *key);
    }
  }

  b
}
