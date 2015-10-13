extern crate ramp;
use ramp::Int;
use std::f64::consts::PI;

fn fib(n: Int) -> Int {
  let mut a = Int::from(0);
  let mut b = Int::from(1);
  let one = Int::from(1);

  for i in (0..32).rev() {
    let d = &a * (&b * 2 - &a);
    let e = &a*&a + &b*&b;
    a = d;
    b = e;
    if (n.clone() >> i) & &one != 0 {
      let c = a + &b;
      a = b;
      b = c;
    }
  }
  a
}

fn method1() {
  let mut n = Int::from(1);
  let mut index = 0u64;
  loop {
    n = &n + 1;
    index += 1;

    let len = format!("{}", fib(n.clone())).len();
    //println!("{}", len);
    if len == 1000 {
      println!("{}", index);
      break;
    }
  }
}

fn method2() {
  let (mut fa, mut fb, mut i) = (Int::from(1), Int::from(1), Int::from(1));
  while format!("{}", &fa).len() < 1000 {
    //fa, fb, i = fb, fa + fb, i + 1
    let tmp = fa.clone();
    fa = fb.clone();
    fb = tmp + &fa;
    i = &i + 1;
  }
  println!("{}", i);
}

fn method3() {
  // Binet's Fibonacci number formula 
  let v: f64 = (((10f64.log(PI) * 999f64 + 5f64.log(PI)/2f64) / 1.6180f64.log(PI)) as f64).round();
  println!("{}", v);
}

fn main() {
  method2();
  method3();
}
