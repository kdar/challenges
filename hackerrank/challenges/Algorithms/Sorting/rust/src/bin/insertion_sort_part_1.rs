use std::io;

#[allow(dead_code)]
fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input
}

#[allow(dead_code)]
fn read_array() -> Vec<i64> {
  let line = read_line();
  line.trim().split(' ').map(|s| { s.parse().unwrap() }).collect()
}

#[allow(dead_code)]
fn read_num() -> i64 {
  let line = read_line();
  line.trim().parse().unwrap()
}

fn print_array(arr: &Vec<i64>) {
  println!("{}", arr.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().connect(" "));
}

fn main() {
  read_line();
  let mut arr = read_array();
  let unsorted = arr[arr.len()-1];
  let mut loc = 0;
  for i in (0..arr.len()-1).rev() {
    if arr[i] > unsorted {
      arr[i+1] = arr[i];
      print_array(&arr);
    } else {
      loc = i+1;
      break;
    }
  }

  arr[loc] = unsorted;
  print_array(&arr);
}
