extern crate rand;

use rand::Rng;

pub struct Robot{
  n: String,
}

impl Robot {
  pub fn new() -> Robot {
    let mut r = Robot{n: "".to_owned()};
    r.reset_name();
    r
  }

  pub fn name<'a>(&'a self) -> &'a str {
    &self.n
  }

  pub fn reset_name(&mut self) {
    let mut name: String = (0..2).map(|_| rand::thread_rng().gen_range(b'A', b'Z'+1) as char).collect();
    let nums: String = (0..3).map(|_| rand::thread_rng().gen_range(b'0', b'9'+1) as char).collect();
    name = name + &nums;
    self.n = name
  }
}
