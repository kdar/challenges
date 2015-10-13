use std::io::{BufReader, BufRead};

#[allow(dead_code)]
static TEST: &'static str = "4 5
10101
11100
11010
00101";

fn solve(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let nm: Vec<usize> = line.split(' ').map(|x| x.trim().parse().unwrap()).collect();

  let mut data: Vec<Vec<bool>> = Vec::new();
  for _ in 0..nm[0] {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let topics: Vec<bool> = line.trim().chars().map(|x| x.to_digit(2).unwrap() == 1).collect();
    data.push(topics);
  }

  let mut max_topics = 0;
  let mut max_teams = 0;
  for (i, topicsi) in data.iter().enumerate() {
    for topicsj in data.iter().skip(i+1) {
      let topics = topicsi.iter().zip(topicsj.iter()).fold(0, |t,x| if x.0 | x.1 { t+1 } else { t });
      if topics > max_topics {
        max_topics = topics;
        max_teams = 1;
      } else if topics == max_topics {
        max_teams += 1;
      }
    }
  }

  println!("{}", max_topics);
  println!("{}", max_teams);
}

fn main() {
  // let mut br = BufReader::new(TEST.as_bytes());
  // solve(&mut br);
  let mut stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  solve(&mut br);
}
