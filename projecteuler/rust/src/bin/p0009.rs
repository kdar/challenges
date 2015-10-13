#![feature(test)]

extern crate test;

// a = m^2-n^2
// b = 2mn
// c = m^2 + n^2
// a+b+c = 1000
//
// m^2-n^2 + 2mn + m^2+n^2 = 1000
// 2mn + m^2 = 1000
// 2m(mn + m) = 1000
// m(m+n) = 500
// m > n
//
// n = (500/m) - m
fn method1(r: i64) -> i64 {
  for m in 1i64..r {
    if 500 % m != 0 {
      continue;
    }

    let n: i64 = 500/m - m;

    if n < 0 || n >= m {
      continue;
    }

    let a = m.pow(2)-n.pow(2);
    let b = 2 * m * n;
    let c = m.pow(2) + n.pow(2);

    if a+b+c == 1000 && a*a + b*b == c*c {
      return a*b*c;
    }
  }
  -1
}

fn method2(r: i64) -> i64 {
  for a in 1..r {
    for b in a+1..r {
      let c = r - (a+b);
      if c > b {
        if a*a + b*b == c*c {
          return a*b*c;
        }
      } else {
        break;
      }
    }
  }
  -1
}

fn main() {
  println!("{}", method1(1000));
}

#[cfg(test)]
mod tests {
  use test::Bencher;

 #[bench]
 fn bench_method1(b: &mut Bencher) {
   b.iter(|| super::method1(1000));
 }

 #[bench]
 fn bench_method2(b: &mut Bencher) {
   b.iter(|| super::method2(1000));
 }
}
