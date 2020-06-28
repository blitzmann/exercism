extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;

pub struct Allergies(u32);

#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        *allergen as u32 & self.0 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter().filter(|a| *a as u32 & self.0 != 0).collect()
    }
}
