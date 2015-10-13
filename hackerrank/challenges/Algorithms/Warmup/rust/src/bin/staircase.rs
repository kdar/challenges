fn main() {
  let mut line: String = String::new();
  std::io::stdin().read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();
  for i in 0..count {
    for _ in 0..count-i-1 {
      print!(" ");
    }
    for _ in count-i..count+1 {
      print!("#");
    }
    println!("");
  }
}
