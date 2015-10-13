
pub fn number(input: &str) -> Option<String> {
  let mut s: String = input.chars().filter(|x| {
    *x as u8 >= b'0' && *x as u8 <= b'9'
  }).collect();

  if s.len() < 10 || s.len() > 11 {
    None
  } else if s.len() == 11 {
    if s.chars().nth(0).unwrap() == '1' {
      s.remove(0);
      Some(s)
    } else {
      None
    }
  } else {
    Some(s)
  }
}

pub fn pretty_print(input: &str) -> String {
  match number(input) {
    Some(num) => {
      format!("({}) {}-{}", &num[..3], &num[3..6], &num[6..])
    },
    None => {
      "invalid".to_owned()
    }
  }
}

pub fn area_code(input: &str) -> Option<String> {
  match number(input) {
    Some(num) => {
      Some(format!("{}", &num[..3]))
    },
    None => None,
  }
}
