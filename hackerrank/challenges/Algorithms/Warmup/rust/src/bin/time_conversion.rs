fn convert(line: String) {
  let mut v: Vec<String> = line.trim().split(':').map(|x| x.to_string()).collect();
  if v[2][2..] == "PM".to_string() {
    if v[0] != "12".to_string() {
      let mut x: usize = v[0].parse().unwrap();
      x += 12;
      v[0] = format!("{:02}", x);
    }
  } else {
    if v[0] == "12".to_string() {
      v[0] = "00".to_string();
    }
  }
  v[2] = v[2][0..2].to_string();
  println!("{}", v.connect(":"));
}

fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();

  convert(line);

  // for i in 1..13 {
  //   convert(format!("{:02}:00:00AM", i));
  // }
  // println!("");
  // for i in 1..13 {
  //   convert(format!("{:02}:00:00PM", i));
  // }
}
