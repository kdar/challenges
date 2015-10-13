#![feature(test, convert, vec_push_all)]

extern crate test;

use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

pub struct Forth {
  stack: Vec<i32>,
  redefines: HashMap<String,Vec<String>>,
}

impl Forth {
  pub fn new() -> Forth {
    Forth{stack: Vec::new(), redefines: HashMap::new()}
  }

  pub fn format_stack(&self) -> String {
    let mut result: String = "".to_owned();
    for (i, el) in self.stack.iter().enumerate() {
      result = result + &format!("{}", el);
      if i < self.stack.len()-1 {
        result = result + " ";
      }
    }
    result
  }

  pub fn eval(&mut self, input: &str) -> ForthResult {
    let input = input.to_lowercase();
    let input: Vec<&str> = input.split(|c: char| c.is_whitespace() || c.is_control()).collect();

    let mut redefine: &str = "";
    let mut redefine_state = 0;
    for term in input {
      if term == ";" {
        redefine_state = 0;
        continue;
      } else if redefine_state == 1 {
        if let Some(x) = term.chars().nth(0) {
          if x.is_numeric() {
            return Err(Error::InvalidWord);
          }
        }

        self.redefines.insert(term.to_owned(), Vec::new());
        redefine = term;
        redefine_state = 2;
        continue;
      } else if redefine_state == 2 {
        if let Some(e) = self.redefines.get_mut(redefine) {
          e.push(term.to_owned());
        }
        continue;
      }

      let mut terms: Vec<String> = Vec::new();
      if let Some(e) = self.redefines.get(term) {
        terms.push_all(e);
      } else {
        terms.push(term.to_owned());
      }

      for arg in terms {
        match arg.as_str() {
          "+" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              self.stack.push(y+x);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "-" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              self.stack.push(y-x);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "*" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              self.stack.push(y*x);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "/" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              if x == 0 {
                return Err(Error::DivisionByZero);
              }
              self.stack.push(y/x);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "dup" => {
            if let Some(x) = self.stack.pop() {
              self.stack.push(x);
              self.stack.push(x);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "drop" => {
            if let Some(_) = self.stack.pop() {
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "swap" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              self.stack.push(x);
              self.stack.push(y);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          "over" => {
            if let (Some(x), Some(y)) = (self.stack.pop(), self.stack.pop()) {
              self.stack.push(y);
              self.stack.push(x);
              self.stack.push(y);
            } else {
              return Err(Error::StackUnderflow);
            }
          },
          ":" => {
            redefine_state = 1;
          },
          _ => {
            if let Ok(int) = arg.parse::<i32>() {
              self.stack.push(int);
            } else {
              return Err(Error::UnknownWord);
            }
          }
        }
      }
    }

    if redefine_state != 0 {
      return Err(Error::InvalidWord);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn bench_count(b: &mut Bencher) {
    let mut f = Forth::new();
    b.iter(|| f.eval("1 2 + 4 -"));
  }
}
