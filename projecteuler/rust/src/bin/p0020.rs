#![feature(core)]

extern crate num;
extern crate core;

use num::{BigUint, FromPrimitive, ToPrimitive};
use core::ops::{Rem, Mul, Div};

fn factorial(n: u64) -> BigUint {
  let mut answer: BigUint = FromPrimitive::from_u64(1).unwrap();
  for i in 1..n+1 {
    let tmp: BigUint = FromPrimitive::from_u64(i).unwrap();
    answer = answer.mul(tmp);
  }

  answer
}

fn main() {
  let n0 = BigUint::from_u64(0).unwrap();
  let n10 = BigUint::from_u64(10).unwrap();
  let mut num = factorial(100);
  let mut sum = 0;
  while num.clone() != n0 {
    let tmp: BigUint = num.clone().rem(&n10);
    sum += tmp.to_u64().unwrap();
    num = num.div(&n10);
  }

  println!("{}", sum);
}
