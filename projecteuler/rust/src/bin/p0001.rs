#![feature(iter_arith)]

fn solve() -> u32 {
  (0..1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main () {
  //let answer = (0..1000).filter(|&x| x % 3 == 0 || x % 5 == 0).fold(0, |sum, i| sum + i);
  println!("{}", solve());
}
