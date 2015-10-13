use std::slice::Iter;
use std::iter::{IntoIterator, FromIterator};

pub struct CustomSet<T> {
  data: Vec<T>,
}

impl<T> CustomSet<T> where T: Eq + Clone {
  pub fn new() -> CustomSet<T> {
    CustomSet{
      data: Vec::new(),
    }
  }

  pub fn insert(&mut self, el: T) -> bool {
    if !self.contains(&el) {
      self.data.push(el);
      return true;
    }
    false
  }

  pub fn remove(&mut self, el: &T) -> bool {
    let mut index: Option<usize> = None;
      for (i, element) in self.data.iter().enumerate() {
        if el == element {
          index = Some(i);
          break;
        }
      }
      match index {
        Some(index) => {
          self.data.remove(index);
          true
        }
        None => false
      }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn contains(&self, el: &T) -> bool {
    for element in self.data.iter() {
      if el == element {
        return true;
      }
    }
    false
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
    let mut cs: CustomSet<T> = CustomSet::new();
    for i in self.iter() {
      if !other.contains(i) {
        cs.insert(i.clone());
      }
    }
    cs
  }

  pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
    let mut cs: CustomSet<T> = CustomSet::new();
    for i in self.iter() {
      if other.contains(i) {
        cs.insert(i.clone());
      }
    }
    cs
  }

  pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
    let mut cs: CustomSet<T> = CustomSet::new();
    for i in self.iter().chain(other.iter()) {
      cs.insert(i.clone());
    }
    cs
  }

  pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
    for i in other.iter() {
      if self.contains(i) {
        return false;
      }
    }
    true
  }

  pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
    for i in self.iter() {
      if !other.contains(i) {
        return false;
      }
    }
    true
  }

  pub fn is_superset(&self, other: &CustomSet<T>) -> bool {
    other.is_subset(self)
  }

  pub fn clear(&mut self) {
    self.data.clear();
  }

  pub fn iter(&self) -> Iter<T> {
    self.data.iter()
  }
}

impl<T> FromIterator<T> for CustomSet<T> where T: Eq + Clone {
  fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> CustomSet<T> {
    let mut set = CustomSet::new();
    set.data = Vec::from_iter(iterable);
    set
  }
}
