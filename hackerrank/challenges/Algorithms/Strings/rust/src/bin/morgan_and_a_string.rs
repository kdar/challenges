// https://www.hackerrank.com/challenges/morgan-and-a-string

fn main() {
  let mut stdin = std::io::stdin();
  let mut line = String::new();
  stdin.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    let mut a = String::new();
    stdin.read_line(&mut a).ok().unwrap();
    let mut b = String::new();
    stdin.read_line(&mut b).ok().unwrap();

    a = a.trim().to_string();
    b = b.trim().to_string();

    a.push('z');
    b.push('z');
    let (la, lb) = (a.len(), b.len());
    let (mut i, mut j) = (0, 0);

    let mut lex = String::new();

    while i < la && j < lb {
      if a[i..] < b[j..] {
        lex.push(a.chars().nth(i).unwrap());
        i += 1;

        // optimization to just keep copying from a
        // if it's the same character we just copied
        let tmp = a.chars().nth(i-1).unwrap();
        while i < la && a.chars().nth(i).unwrap() == tmp {
          lex.push(a.chars().nth(i).unwrap());
          i += 1
        }
      } else {
        lex.push(b.chars().nth(j).unwrap());
        j += 1;

        // optimization to just keep copying from b
        // if it's the same character we just copied
        let tmp = b.chars().nth(j-1).unwrap();
        while j < lb && b.chars().nth(j).unwrap() == tmp {
          lex.push(b.chars().nth(j).unwrap());
          j += 1
        }
      }
    }

    for x in i..la {
      lex.push(a.chars().nth(x).unwrap());
    }
    for x in j..lb {
      lex.push(b.chars().nth(x).unwrap());
    }

    println!("{}", lex.chars().take(lex.len()-2).collect::<String>());
  }
}
