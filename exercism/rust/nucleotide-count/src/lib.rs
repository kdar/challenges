use std::collections::HashMap;

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
  let mut m: HashMap<char, usize> = HashMap::new();
  m.insert('A', 0);
  m.insert('T', 0);
  m.insert('C', 0);
  m.insert('G', 0);

  for i in s.chars() {
    let m2 = m.clone();
    let count = m2.get(&i).unwrap();
    m.insert(i, count+1);
  }

  m
}

pub fn count(c: char, s: &str) -> i32 {
  let mut count = 0;
  for i in s.chars() {
    if i == c {
      count+=1;
    }
  }
  count
}
