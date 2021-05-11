use crate::traits::{CalculatesSize, Updates};
use super::Section;
use std::io::{Read, Seek, Cursor};

pub const TAG_START: u16 = 0x0E;
pub const TAG_END: u16 = 0x0F;

#[derive(Debug, Clone)]
pub struct Txt2 {
  pub(crate) section: Section,
  pub(crate) values: Vec<Vec<Token>>,
}

impl Txt2 {
  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn values(&self) -> &[Vec<Token>] {
    &self.values
  }
}

impl CalculatesSize for Txt2 {
  fn calc_size(&self) -> usize {
    self.section.calc_size()
      + std::mem::size_of::<u32>() // value count
      + std::mem::size_of::<u32>() * self.values.len() // offsets
      + self.values.iter()
        .map(|v| v.iter().flat_map(|vv| vv.to_bytes())
        .collect::<Vec<u8>>().len()).sum::<usize>()
  }
}

impl Updates for Txt2 {
  fn update(&mut self) {
    let value_count = self.values.len() as u32;
    let values_size = self.values.iter()
      .map(|v| v.iter().flat_map(|vv| vv.to_bytes())
      .collect::<Vec<u8>>().len()).sum::<usize>();
    let new_size = values_size
      + value_count as usize * std::mem::size_of::<u32>() // all offsets
      + std::mem::size_of_val(&value_count);
    self.section.size = new_size as u32;
  }
}

pub fn parse_bytes(bytes: &[u8]) -> Vec<Token> {
  let mut rdr = byteordered::ByteOrdered::le(Cursor::new(bytes));
  let mut tokens: Vec<Token> = Vec::new();
  
  while let Ok(byte) = rdr.read_u16() {
    match byte {
      TAG_START => {
        let group_code = rdr.read_u16().unwrap();
        let tag_code = rdr.read_u16().unwrap();
        let params_size = rdr.read_u16().unwrap() as usize;
        let mut params = vec![0; params_size];
        rdr.read_exact(&mut params).unwrap();
        tokens.push(Token::TagStart(group_code, tag_code, params));
      },
      TAG_END => tokens.push(Token::TagEnd),
      0x00 => {
        // Some games e.g. mario & luigi have null bytes in text
        rdr.seek(std::io::SeekFrom::Current(-2)).unwrap();
        let mut padding_end = Vec::new();
        rdr.read_to_end(&mut padding_end).unwrap();
        tokens.push(Token::Padding(padding_end));
      },
      _ => {
        if tokens.len() == 0 {
          tokens.push(Token::Text(Vec::new()));
        } else {
          match tokens.get(tokens.len() - 1) {
            Some(Token::Text(_)) => (),
            _ => tokens.push(Token::Text(Vec::new()))
          };
        }
        let last_i = tokens.len() - 1;
        if let Some(Token::Text(ref mut b)) = tokens.get_mut(last_i) {
          b.extend(&byte.to_le_bytes());
        }
      }
    };
  };

  tokens
}

#[derive(Debug, Clone)]
pub enum Token {
  TagStart(u16, u16, Vec<u8>),
  Text(Vec<u8>),
  TagEnd,
  Padding(Vec<u8>)
}

impl Token {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      Self::TagStart(group_code, tag_code, param_bytes) => {
        let mut result = Vec::new();
        result.extend(&TAG_START.to_le_bytes());
        result.extend(&group_code.to_le_bytes());
        result.extend(&tag_code.to_le_bytes());
        result.extend(param_bytes);
        result
      },
      Self::Text(bytes) => bytes.to_vec(),
      Self::TagEnd => TAG_END.to_le_bytes().to_vec(),
      Self::Padding(padding) => {
        let mut result = Vec::new();
        result.extend(&0u16.to_le_bytes());
        result.extend(padding);
        result
      }
    }
  }
}
