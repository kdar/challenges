use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "5 4 1
01 02 03 04
14 15 16 05
13 20 17 06
12 19 18 07
11 10 09 08";
// static TEST: &'static str = "5 6 1
// 01 02 03 04 05 06
// 18 19 20 21 22 07
// 17 28 29 30 23 08
// 16 27 26 25 24 09
// 15 14 13 12 11 10";
// static TEST: &'static str = "6 8 1
// 01 02 03 04 05 06 07 08
// 24 25 26 27 28 29 30 09
// 23 40 41 42 43 44 31 10
// 22 39 48 47 46 45 32 11
// 21 38 37 36 35 34 33 12
// 20 19 18 17 16 15 14 13";

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let mnr: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
  let (m, n, r) = (mnr[0], mnr[1], mnr[2]);

  let mut matrix: Vec<Vec<usize>> = vec![];
  for _ in 0..m {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let row: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    matrix.push(row);
  }

  let mut new_matrix = matrix.clone();

  // Go through each each and flatten it into a single layer.
  let (mut top_x, mut top_y) = (0, 0);
  let mut flattened: Vec<Vec<usize>> = vec![];
  loop {
    let mut flat = vec![];
    // flatten top row
    for x in top_x..n-top_x {
      flat.push(matrix[top_y][x]);
    }
    // flatten right column
    for y in top_y+1..m-top_y {
      flat.push(matrix[y][n-top_x-1]);
    }
    // flatten bottom row
    for x in (top_x..n-top_x-1).rev() {
      flat.push(matrix[m-top_y-1][x]);
    }
    // flatten left column
    for y in (top_y+1..m-top_y-1).rev() {
      flat.push(matrix[y][top_x]);
    }
    flattened.push(flat);
    top_x += 1;
    top_y += 1;

    if top_x >= n / 2 || top_y >= m / 2 {
      break;
    }
  }

  // Unflatten the layer into a grid again.
  top_x = 0;
  top_y = 0;
  loop {
    let mut index = 0;
    // rotate the flattened layer
    let tmp = flattened[top_y].clone();
    let (v1, v2) = tmp.split_at(r%flattened[top_y].len());
    flattened[top_y] = v2.to_vec();
    flattened[top_y].extend(v1.to_vec());

    // unflatten top row
    for x in top_x..n-top_x {
      new_matrix[top_y][x] = flattened[top_y][index];
      index += 1;
    }
    // unflatten right column
    for y in top_y+1..m-top_y {
      new_matrix[y][n-top_x-1] = flattened[top_y][index];
      index += 1;
    }
    // unflatten bottom row
    for x in (top_x..n-top_x-1).rev() {
      new_matrix[m-top_y-1][x] = flattened[top_y][index];
      index += 1;
    }
    // unflatten left column
    for y in (top_y+1..m-top_y-1).rev() {
      new_matrix[y][top_x] = flattened[top_y][index];
      index += 1;
    }

    top_x += 1;
    top_y += 1;

    if top_x >= n / 2 || top_y >= m / 2 {
      break;
    }
  }

  for (i, _) in matrix.iter().enumerate() {
    // println!("{} \t {}",
    //   new_matrix[i].iter().map(|x| format!("{:02}", x)).collect::<Vec<String>>().join(" "),
    //   matrix[i].iter().map(|x| format!("{:02}", x)).collect::<Vec<String>>().join(" "));
    println!("{}",
      new_matrix[i].iter().map(|x| format!("{}", x)).collect::<Vec<String>>().connect(" "));
  }
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      solve(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
