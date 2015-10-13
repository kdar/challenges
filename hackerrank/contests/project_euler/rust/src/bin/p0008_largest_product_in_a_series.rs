use std::io::{BufReader, BufRead};
use std::env;

#[allow(dead_code)]
static TEST: &'static str = "3
10 5
3675356291
10 5
2709360626
10 5
73167176531330624919225119674426574742355349194999
10 13
7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

fn solve(nums: String, k: usize) -> u64 {
  // split by 0's because we don't want to divide or multiply by 0.
  let mut max = 0;
  for section in nums.split("0") {
    if section.len() < k {
      continue;
    }

    // Compute max product O(n). Works by multiplying the first k terms,
    // and then for each iteration divide the product by n-k and then
    // multiply it by the next term. Thereby getting the product of
    // the next k terms;
    let mut product = section.chars().take(k).fold(1, |t,x| {
      t * (x as u64 - '0' as u64)
    });
    if product > max {
      max = product;
    }

    for n in k..section.len() {
      let remove: u64 = section.chars().nth(n-k).unwrap() as u64 - '0' as u64;
      let next: u64 = section.chars().nth(n).unwrap() as u64 - '0' as u64;
      product = product / remove * next;
      if product > max {
        max = product;
      }
    }
  }


  // // naive way of computing: O(n*k)
  // let mut max2: u64 = 0;
  // for n in 0..nums.len()-k+1 {
  //   let mut product = 1u64;
  //   for j in nums.chars().skip(n).take(k) {
  //     product *= j as u64 - '0' as u64;
  //   }
  //   if product > max2 {
  //     max2 = product;
  //   }
  // }

  max
}

fn setup(input: &mut BufRead) {
  let mut line = String::new();
  input.read_line(&mut line).ok().unwrap();
  let test_cases = line.trim().parse().unwrap();

  for _ in 0..test_cases {
    line.clear();
    input.read_line(&mut line).ok().unwrap();
    let nk: Vec<usize> = line.trim().split(' ').map(|x| x.trim().parse::<usize>().unwrap()).collect();

    let mut nums = String::new();
    input.read_line(&mut nums).ok().unwrap();
    nums = nums.trim().to_string();

    println!("{}", solve(nums, nk[1]));
  }
}

fn main() {
  // for testing purposes
  if let Some(arg) = env::args().nth(1) {
    if arg == "test" {
      let mut br = BufReader::new(TEST.as_bytes());
      setup(&mut br);
      return;
    }
  }

  let stdin = std::io::stdin();
  let mut br = BufReader::new(stdin);
  setup(&mut br);
}
