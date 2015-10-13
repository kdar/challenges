use std::cmp;

fn binomial_coefficient(mut n: usize, k: usize) -> usize {
  if n == k || k == 0 {
    return 1;
  } else if 2*k < n {
    return binomial_coefficient(n,n-k);
  }

  let mut e = n-k+1;
  for i in 2..k+1 {
    e *= n-k+i;
    e /= i;
  }
  e
}

fn main() {
  // https://en.wikipedia.org/wiki/Lattice_path
  let bottom_right_corner = (20, 20);
  println!("{}", binomial_coefficient(bottom_right_corner.0 + bottom_right_corner.0, bottom_right_corner.1));
}
