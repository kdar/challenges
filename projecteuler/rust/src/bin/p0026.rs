// fn decimals(mut dividend: usize, divisor: usize) -> String {
//   let mut answer = String::new();
//   // in the equation 1/divisor, the maximum recurring length is
//   // divisor-1.
//   for _ in 0..divisor {
//     dividend = (dividend % divisor) * 10;
//     answer.push_str(&format!("{}", dividend / divisor));
//   }
//   answer
// }

fn method1() {
  let mut answer = 0;
  let mut length = 0;
  for i in (0..1000).rev() {
    if length >= i {
      break;
    }

    let mut value = 1;
    let mut position = 0;
    let mut remainders: Vec<usize> = vec![0; i];

    // so basically we keep finding the remainder of the
    // value divided by the divisor, and we set in our list
    // of remainders the position we are at when we get that
    // answer. We then stop if the value goes to 0, or we find
    // a remainder in the 0 position (both means the number is
    // done dividing). This then gives us the length of the repeated
    // value.
    while remainders[value] == 0 && value != 0 {
      //println!("remainders[{}] = {}", value, position);
      remainders[value] = position;
      value *= 10;
      value %= i;
      position += 1;

      //println!("{} {}", remainders[value] == 0, value != 0);
    }
    //println!("");

    if position - remainders[value] > length {
      answer = i;
      length = position - remainders[value];
    }
  }

  println!("{}", answer);
}

fn method2() {
  let mut max = 0;
  let mut found = 0;
  for i in (1..1000).rev() {
    // if i is not divisible by 2 nor 5, then it is purely recurring.
    if !is_repeating(i) {
      continue;
    }

    let mut dividend = 1;
    let mut count = 0;

    // We start at dividend = 1 and do long division on it while counting
    // our steps. If the dividend equals to 1, then that means we're starting
    // the sequence over again which means we should stop. If dividend equals
    // 0 then we are done as well.
    loop {
      dividend *= 10;
      dividend = dividend - (dividend / i)*i;
      count += 1;

      if dividend == 1 || dividend == 0 {
        break;
      }
    }

    if count > max {
      found = i;
      max = count;
    }
  }

  println!("{}", found);
}

fn is_repeating(n: usize) -> bool {
  if n == 2 || n == 5 {
    return false;
  }

  let mut i = 2;
  while i*i <= n {
    if n % i == 0 {
      if i == 2 || i == 5 {
        return false;
      }
      if i*i != n {
        if n/i == 2 || n/i == 5 {
          return false;
        }
      }
    }
    i+=1;
  }
  true
}

fn main() {
  method2();
}
