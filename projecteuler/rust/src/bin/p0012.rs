#![feature(test)]

extern crate test;

// fn factors(n: usize) -> Vec<usize> {
//   let mut factors: Vec<usize> = Vec::new();
//   factors.push(1);
//   factors.push(n);
//   let mut i = 2;
//   while i*i <= n {
//     if n % i == 0 {
//       factors.push(i);
//       if i*i != n {
//         factors.push(n/i);
//       }
//     }
//     i+=1;
//   }
//   // factors.sort();
//   factors
// }

fn factor_count(n: usize) -> usize {
  let mut count = 1;
  if n == 1 {
    return count;
  }

  // add for n
  count += 1;

  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      count += 1;
      if i*i != n {
        count += 1;
      }
    }
    i+=1;
  }
  // factors.sort();
  count
}

fn triangular(nth: usize) -> usize {
  //(0..nth+1).fold(0, |t,x| t+x)
  nth*(nth+1) / 2
}

fn method1() -> usize {
  let mut i = 0;
  loop {
    let tri = triangular(i);
    if factor_count(tri) > 500 {
      return tri;
    }
    i += 1;
  }
  0
}

fn method2() -> usize {
  let mut n = 1;
  let mut f1 = 1;
  let mut f2 = 0;
  let mut x = 0;
  loop {
    n += 1;
    x = n;
    if n % 2 == 0 {
      x /= 2;
    }
    f2 = 0;
    let max = (x as f64).sqrt().round() as usize;
    for i in 1..max {
      if x % i == 0 {
        f2 += 2;
      }
    }
    if max *max == x {
      f2 -= 1;
    }
    let factors = f1*f2;
    f1 = f2;
    if factors > 500 {
      return n * (n - 1) / 2;
    }
  }
  0
}

fn main() {
  //println!("{:?}", factors(10));
  //println!("{:?}", factor_count(triangular(7)));
  println!("{}", method1());
  println!("{}", method2());
}

#[cfg(test)]
mod tests {
  use test::Bencher;

 #[bench]
 fn bench_method1(b: &mut Bencher) {
   b.iter(|| super::method1());
 }

 #[bench]
 fn bench_method2(b: &mut Bencher) {
   b.iter(|| super::method2());
 }
}
