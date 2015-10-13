fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let v: Vec<&str> = line.trim().split(' ').collect();
    let x = v[0].parse().unwrap();
    let y = v[1].parse().unwrap();
    println!("{}", add(x, y));
  }
}
