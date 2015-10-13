#![feature(iter_arith)]

fn word_length(n: usize) -> usize {
  match n {
      0 => 0,
      1 => "one".len(),
      2 => "two".len(),
      3 => "three".len(),
      4 => "four".len(),
      5 => "five".len(),
      6 => "six".len(),
      7 => "seven".len(),
      8 => "eight".len(),
      9 => "nine".len(),
      10 => "ten".len(),
      11 => "eleven".len(),
      12 => "twelve".len(),
      13 => "thirteen".len(),
      14 => "fourteen".len(),
      15 => "fifteen".len(),
      16 => "sixteen".len(),
      17 => "seventeen".len(),
      18 => "eighteen".len(),
      19 => "nineteen".len(),
      20 => "twenty".len(),
      30 => "thirty".len(),
      40 => "forty".len(),
      50 => "fifty".len(),
      60 => "sixty".len(),
      70 => "seventy".len(),
      80 => "eighty".len(),
      90 => "ninety".len(),
      n if n < 100 => {
        let u = n % 10;
        let t = n - u;
        word_length(t) + word_length(u)
      },
      n if n < 1000 => {
        word_length(n / 100) + "hundred".len() + if n % 100 == 0 {
          0
        } else {
          "and".len() + word_length(n % 100)
        }
      },
      n if n % 1000 == 0 => word_length(n / 1000) + "thousand".len(),
      n => panic!("{}", n),
  }
}

fn main() {
  //println!("{} {}", "onehundredandfourteen".len(), word_length(114));
  // println!("{} {}", "ninetyfive".len(), word_length(95));
  // println!("{} {}", "threehundredandfortytwo".len(), word_length(342));
  // println!("{} {}", "onehundredandfifteen".len(), word_length(115));
  // println!("{} {}", "fivehundredandtwentyone".len(), word_length(521));
  // println!("{} {}", "onethousand".len(), word_length(1000));
  // println!("{} {}", "onehundred".len(), word_length(100));
  // println!("{} {}", "onehundredandone".len(), word_length(101));
  // println!("{} {}", "onehundredandtwelve".len(), word_length(112));
  // println!("{} {}", "onethousandonehundredandeleven".len(), word_length(1111));
  // println!("{} {}", "onethousandtwohundredandthirtyfour".len(), word_length(1234));
  // println!("{} {}", "fivethousandsixhundredandseventyeight".len(), word_length(5678));
  // println!("{} {}", "ninethousandninehundredandninetynine".len(), word_length(9999));
  let answer: usize = (1..1001).map(|x| word_length(x)).sum();
  println!("{}", answer);
}
