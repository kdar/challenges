extern crate threadpool;

use std::collections::HashMap;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub fn frequency(data: &[&'static str], workers: usize) -> HashMap<char, usize> {
  let mut h: HashMap<char, usize> = HashMap::new();
  let pool = threadpool::ThreadPool::new(workers);

  let (tx, rx) = mpsc::channel();

  for &line in data {
    let tx = tx.clone();
    pool.execute(move || {
      tx.send(calculate(line)).unwrap();
    })
  }

  for _ in 0..data.len() {
    let partial = rx.recv().unwrap();
    for (c, n) in partial.into_iter() {
      let v =h.entry(c).or_insert(0);
      *v = *v + n;
    }
  }

  h
}

fn calculate(line: &str) -> HashMap<char, usize> {
  let mut h: HashMap<char, usize> = HashMap::new();
  for x in line.chars() {
    if x.is_alphabetic() {
      let v = h.entry(x.to_lowercase().next().unwrap()).or_insert(0);
      *v = *v + 1;
    }
  }
  h
}
