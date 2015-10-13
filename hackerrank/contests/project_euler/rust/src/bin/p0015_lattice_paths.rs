use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "2
2 3
500 500";

fn binomial(n: usize, mut k: usize) -> BigUint {
  // Since binomial(n, k) = binomial(n, n - k), we might as well use
  // the smaller k to optimize
  if n - k < k {
    k = n - k;
  }

  // Compute the coefficient
  let mut res: BigUint = BigUint::new(1);
  for i in (1..k + 1) {
    //let m: BigUint = (n - k + i).to_biguint().unwrap();
    res = res.mul(n - k + i);
    //let d: BigUint = (i).to_biguint().unwrap();
    res = res.div(i);
  }

  res
}

// fn count_routes(m: usize, n: usize) -> Int {
//   if m == n {
//     let mut result = Int::from(1);
//     for i in 1..n+1 {
//       result = result * (n + i)/i;
//     }
//     return result;
//   }
//
//   let mut grid = vec![vec![Int::from(0); n+1]; m+1];
//   for i in 0..m+1 {
//     grid[i][0] = Int::from(1);
//   }
//   for j in 0..n+1 {
//     grid[0][j] = Int::from(1);
//   }
//   for i in 1..m+1 {
//     for j in 1..n+1 {
//       grid[i][j] = &grid[i - 1][j] + &grid[i][j - 1];
//     }
//   }
//   grid[m][n].clone()
// }

fn solve(corner: (usize, usize)) -> BigUint {
  binomial(corner.0 + corner.1, corner.1)
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let corner: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    println!("{}", solve((corner[0], corner[1])).rem(1000000007));
  }
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      setup(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  setup(&mut br);
}


// ----big number impl----

use std::fmt;

use std::clone::Clone;
use std::ops;

const MAXLEN: usize = 10000;
const BASE: usize = 10000;
const BASELEN: usize = 4;

struct ArrMax<T>([T; MAXLEN]);

#[allow(dead_code)]
impl<T: Copy> ArrMax<T> {
  fn new(initial: T) -> ArrMax<T> {
    ArrMax([initial; MAXLEN])
  }
}

impl<T: Copy> Clone for ArrMax<T> {
  #[inline]
  fn clone(&self) -> ArrMax<T> {
    ArrMax(self.0)
  }
}

impl<T> ops::Index<usize> for ArrMax<T> {
  type Output = T;
  #[inline]
  fn index(&self, index: usize) -> &T {
    &self.0[index]
  }
}

impl<T> ops::IndexMut<usize> for ArrMax<T> {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut T {
    &mut self.0[index]
  }
}

impl<T: fmt::Debug> fmt::Debug for ArrMax<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(&&self.0[..], f)
  }
}

pub struct BigUint {
  num: ArrMax<usize>,
  len: usize,
}

impl Clone for BigUint {
  fn clone(&self) -> BigUint {
    BigUint {
      num: self.num.clone(),
      len: self.len,
    }
  }
}

#[allow(dead_code)]
impl BigUint {
  pub fn new(n: usize) -> BigUint {
    let mut array = ArrMax::new(0);
    array[0] = n;
    BigUint{
      num: array,
      len: 1,
    }
  }

  pub fn debug_print(&self) {
    println!("{:?} {}", &self.num.0[0..10], self.len);
  }

  pub fn from_str(val: &str) -> BigUint {
    let mut buint = BigUint::new(0);

    let mut vstr = val.clone();
    while vstr.len() > 0 {
      let mut temp = 0;
      if vstr.len() > BASELEN {
        temp = usize::from_str_radix(&vstr[vstr.len()-4..], 10).unwrap();
        vstr = &vstr[0..vstr.len()-4];
      } else {
        temp = usize::from_str_radix(&vstr, 10).unwrap();
        vstr = "";
      }

      buint.num[buint.len-1] = temp;
      buint.len += 1;
    }

    buint.len -= 1;

    buint
  }

  fn set(&mut self, index: usize, val: usize) {
    self.num[index] = val;
  }

  pub fn mul(&self, b: usize) -> BigUint {
    let mut c = BigUint::new(0);
  	let mut carry = 0;
    let mut nc = 0;
  	for i in 0..self.len {
  		let val = self.num[i] * b;
  		c.num[nc] = (carry + val) % BASE;
  		carry = (carry + val) / BASE;
      nc += 1;
  	}
  	while carry != 0 {
  		c.num[nc] = carry % BASE;
      nc += 1;
      carry /= BASE;
    }
  	c.len = nc;
  	return c;
  }

  pub fn add(&self, mut b: usize) -> BigUint {
    let mut c = BigUint::new(0);
  	let mut carry = 0;
    let mut nc = 0;
    c.len = self.len;
  	for i in 0..self.len {
      // if b == 0 {
      //   c.num[nc] = self.num[i] + carry;
      //   carry = 0;
      //   nc += 1;
      //   continue;
      // }
  		let val = (carry + self.num[i]) + b % BASE;
      b = b / BASE;
      c.num[nc] = val % BASE;
      carry = (val / BASE) % BASE;
      nc += 1;
  	}

    if b != 0 {
      c.num[nc] = b;
      nc += 1;
    }

    if carry != 0 {
  		c.num[nc] += carry;
      nc += 1;
    }
  	c.len = nc;

  	return c;
  }

  pub fn sub(&self, mut b: usize) -> BigUint {
    let mut c = BigUint::new(0);
    let mut minuend = self.clone();
    let mut nc = 0;
    c.len = self.len;
  	for i in 0..self.len {
  		let mut val = 0;
      if minuend.num[i] >= b % BASE {
        val = minuend.num[i] - b % BASE;
      } else if i < self.len-1 {
        minuend.num[i+1] -= 1;
        val = minuend.num[i] + BASE - b % BASE;
      }

      b = b / BASE;
      c.num[nc] = val % BASE;
      nc += 1;
  	}

    while c.len > 1 && c.num[c.len-1] == 0 {
      c.len -= 1;
    }

  	return c;
  }

  pub fn div(&self, b: usize) -> BigUint {
    let mut c = BigUint::new(0);
  	let mut carry = 0;
    let mut nc = self.len-1;
    c.len = self.len;
  	for i in (0..self.len).rev() {
      let temp = carry*BASE + self.num[i];
  		c.num[nc] = temp / b;
  		carry = temp % b;
      if i == c.len-1 && c.num[nc] == 0 {
        c.len -= 1;
      }

      if nc > 0 {
        nc -= 1;
      }
  	}
  	return c;
  }

  pub fn rem(&self, b: usize) -> BigUint {
    let mut c = BigUint::new(0);
  	let mut carry = 0;
    let mut nc = self.len-1;
    c.len = self.len;
  	for i in (0..self.len).rev() {
      let temp = carry*BASE + self.num[i];
  		c.num[nc] = temp / b;
  		carry = temp % b;
      if i == c.len-1 && c.num[nc] == 0 {
        c.len -= 1;
      }

      if nc > 0 {
        nc -= 1;
      }
  	}
  	BigUint::new(carry)
  }
}

impl fmt::Display for BigUint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "{}", self.num[self.len - 1]));
    for i in (0..(self.len-1)).rev() {
      try!(write!(f, "{:04}", self.num[i]));
    }
    Ok(())
  }
}
