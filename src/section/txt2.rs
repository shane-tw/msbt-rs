use crate::traits::{CalculatesSize, Updates};
use super::Section;

#[derive(Debug, Clone)]
pub struct Txt2 {
  pub(crate) section: Section,
  pub(crate) values: Vec<Vec<u8>>,
}

impl Txt2 {
  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn values(&self) -> &[Vec<u8>] {
    &self.values
  }
}

impl CalculatesSize for Txt2 {
  fn calc_size(&self) -> usize {
    self.section.calc_size()
      + std::mem::size_of::<u32>() // value count
      + std::mem::size_of::<u32>() * self.values.len() // offsets
      + self.values.iter().map(Vec::len).sum::<usize>()
  }
}

impl Updates for Txt2 {
  fn update(&mut self) {
    let value_count = self.values.len() as u32;
    let values_len = self.values.iter().map(Vec::len).sum::<usize>();
    let new_size = values_len // length of all values
      + value_count as usize * std::mem::size_of::<u32>() // all offsets
      + std::mem::size_of_val(&value_count); // length of string count
    self.section.size = new_size as u32;
  }
}
