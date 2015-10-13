use std::iter::repeat;

fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let n: i32 = line.trim().parse().unwrap();

    let mut pivot = n;
    while pivot > 0 {
      if pivot % 3 == 0 {
        break;
      } else {
        pivot -= 5;
      }
    }

    if pivot < 0 {
      println!("-1");
    } else {
      println!("{}{}", repeat("5").take(pivot as usize).collect::<String>(), repeat("3").take((n-pivot) as usize).collect::<String>());
    }
  }
}
