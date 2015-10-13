fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  let mut matrix: Vec<Vec<i32>> = Vec::new();
  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let v: Vec<&str> = line.trim().split(' ').collect();
    let vec: Vec<i32> = v.iter().map(|x| x.trim().parse().unwrap()).collect();
    matrix.push(vec);
  }

  let mut diag1 = 0i32;
  let mut diag2 = 0i32;
  for i in 0..count {
    diag1 += matrix[i][i];
    diag2 += matrix[i][count-i-1];
  }

  println!("{}", (diag1 as f64 - diag2 as f64).abs() as usize);
}
