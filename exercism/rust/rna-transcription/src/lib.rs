use std::convert::AsRef;

#[derive(Clone, PartialEq, Debug)]
pub struct RibonucleicAcid(pub String);

impl RibonucleicAcid {
  pub fn new(s: &str) -> RibonucleicAcid {
    RibonucleicAcid(s.to_owned())
  }

  pub fn as_ref(&self) -> &str {
    &self.0
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeoxyribonucleicAcid(pub String);

impl DeoxyribonucleicAcid {
  pub fn new(s: &str) -> DeoxyribonucleicAcid {
    DeoxyribonucleicAcid(s.to_owned())
  }

  pub fn to_rna(&self) -> RibonucleicAcid {
    let rna: String = self.0.chars().map(|x| {
      match x {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => unreachable!(),
      }
    }).collect();

    RibonucleicAcid::new(&rna)
  }
}
