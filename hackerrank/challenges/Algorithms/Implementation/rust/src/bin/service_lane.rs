fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let nt: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

  line.clear();
  stdin.read_line(&mut line).ok().unwrap();
  let widths: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

  for _ in 0..nt[1] {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let range: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let v = widths.iter().skip(range[0]).take(range[1]-range[0]+1).map(|x| *x).min();
    println!("{}", v.unwrap());
  }
}
