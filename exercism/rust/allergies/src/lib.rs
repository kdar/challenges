#[derive(Clone, PartialEq, Debug)]
pub enum Allergen {
  Eggs = 1,
  Peanuts = 2,
  Shellfish = 4,
  Strawberries = 8,
  Tomatoes = 16,
  Chocolate = 32,
  Pollen = 64,
  Cats = 128,
}

pub struct Allergies(pub u32);

impl Allergies {
  pub fn allergies(&self) -> Vec<Allergen> {
    let mut a = vec![Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish,
      Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate,
      Allergen::Pollen, Allergen::Cats];
    a.retain(|x| (self.0 & x.clone() as u32) != 0);
    a
  }

  pub fn is_allergic_to(&self, a: &Allergen) -> bool {
    (self.0 & a.clone() as u32) != 0
  }
}
