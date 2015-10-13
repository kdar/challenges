fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  let mut matrix: Vec<Vec<u8>> = Vec::new();
  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    matrix.push(line.trim().chars().map(|x| x as u8 - b'0').collect::<Vec<u8>>());
  }

  for y in 0..matrix.len() {
    for x in 0..matrix[y].len() {
      let z = matrix[y][x];
      if y > 0 && y < matrix.len()-1 && x > 0 && x < matrix[y].len()-1 &&
          z > matrix[y][x-1] && z > matrix[y][x+1] &&
          z > matrix[y-1][x] && z > matrix[y+1][x] {
        print!("X");
      } else {
        print!("{}", z);
      }
    }
    println!("");
  }
}
