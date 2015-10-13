// Simple/unoptimized way of finding rectangles. First find all the corners that are
// connected to other corners. Then for each corner that connects RIGHT and DOWN, find
// another corner that connects LEFT and UP, and test if they fully connect to each other
// to form a rectangle.
//
// Even though this is unoptmized, it's about 8 times faster than the public solution.

#![feature(test)]

extern crate test;

use std::fmt;

const DIR_RIGHT: usize = 0x02;
const DIR_DOWN: usize = 0x04;
const DIR_LEFT: usize = 0x08;
const DIR_UP: usize = 0x20;

#[derive(Debug)]
struct Corner {
  col: usize,
  row: usize,
  dir: usize,
}

// For debugging purposes.
impl fmt::Display for Corner {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut dir = "".to_owned();
    if self.dir & DIR_RIGHT > 0 { dir = dir + "_RIGHT_"; }
    if self.dir & DIR_DOWN > 0 { dir = dir + "_DOWN_"; }
    if self.dir & DIR_LEFT > 0 { dir = dir + "_LEFT_"; }
    if self.dir & DIR_UP > 0 { dir = dir + "_UP_"; }

    write!(f, "Corner{{col:{}, row:{}, dir:{}}}", self.col, self.row, dir)
  }
}

pub fn count(lines: &Vec<&str>) -> u32 {
  if lines.len() < 3 {
    return 0;
  }

  let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
  let mut corners: Vec<Corner> = Vec::new();
  for row in 0..chars.len() {
    for col in 0..chars[row].len() {
      if chars[row][col] == '+' {
        if let Some(corner) = get_corner(&chars, row, col) {
          corners.push(corner);
        }
      }
    }
  }

  let mut count = 0;
  for corner1 in corners.iter() {
    if corner1.dir & (DIR_RIGHT|DIR_DOWN) != DIR_RIGHT|DIR_DOWN {
      continue;
    }
    for corner2 in corners.iter() {
      if corner2.dir & (DIR_LEFT|DIR_UP) != DIR_LEFT|DIR_UP {
        continue;
      }

      if corner2.col > corner1.col && corner2.row > corner1.row {
        if is_connected(&chars, corner1, corner2) {
          count += 1;
        }
      }
    }
  }

  count
}

// See if corner1 is connected fully to corner2. Meaning, there are no empty spaces
// between the two.
fn is_connected(chars: &Vec<Vec<char>>, corner1: &Corner, corner2: &Corner) -> bool {
  // search up
  !(corner1.row+1..corner2.row+1).any(|x| chars[x][corner1.col] == ' ') &&
  // search right
  !(corner1.col+1..corner2.col+1).any(|x| chars[corner1.row][x] == ' ') &&
  // search down
  !(corner1.row+1..corner2.row+1).rev().any(|x| chars[x][corner2.col] == ' ') &&
  // search left
  !(corner1.col+1..corner2.col+1).rev().any(|x| chars[corner2.row][x] == ' ')
}

fn get_corner(chars: &Vec<Vec<char>>, row: usize, col: usize) -> Option<Corner> {
  let mut dir = 0usize;

  // search up
  for x in (0..row).rev() {
    match chars[x][col] {
      '+' => dir |= DIR_UP,
      '|' => {},
      _ => break,
    };
  }

  // search right
  for x in col+1..chars[0].len() {
    match chars[row][x] {
      '+' => dir |= DIR_RIGHT,
      '-' => {},
      _ => break,
    };
  }

  // search down
  for x in row+1..chars.len() {
    match chars[x][col] {
      '+' => dir |= DIR_DOWN,
      '|' => {},
      _ => break,
    };
  }

  // search left
  for x in (0..col).rev() {
    match chars[row][x] {
      '+' => dir |= DIR_LEFT,
      '-' => {},
      _ => break,
    };
  }

  if dir & (DIR_UP | DIR_DOWN) > 0 && dir & (DIR_LEFT | DIR_RIGHT) > 0 {
    return Some(Corner{col: col, row: row, dir: dir});
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn bench_count(b: &mut Bencher) {
    let lines = vec![
      "+------+----+",
      "|      |    |",
      "+---+--+    |",
      "|   |       |",
      "+---+-------+"
      ];
    b.iter(|| count(&lines));
  }
}
