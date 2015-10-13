// fn collatz_sequence(mut n: usize) -> Vec<usize> {
//   let mut answer: Vec<usize> = Vec::new();
//
//   while n != 1 {
//     answer.push(n);
//     if n % 2 == 0 {
//       n = n/2;
//     } else {
//       n = 3*n + 1;
//     }
//   }
//   answer.push(1);
//
//   answer
// }

fn collatz_sequence_count(mut n: usize) -> usize {
  let mut count = 0;

  while n != 1 {
    count+=1;
    if n % 2 == 0 {
      n = n/2;
    } else {
      n = 3*n + 1;
    }
  }
  count+=1;

  count
}

fn main() {
  //println!("{:?}", collatz_sequence(13));
  //println!("{:?}", collatz_sequence_count(13));
  let mut max = 0;
  let mut num = 0;
  for i in (1000000/2..1000000).rev() {
    let count = collatz_sequence_count(i);
    if count > max {
      max = count;
      num = i;
    }
  }
  println!("{}", num);
}
