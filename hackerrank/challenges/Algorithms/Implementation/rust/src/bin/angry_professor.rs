fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let nk: Vec<usize> = line.trim().split(' ').map(|x| x.trim().parse::<usize>().unwrap()).collect();
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let times: usize = line.trim().split(' ').fold(0, |t,x| {
      if x.trim().parse::<i32>().unwrap() <= 0 {
        return t+1;
      }
      t
    });

    if times < nk[1] {
      println!("YES");
    } else {
      println!("NO");
    }
  }
}
