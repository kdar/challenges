// p_10001 = 1+sum_(k=1)^(2^10001) floor(10001^(1/10001) (1/(1+pi(k)))^(1/10001))

fn simple_sieve(limit: usize) -> Vec<usize> {
  let mut is_prime = vec![true; limit+1];
  is_prime[0] = false;
  if limit >= 1 { is_prime[1] = false }

  for num in 2..limit+1 {
    if is_prime[num] {
      let mut multiple = num*num;
      while multiple <= limit {
        is_prime[multiple] = false;
        multiple += num;
      }
    }
  }

  is_prime.iter().enumerate()
    .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
    .collect()
}

fn main() {
  //let inner
  //let answer: u64 = 1 + (1..n+1).fold(0, |t,x| t + x);
  println!("{:?}", simple_sieve(10001*200).iter().nth(10000));
}
