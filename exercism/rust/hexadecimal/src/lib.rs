
pub fn hex_to_int(input: &str) -> Option<u32> {
  let mut exp = 0u32;
  let mut answer = 0;
  for i in input.chars().rev() {
    let n: u8 = i as u8;
    let n: u8 = if n >= b'a' && n <= b'f' {
      n  - b'a' + 10
    } else if n >= b'0' && n <= b'9' {
      n - b'0'
    } else {
      return None;
    };

    answer += n as u32 * 16u32.pow(exp);
    exp += 1u32;
  }

  Some(answer)
}
