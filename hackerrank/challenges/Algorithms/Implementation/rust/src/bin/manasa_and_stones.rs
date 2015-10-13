use std::io::{BufReader, BufRead};
use std::cmp;

#[allow(dead_code)]
static TEST: &'static str = "2
3
1
2
4
100
100";

fn get_set(n: usize, a: usize, b: usize) -> Vec<usize> {
  let atmp = cmp::min(a, b);
  let btmp = cmp::max(a, b);
  let (a, b) = (atmp, btmp);

  let last_min = (n-1) * a;

  let mut result = vec![];

  if a == b {
    return vec![last_min];
  }

  for i in 0..n {
    result.push(last_min + ((b-a) * i));
  }

  result
}

// #[allow(dead_code)]
// fn generate(n: usize, a: usize, b: usize) {
//   generate_(n, 0, cmp::min(a, b), cmp::max(a, b), vec![]);
// }
//
// #[allow(dead_code)]
// fn generate_(n: usize, i: usize, a: usize, b: usize, mut v: Vec<usize>) {
//   if i == 0 {
//     v.push(0);
//     generate_(n, i+1, a, b, v);
//   } else if i == n {
//     println!("{:?}", v);
//   } else {
//     let mut v1 = v.clone();
//     let mut v2 = v.clone();
//     v1.push(v[v.len()-1]+a);
//     v2.push(v[v.len()-1]+b);
//     generate_(n, i+1, a, b, v1);
//     generate_(n, i+1, a, b, v2);
//   }
// }

#[allow(dead_code)]
fn generate2(n: usize, a: usize, b: usize) -> Vec<Vec<usize>> {
  generate2_(n, 0, cmp::min(a, b), cmp::max(a, b), vec![])
}

#[allow(dead_code)]
fn generate2_(n: usize, i: usize, a: usize, b: usize, mut v: Vec<usize>) -> Vec<Vec<usize>> {
  let mut answer: Vec<Vec<usize>> = Vec::new();
  if i == 0 {
    v.push(0);
    answer.extend(generate2_(n, i+1, a, b, v));
  } else if i == n {
    answer.push(v);
  } else {
    let mut v1 = v.clone();
    let mut v2 = v.clone();
    v1.push(v[v.len()-1]+a);
    v2.push(v[v.len()-1]+b);
    answer.extend(generate2_(n, i+1, a, b, v1));
    answer.extend(generate2_(n, i+1, a, b, v2));
  }
  answer
}

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let stones: usize = line.trim().parse().unwrap();
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let a: usize  = line.trim().parse().unwrap();
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let b: usize  = line.trim().parse().unwrap();

    let vec = get_set(stones, a, b);
    println!("{}", vec.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().connect(" "));
  }
}

fn main() {
  //println!("{:?}", generate(4, 10, 100));

  // let mut br = BufReader::new(TEST.as_bytes());
  // solve(&mut br);
  let mut stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
