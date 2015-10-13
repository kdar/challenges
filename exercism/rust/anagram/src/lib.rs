pub fn anagrams_for<'a>(s: &str, inputs: &[&'a str]) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();

  let mut ss: Vec<char> = s.to_lowercase().chars().collect();
  ss.sort();

  for i in inputs {
    if *i.to_lowercase() == s.to_lowercase() {
      continue;
    }

    let mut si: Vec<char> = i.to_lowercase().chars().collect();
    si.sort();

    if si == ss {
      result.push(i);
    }
  }

  result
}
