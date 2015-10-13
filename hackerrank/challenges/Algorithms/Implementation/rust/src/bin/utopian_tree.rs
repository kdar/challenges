
fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let mut n: usize = line.trim().parse().unwrap();
    let mut height = 1;
    for i in 0..n {
      if i % 2 == 0 {
        height += height;
      } else {
        height += 1;
      }
    }
    println!("{}", height);
  }
}
