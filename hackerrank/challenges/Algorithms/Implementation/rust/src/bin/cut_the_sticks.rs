fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  line.clear();
  stdin.read_line(&mut line).ok().unwrap();
  let mut sticks: Vec<usize> = line.split(' ').map(|x| x.trim().parse::<usize>().unwrap()).collect();

  while sticks.len() > 0 {
    println!("{}", sticks.len());
    let cloned = sticks.clone();
    if let Some(min) = cloned.iter().min() {
      let sticks_new = cloned.iter().filter_map(|x| {
        let new = x - min;
        if new > 0 {
          return Some(new);
        }
        None
      }).collect();
      sticks = sticks_new;
    }
  }
}
