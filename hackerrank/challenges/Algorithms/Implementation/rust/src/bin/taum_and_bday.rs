use std::io::{BufReader, BufRead};

#[allow(dead_code)]
static TEST: &'static str = "5
10 10
1 1 1
5 9
2 3 4
3 6
9 1 1
7 7
4 2 1
3 3
1 9 2";

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let bw: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    let (b, w) = (bw[0], bw[1]);
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let xyz: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    let (x, y, z) = (xyz[0], xyz[1], xyz[2]);

    println!("{}", vec![b*x + w*y, b*(y+z) + w*y, b*x + w*(x+z), b*(y+z) + w*(x+z)].iter().min().unwrap());
  }
}

fn main() {
  // let mut br = BufReader::new(TEST.as_bytes());
  // solve(&mut br);
  let mut stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
