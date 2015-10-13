use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input.clear();
  io::stdin().read_line(&mut input).unwrap();
  let mut sum = 0usize;
  for i in input.split(' ') {
    sum += i.trim().parse::<usize>().unwrap();
  }
  println!("{}", sum);
}
