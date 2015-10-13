
pub fn hamming_distance(s1: &str, s2: &str) -> Result<i32, &'static str> {
  if s1.len() != s2.len() {
    return Err("inputs of different length");
  }

  let mut distance = 0;
  for i in 0..s1.len() {
    if s1.chars().nth(i) != s2.chars().nth(i) {
      distance += 1;
    }
  }

  Ok(distance)
}
