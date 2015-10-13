#![feature(test)]

extern crate test;

fn is_palindrome(n: u64) -> bool {
  if n < 10 {
    return true;
  }

  let mut l = ((n as f64).log10()+1.0).floor() as u32;
  let mut n2 = 0u64;
  let mut nn = n;
  loop {
    if nn == 0 {
      break;
    }

    l -= 1;
    n2 += (nn % 10) * 10u64.pow(l);
    nn /= 10;
  }

  n == n2
}

#[allow(dead_code)]
fn is_palindrome_str(n: u64) -> bool {
  let s1 = format!("{}", n);
  let s2: String = format!("{}", n).chars().rev().collect();
  s1 == s2
}

fn solve() -> u64 {
  let mut largest = 0u64;
  for x in (100..1000).rev() {
    for y in (100..1000).rev() {
      if is_palindrome(x*y) && x*y > largest {
        largest = x*y;
      }
    }
  }

  largest
}

fn main() {
  println!("{}", solve());
}

#[cfg(test)]
mod tests {
  use test::Bencher;

  #[test]
  fn test_is_palindrome() {
    assert_eq!(true, super::is_palindrome(6));
    assert_eq!(true, super::is_palindrome(9009));
    assert_eq!(false, super::is_palindrome(10));
    assert_eq!(false, super::is_palindrome(9109));
    assert_eq!(false, super::is_palindrome(9019));
    assert_eq!(false, super::is_palindrome(9008));
    assert_eq!(false, super::is_palindrome(8009));
  }

 #[bench]
 fn bench_is_palindrome(b: &mut Bencher) {
   b.iter(|| super::is_palindrome(909909));
 }

 #[bench]
 fn bench_is_palindrome_str(b: &mut Bencher) {
   b.iter(|| super::is_palindrome_str(909909));
 }
}
