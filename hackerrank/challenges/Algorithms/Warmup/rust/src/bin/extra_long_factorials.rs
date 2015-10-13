// extern crate num;
// use num::{BigUint, FromPrimitive};
//
// fn factorial(mut n: BigUint) -> BigUint {
//   let zero: BigUint = FromPrimitive::from_u64(0).unwrap();
//   let one: BigUint = FromPrimitive::from_u64(1).unwrap();
//   let mut answer: BigUint = FromPrimitive::from_u64(1).unwrap();
//   while n != zero {
//     answer = answer * n.clone();
//     n = n.clone() - &one;
//   }
//   answer
// }
//
// fn main() {
//   let mut line: String = String::new();
//   let mut stdin = std::io::stdin();
//   stdin.read_line(&mut line).ok().unwrap();
//   let n = line.trim().parse().unwrap();
//   println!("{}", factorial(FromPrimitive::from_u64(n).unwrap()));
// }

use std::fmt;

const MAXLEN: usize = 10000;
const BASE: usize = 10000;

struct BigInt {
  num: [usize; MAXLEN],
  len: usize,
}

impl BigInt {
  fn new(n: usize) -> BigInt {
    let mut array = [0; MAXLEN];
    array[0] = n;
    BigInt{
      num: array,
      len: 1,
    }
  }

  fn mul(&self, b: usize) -> BigInt {
    let mut c = BigInt::new(0);
  	let mut carry = 0;
    let mut nc = 0;
  	for i in 0..self.len {
  		let val = self.num[i] * b;
  		c.num[nc] = (carry + val) % BASE;
  		carry = (carry + val) / BASE;
      nc += 1;
  	}
  	if carry != 0 {
  		c.num[nc] = carry;
      nc += 1;
    }
  	c.len = nc;
  	return c;
  }
}

impl fmt::Display for BigInt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "{}", self.num[self.len - 1]));
    for i in (0..(self.len-1)).rev() {
      try!(write!(f, "{:04}", self.num[i]));
    }
    Ok(())
  }
}

fn factorial(n: usize) -> BigInt {
  let mut answer: BigInt = BigInt::new(1);
  for i in 2..n+1 {
    answer = answer.mul(i);
  }
  answer
}

fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let n = line.trim().parse().unwrap();
  println!("{}", factorial(n));
}
