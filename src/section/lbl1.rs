use crate::{
  traits::{CalculatesSize, Updates},
};
use super::Section;

#[derive(Debug, Clone)]
pub struct Lbl1 {
  pub(crate) section: Section,
  pub(crate) groups: Vec<Group>,
  pub(crate) labels: Vec<Label>,
}

impl Lbl1 {
  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn groups(&self) -> &[Group] {
    &self.groups
  }

  pub fn labels(&self) -> &[Label] {
    &self.labels
  }

  pub fn labels_mut(&mut self) -> &mut [Label] {
    &mut self.labels
  }

  fn update_group_offsets(&mut self) {
    let mut total = 0;
    let group_len = self.groups.len() as u32;
    let checksums: Vec<u32> = self.labels.iter()
      .map(|lbl| lbl.checksum(self)).collect();
    for (i, group) in self.groups.iter_mut().enumerate() {
      group.offset = group_len * group.calc_size() as u32
        + std::mem::size_of::<u32>() as u32 // group count
        + total;
      total += self.labels.iter().enumerate()
        .filter(|(k,_)| checksums[*k] == i as u32)
        .map(|(_,lbl)| lbl.calc_size() as u32)
        .sum::<u32>();
    }
  }
}

#[derive(Debug, Clone)]
pub struct Group {
  pub(crate) label_count: u32,
  pub(crate) offset: u32,
}

impl Group {
  pub fn label_count(&self) -> u32 {
    self.label_count
  }

  pub fn offset(&self) -> u32 {
    self.offset
  }
}

#[derive(Debug, Clone)]
pub struct Label {
  pub(crate) name: String,
}

impl Label {
  pub(crate) const HASH_MAGIC: u32 = 0x492;

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name<S>(&mut self, name: S)
    where S: Into<String>,
  {
    self.name = name.into();
  }

  pub fn checksum(&self, lbl1: &Lbl1) -> u32 {
    let hash: u32 = self.name.as_bytes().iter()
      .fold(0, |hash, b| hash.overflowing_mul(Label::HASH_MAGIC).0.overflowing_add(u32::from(*b)).0);
    hash % lbl1.groups.len() as u32
  }
}

impl Updates for Lbl1 {
  fn update(&mut self) {
    self.section.size = self.calc_size() as u32 - self.section.calc_size() as u32;
    self.update_group_offsets();
  }
}

impl CalculatesSize for Lbl1 {
  fn calc_size(&self) -> usize {
    self.section.calc_size()
      + std::mem::size_of::<u32>() // group count
      + self.groups.iter().map(&CalculatesSize::calc_size).sum::<usize>()
      + self.labels.iter().map(&CalculatesSize::calc_size).sum::<usize>()
  }
}

impl CalculatesSize for Group {
  fn calc_size(&self) -> usize {
    std::mem::size_of_val(&self.label_count) + std::mem::size_of_val(&self.offset)
  }
}

impl CalculatesSize for Label {
  fn calc_size(&self) -> usize {
    std::mem::size_of::<u8>() // name length
      + self.name.as_bytes().len()
      + std::mem::size_of::<u32>() // index
  }
}
