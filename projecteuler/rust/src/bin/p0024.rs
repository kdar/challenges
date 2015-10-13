//use std::sync::{Arc, Mutex};

// fn permutations<F>(set: &Vec<String>, closure: &F)
//     where F: Fn(Vec<String>) {
// 	_perm(set, closure, set.len(), 0, vec![String::new(); set.len()]);
// }
//
// fn _perm<F>(set: &Vec<String>, closure: &F, length: usize, index: usize, mut perm: Vec<String>)
//     where F: Fn(Vec<String>) {
// 	if index == length {
//     closure(perm);
// 	} else {
// 		for i in 0..set.len() {
//       perm[index] = set[i].clone();
// 			_perm(set, closure, length, index+1, perm.clone());
// 		}
// 	}
// }

// fn permutations(set: &Vec<u8>) -> Vec<Vec<u8>> {
// 	_perm(set, set.len(), 0, vec![0; set.len()])
// }
//
// fn _perm(set: &Vec<u8>, length: usize, index: usize, mut perm: Vec<u8>) -> Vec<Vec<u8>> {
//   let mut result: Vec<Vec<u8>> = Vec::new();
//   if index == length {
//     result.push(perm);
// 	} else {
// 		for i in 0..set.len() {
//      perm[index] = set[i].clone();
// 			result.extend(_perm(set, length, index+1, perm.clone()));
// 		}
// 	}
//
//   result
// }

fn factoradic(mut n: usize, nits: usize) -> Vec<i32> {
  let mut factoradic: Vec<i32> = vec![0; nits];

  for j in 1..nits+1  {
    factoradic[nits - j] = (n % j) as i32;
    n /= j;
  }

  factoradic
}

fn permutation(n: usize, k: usize) -> Vec<i32> {
  let mut perm: Vec<i32> = vec![0; n];
  let fact = factoradic(k, n);

  let mut temp: Vec<i32> = vec![0; n];
  for i in 0..n {
    temp[i] = fact[i]+1;
  }

  perm[n-1] = 1;

  for i in (0..n-1).rev() {
    perm[i] = temp[i];
    for j in i+1..n {
      if perm[j] >= perm[i] {
        perm[j] += 1;
      }
    }
  }

  for i in 0..n {
    perm[i] -= 1;
  }

  perm
}


fn method1() {
  // https://msdn.microsoft.com/en-us/library/Aa302371
  // We find the answer by computing the factoradic of
  // the number, and then getting the permutation for that
  // factoradic.
  println!("{:?}", permutation(10, 1000000 - 1));
}

fn factorial(mut n: u64) -> u64 {
  let mut result = 1;
  while n > 0 {
    result *= n;
    n-=1;
  }
  result
}

fn method2() {
  let mut idx = 1000000 - 1;
  let mut set = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut result = vec![];
  while set.len() > 0 {
    let perm = factorial(set.len() as u64 - 1);
    let rest = idx % perm;
    let rm_idx = idx / perm;
    idx = rest;
    result.push(set.remove(rm_idx as usize));
  }

  // convert to u64
  let radix = 10;
  let mut order = 1u64;
  let mut answer = 0u64;
  for d in result.iter().rev() {
    answer = answer + order * d;
    order = order * radix;
  }

  println!("{}", answer);
}

// fast in-place permutation using SEPA algorithm: http://www.quickperm.org/soda_submit.php
fn permute(s: &mut Vec<char>) -> u64 {
  let mut len = s.len();
  let mut key = len-1;
  let mut newkey = len-1;
  while (key > 0) && (s[key] <= s[key-1]) {
    key-=1;
  }
  key-=1;
  if key < 0 {
    return 0;
  }
  newkey = len-1;
  while (newkey > key) && (s[newkey] <= s[key]) {
    newkey-=1;
  }

  let tmp = s[key];
  s[key] = s[newkey];
  s[newkey] = tmp;

  len-=1;
  key+=1;
  while len>key {
    let tmp = s[key];
    s[key] = s[len];
    s[len] = tmp;

    key+=1;
    len-=1;
  }

  return 1;
}

fn method3() {
  let mut st: Vec<char> = "0123456789".chars().collect();
  for i in 0..1000000-1 {
    permute(&mut st);
  }
  println!("{:?}", st);
}

fn main() {
  method1();
  method2();
  method3();
}
