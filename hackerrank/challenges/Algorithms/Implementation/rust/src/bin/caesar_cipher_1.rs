fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();

  let mut phrase = String::new();
  stdin.read_line(&mut phrase).ok().unwrap();

  line.clear();
  stdin.read_line(&mut line).ok().unwrap();
  let k: u8 = line.trim().parse().unwrap();

  println!("{}", phrase.chars().map(|x| {
    if x.is_alphabetic() {
      let mut a = b'a';
      if x.is_uppercase() {
        a = b'A';
      }
      return (a + (x as u8 - a + k)%26) as char;
    }
    x
  }).collect::<String>());
}
