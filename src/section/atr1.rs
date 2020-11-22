use crate::{
  Msbt,
  traits::CalculatesSize,
};
use super::Section;

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Atr1 {
  pub(crate) msbt: NonNull<Msbt>,
  pub(crate) section: Section,
  pub(crate) _unknown: Vec<u8>,
}

impl Atr1 {
  pub fn new_unlinked<V: Into<Vec<u8>>>(unknown_bytes: V) -> Self {
    let bytes = unknown_bytes.into();
    Atr1 {
      msbt: NonNull::dangling(),
      section: Section::new(*b"ATR1", bytes.len() as u32),
      _unknown: bytes,
    }
  }

  pub fn msbt(&self) -> &Msbt {
    unsafe { self.msbt.as_ref() }
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn unknown_bytes(&self) -> &[u8] {
    &self._unknown
  }
}

impl CalculatesSize for Atr1 {
  fn calc_size(&self) -> usize {
    self.section.calc_size() + self._unknown.len()
  }
}
