#![feature(core)]

extern crate num;
extern crate core;
use num::{BigUint, FromPrimitive};
use core::ops::Rem;
use num::traits::ToPrimitive;
use core::ops::Div;
use num::pow;

fn main() {
  let n0 = BigUint::from_u64(0).unwrap();
  let n10 = BigUint::from_u64(10).unwrap();
  let mut num = pow(BigUint::from_u64(2).unwrap(), 1000);

  let mut sum = 0;
  loop {
    sum += num.clone().rem(&n10).to_usize().unwrap();
    let n = num.clone().div(&n10);
    num = n;
    if num == n0 {
      break;
    }
  }

  println!("{}", sum);
}
