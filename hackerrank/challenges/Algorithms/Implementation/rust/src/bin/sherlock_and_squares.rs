#[allow(dead_code)]
fn method1() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let range: Vec<u64> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let mut count = 0;
    let mut i = (range[0] as f64).sqrt() as u64;
    loop {
      if i*i >= range[0] && i*i <= range[1] {
        count += 1;
      } else if i*i > range[1] {
        break;
      }
      i += 1;
    }

    println!("{}", count);
  }
}

fn method2() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let range: Vec<u64> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let a = (range[0] as f64).sqrt().ceil();
    let b = (range[1] as f64).sqrt().floor();

    println!("{}", (b - a+1.0) as u64);
  }
}

fn main() {
  method2();
}
