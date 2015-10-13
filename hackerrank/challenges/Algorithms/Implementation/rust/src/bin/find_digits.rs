
fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let n: u64 = line.trim().parse().unwrap();

    let mut count = 0;
    let mut m = n;
    while m != 0 {
      let i = m % 10;
      m /= 10;

      if i != 0 && n % i == 0 {
        count += 1;
      }
    }

    println!("{}", count);
  }
}
