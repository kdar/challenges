fn is_leap(year: usize) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_of_month(year: usize, month: usize) -> usize {
  match month {
    4 | 6 | 9 | 11 => 30,
    2 => {
      if is_leap(year) {
        29
      } else {
        28
      }
    },
    _ => 31,
  }
}

fn main() {
  let mut sundays = 0;
  let mut today = 2; // 2 = monday, 1 = sunday
  for year in 1900..2000+1 {
    for month in 1..12+1 {
      today += days_of_month(year, month);
      if year != 1900 && today%7 == 1 {
        sundays += 1;
      }
    }
  }

  println!("{}", sundays);
}
