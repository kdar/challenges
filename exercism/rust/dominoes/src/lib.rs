pub type Domino = (usize, usize);

pub fn chain(input: &Vec<Domino>) -> Option<Vec<Domino>> {
  let mut doms = input.clone();
  if input.is_empty() {
    return Some(vec![]);
  } else if input.len() == 1 {
    return Some(doms);
  }

  // Since we're dealing with a Eulerian circuit, one of the rules is
  // the sum of the degrees of all the vertices of a graph is an even number
  // (exactly twice the number of edges). Meaning, if we have an odd number of
  // a certain digit (e.g. 4), then we cannot complete the circuit.
  let mut check = 0;
  let mut matrix = [[0u8; 6]; 6];
  for (x, i) in input.iter().enumerate() {
    check ^= i.0 ^ i.1;

    // Compute the matrix up to, but not including, the last element.
    // We don't need the last element because we'll be popping it off.
    if x < input.len()-1 {
      if i.0 == i.1 {
        matrix[i.0-1][i.1-1] += 1;
      } else {
        matrix[i.0-1][i.1-1] += 1;
        matrix[i.1-1][i.0-1] += 1;
      }
    }
  }
  if check != 0 {
    return None;
  }

  let first = doms.pop().unwrap();
  let mut v: Vec<Domino> = Vec::new();
  v.push(first);

  let mut n = first.1;
  loop {
    let mut m = 0;
    // Find the next vertex whose row is in the same column
    // as n, save it as m, then remove it from the matrix.
    for y in 1..7 {
      if matrix[n-1][y-1] > 0 {
        if n == y {
          matrix[n-1][y-1] -= 1;
        } else {
          matrix[n-1][y-1] -= 1;
          matrix[y-1][n-1] -= 1;
        }
        m = y;
        break;
      }
    }

    if m == 0 {
      break;
    }

    v.push((n,m));
    n = m;
  }

  Some(v)
}
