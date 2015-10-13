
pub fn square_of_sum(n: i32) -> i32 {
  let mut total = 0;
  for i in 1..n+1 {
    total += i;
  }

  total.pow(2)
}


pub fn sum_of_squares(n: i32) -> i32 {
  let mut total = 0;
  for i in 1..n+1 {
    total += i.pow(2);
  }

  total
}

pub fn difference(n: i32) -> i32 {
  square_of_sum(n) - sum_of_squares(n)
}
