fn gcd(mut a: u64, mut b: u64) -> u64 {
  if a < b {
    let t = b;
    b = a;
    a = t;
  }

  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }

  a
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn solve() -> u64 {
  let mut n = 2u64;
  for i in 1..21 {
    n = lcm(n, i);
  }
  n
}

fn main() {
  println!("{}", solve());
}
