use crate::traits::CalculatesSize;
use super::Section;

#[derive(Debug, Clone)]
pub struct Ato1 {
  pub(crate) section: Section,
  pub(crate) _unknown: Vec<u8>, // large collection of 0xFF
}

impl Ato1 {
  pub fn new_unlinked<V: Into<Vec<u8>>>(unknown_bytes: V) -> Self {
    let bytes = unknown_bytes.into();
    Ato1 {
      section: Section::new(*b"ATO1", bytes.len() as u32),
      _unknown: bytes,
    }
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn unknown_bytes(&self) -> &[u8] {
    &self._unknown
  }
}

impl CalculatesSize for Ato1 {
  fn calc_size(&self) -> usize {
    self.section.calc_size() + self._unknown.len()
  }
}
