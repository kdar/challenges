#[derive(PartialEq)]
struct date {
  year: usize,
  day: usize,
  month: usize,
}

fn compute(actual: date, expected: date) -> usize {
  if actual.year > expected.year { 10000 }
  else if actual.year < expected.year { 0 }
  else if actual.month > expected.month { (actual.month - expected.month) * 500 }
  else if actual.month < expected.month { 0 }
  else if actual.day > expected.day { (actual.day - expected.day) * 15 }
  else { 0 }
}

fn main() {
  let mut date1: String = String::new();
  let mut date2: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut date1).ok().unwrap();
  stdin.read_line(&mut date2).ok().unwrap();

  let date1: Vec<usize> = date1.split(' ').map(|x| x.trim().parse().unwrap()).collect();
  let date2: Vec<usize> = date2.split(' ').map(|x| x.trim().parse().unwrap()).collect();

  println!("{}", compute(date{
    day: date1[0],
    month: date1[1],
    year: date1[2],
  }, date{
    day: date2[0],
    month: date2[1],
    year: date2[2],
  }));
}
