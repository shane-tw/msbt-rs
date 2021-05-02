use crate::{
  Encoding,
  Header,
  Msbt,
  SectionTag,
  section::*,
  traits::Updates,
};

use byteordered::Endianness;

pub struct MsbtBuilder {
  section_order: Vec<SectionTag>,
  header: Header,
  lbl1: Option<Lbl1>,
  txt2: Option<Txt2>,
  nli1: Option<Nli1>,
  ato1: Option<Ato1>,
  atr1: Option<Atr1>,
  tsy1: Option<Tsy1>,
  pad_byte: u8,
}

  macro_rules! add_item {
    ($lower:ident, $upper:ident) => {
      pub fn $lower(mut self, $lower: $upper) -> Self {
        if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::$upper) {
          self.section_order.remove(pos);
        }
        self.section_order.push(SectionTag::$upper);
        self.$lower = Some($lower);

        self
      }
    };
  }

impl MsbtBuilder {
  pub fn new(endianness: Endianness, encoding: Encoding, group_count: Option<u32>) -> Self {
    let lbl1 = group_count
      .map(|gc| {
        let groups = (0..gc)
          .map(|_| crate::section::lbl1::Group {
            label_count: 0,
            offset: 0,
          })
          .collect();
        Lbl1 {
          section: Section::new(*b"LBL1", 0),
          groups,
          labels: Vec::with_capacity(gc as usize),
        }
      });
    let txt2 = group_count.map(|_| Txt2 {
      section: Section::new(*b"TXT2", 0),
      values: Vec::new(),
    });
    let (section_count, section_order) = if group_count.is_some() {
      let mut order = Vec::with_capacity(6);
      order.push(SectionTag::Lbl1);
      order.push(SectionTag::Txt2);
      (2, order)
    } else {
      (0, Vec::with_capacity(6))
    };
    MsbtBuilder {
      section_order,
      header: Header {
        magic: crate::HEADER_MAGIC,
        endianness,
        _unknown_1: 0,
        encoding,
        _unknown_2: 3,
        section_count,
        _unknown_3: 0,
        file_size: 0,
        padding: [0; 10],
      },
      lbl1,
      txt2,
      nli1: None,
      ato1: None,
      atr1: None,
      tsy1: None,
      pad_byte: 0,
    }
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn build(self) -> Msbt {
    let mut msbt = Msbt {
      header: self.header,
      section_order: self.section_order,
      lbl1: self.lbl1,
      nli1: self.nli1,
      ato1: self.ato1,
      atr1: self.atr1,
      tsy1: self.tsy1,
      txt2: self.txt2,
      pad_byte: self.pad_byte,
    };

    if let Some(lbl1) = msbt.lbl1.as_mut() {
      lbl1.update();
    }
    if let Some(txt2) = msbt.txt2.as_mut() {
      txt2.update();
    }

    msbt.update();

    msbt
  }

  pub fn add_label<N: Into<String>, V: Into<Vec<u8>>>(mut self, name: N, value: V) -> Self {
    let name = name.into();
    let value = value.into();

    let lbl1 = match self.lbl1.as_mut() {
      Some(l) => l,
      None => return self,
    };
    let txt2 = match self.txt2.as_mut() {
      Some(l) => l,
      None => return self,
    };

    let label = crate::section::lbl1::Label {
      name,
      index: txt2.values().len() as u32,
    };

    txt2.values.push(value);
    let group_index = label.checksum(&lbl1) as usize;
    lbl1.groups[group_index].label_count += 1;
    lbl1.labels.push(label);

    self
  }

  add_item!(nli1, Nli1);
  add_item!(ato1, Ato1);
  add_item!(atr1, Atr1);
  add_item!(tsy1, Tsy1);
  add_item!(txt2, Txt2);
}
