
pub fn reply(input: &str) -> &str {
  let upper: Vec<char> = input.chars().filter(|x| x.is_uppercase()).collect();
  let lower: Vec<char> = input.chars().filter(|x| x.is_lowercase()).collect();

  if input.is_empty() {
    return "Fine. Be that way!";
  } else if upper.len() > lower.len() {
    return "Whoa, chill out!";
  }

  if let Some(ch) = input.chars().last() {
    if ch == '?' {
      return "Sure.";
    }
  }

  "Whatever."
}
