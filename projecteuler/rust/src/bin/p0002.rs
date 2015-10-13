fn solve() -> u64 {
  let mut sum = 0;
  let (mut a, mut b) = (1u64, 1u64);
  loop {
    let z = b;
    b = a+b;
    a = z;

    if a > 4000000 {
      break;
    }

    if a % 2 == 0 {
      sum += a;
    }
  }

  sum
}

fn main() {
  println!("{}", solve());
}
