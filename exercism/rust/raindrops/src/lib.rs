
pub fn raindrops(n: u32) -> String {
  let answer = vec![(3, "Pling"),(5, "Plang"),(7, "Plong")].iter().map(|&(x,s) | {
    if n % x == 0 { s } else {""}
  }).collect::<String>();

  if answer.is_empty() {
    return n.to_string();
  }

  answer
}
