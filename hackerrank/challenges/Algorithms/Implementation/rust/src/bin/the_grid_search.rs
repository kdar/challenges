use std::io::{BufReader, BufRead};

static TEST: &'static str = "1
4 6
123412
561212
123612
781234
2 2
12
34";

fn find_all(haystack: &str, needle: &str) -> Option<Vec<usize>> {
  let mut p: &str = haystack;
  let mut base_index = 0;
  let mut result: Vec<usize> = Vec::new();
  while let Some(index) = p.find(&needle) {
    result.push(index+base_index);
    base_index += index+1;
    if p.len() - index >= needle.len() {
      p = &p[index+1..];
    } else {
      break;
    }
  }

  if !result.is_empty() {
    Some(result)
  } else {
    None
  }
}

fn one_in_common<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
  for i in a {
    for j in b {
      if i == j {
        return true;
      }
    }
  }
  false
}

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  'TESTCASE: for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let size1: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let mut matrix1: Vec<String> = Vec::new();
    for _ in 0..size1[0] {
      line.clear();
      input.read_line(&mut line).ok().unwrap();
      matrix1.push(line.trim().to_string());
    }

    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let size2: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let mut matrix2: Vec<String> = Vec::new();
    for _ in 0..size2[0] {
      line.clear();
      input.read_line(&mut line).ok().unwrap();
      matrix2.push(line.trim().to_string());
    }

    for (row1i, row1) in matrix1.iter().enumerate() {
      if matrix1.len()-row1i+1 < matrix2.len() {
        break;
      }

      if let Some(locs) = find_all(row1, &matrix2[0]) {
        let mut found = true;
        for i in 1..matrix2.len() {
          if let Some(locs2) = find_all(&matrix1[row1i+i], &matrix2[i]) {
            if !one_in_common(&locs, &locs2) {
              found = false;
              break;
            }
          } else {
            found = false;
            break;
          }
        }

        if found {
          println!("YES");
          continue 'TESTCASE;
        }
      }
    }

    println!("NO");
  }
}

fn main() {
  //let mut br = BufReader::new(TEST.as_bytes());
  //solve(&mut br);
  let mut stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
