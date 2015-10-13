#![feature(iter_arith)]

fn factors(n: usize) -> Vec<usize> {
  let mut factors: Vec<usize> = Vec::new();
  factors.push(1);
  factors.push(n);
  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      factors.push(i);
      if i*i != n {
        factors.push(n/i);
      }
    }
    i+=1;
  }
  //factors.sort();
  factors
}

fn sum_of_divisors(mut n: usize) -> usize {
  let mut prod = 1;
  let mut k = 2;
  while k*k<=n {
    let mut p=1;
    while n%k==0 {
      p=p*k+1;
      n/=k;
    }
    prod*=p;
    k += 1;
  }

  if n>1 {
    prod*=1+n;
  }
  prod
}

fn method1() {
  let limit = 28123;
  let mut abundant: Vec<usize> = Vec::new();
  for i in 0..limit {
    let sum: usize = factors(i).iter().sum();
    if sum > i {
      abundant.push(i);
    }
  }

  let mut marked: Vec<bool> = vec![false; limit+1];
  for i in 0..abundant.len() {
    for j in i..abundant.len() {
      if abundant[i] + abundant[j] <= limit {
        marked[abundant[i] + abundant[j]] = true;
      } else {
        break;
      }
    }
  }

  let mut total = 0;
  for (i, m) in marked.iter().enumerate() {
    if !m {
      total += abundant[i];
    }
  }

  println!("{}", total);
}

fn method2() {
  let limit = 28123;
  let mut abundant: Vec<usize> = Vec::new();

  for i in 2..limit {
    if sum_of_divisors(i)>2*i {
      abundant.push(i);
    }
  }

  let mut sum_of_abundants: Vec<bool> = vec![false; limit*2+1];

  for i in 0..abundant.len() {
    for j in 0..i {
      sum_of_abundants[abundant[i]+abundant[j]]=true;
    }
  }

  let mut sum = 0;
  for i in 1..30000 {
    if !sum_of_abundants[i] {
      sum+=i;
    }
  }

  println!("{}", sum);
}

fn main() {
  method2();
}
