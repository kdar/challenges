fn main() {
  let mut line: String = String::new();
  let mut stdin = std::io::stdin();
  stdin.read_line(&mut line).ok().unwrap();
  let count = line.trim().parse().unwrap();

  for _ in 0..count {
    line.clear();
    stdin.read_line(&mut line).ok().unwrap();
    let data: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    let (money, cost, discount) = (data[0], data[1], data[2]);
    let bought = money / cost;
    let mut chocolates: Vec<usize> = vec![bought];
    let mut wrappers_left = 0;
    loop {
      let new = (chocolates[chocolates.len()-1] + wrappers_left) / discount;
      wrappers_left = (chocolates[chocolates.len()-1] + wrappers_left) % discount;
      chocolates.push(new);

      if chocolates[chocolates.len()-1] + wrappers_left < discount {
        break;
      }
    }

    println!("{}", chocolates.iter().fold(0, |t,x| t+x));
  }
}
