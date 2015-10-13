use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input.clear();
  io::stdin().read_line(&mut input).unwrap();
  let mut positives = 0.0;
  let mut negatives = 0.0;
  let mut zeros = 0.0;

  for i in input.split(' ') {
   let x: i32 = i.trim().parse().unwrap();
   if x == 0 {
     zeros += 1.0;
   } else if x > 0 {
     positives += 1.0;
   } else {
     negatives += 1.0;
   }
  }

  let total = (positives + negatives + zeros) as f64;
  println!("{:.3}\n{:.3}\n{:.3}", positives/total, negatives/total, zeros/total);
}
