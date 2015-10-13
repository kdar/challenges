fn solve(mut n: u64) -> u64 {
  let mut i = 2;
  while i * i <= n {
    if n % i != 0 {
      i += 1;
    } else {
      n = (n as f64 / i as f64).floor() as u64;
    }
  }
  n
}

fn main() {
  println!("{}", solve(600851475143));
}
