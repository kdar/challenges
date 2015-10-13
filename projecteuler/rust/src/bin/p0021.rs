#![feature(iter_arith)]

fn factors_not_including_self(n: usize) -> Vec<usize> {
  let mut factors: Vec<usize> = Vec::new();
  factors.push(1);
  //factors.push(n);
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

fn main() {
  let mut total = 0;
  for i in 2..10001 {
    let asum: usize = factors_not_including_self(i).iter().sum();
    let bsum: usize = factors_not_including_self(asum).iter().sum();

    if i == bsum && i != asum {
      //println!("{} {} {}", i, asum, bsum);
      total += i;
    }
  }

  //println!("{:?}", factors(220));
  println!("{}", total);
}
