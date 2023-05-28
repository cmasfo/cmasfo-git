
#[derive(Default, Clone, PartialEq, Eq)]
pub struct DevStr {
  string: String
}

// basic
impl DevStr {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn from(devstr: &Self) -> Self {
    devstr.clone()
  }
}

// compatibility
impl DevStr {
  pub fn from_string(string: String) -> Self {
    Self {
      string
    }
  }
  pub fn to_string(&self) -> String {
    self.string.clone()
  }
  pub fn from_str(str: &str) -> Self {
    Self {
      string: String::from(str)
    }
  }
  pub fn to_str(&self) -> &str {
    &self.string
  }
}

use std::{
  str::{
    Chars, Split, SplitWhitespace
  }
};

// feature
impl DevStr {
  pub fn chars(&self) -> Chars<'_> {
    self.string.chars()
  }
  pub fn rev(&self) -> Self {
    let chars = self.string.chars().rev();
    Self {
      string: chars.into_iter().collect()
    }
  }
  pub fn nth(&self, idx: usize) -> Option<char> {
    self.string.chars().nth(idx)
  }
  pub fn split(&self, split: char) -> Split<char> {
    self.string.split(split)
  }
  pub fn split_str<'a>(&self, split: &'a str) -> Split<&'a str> {
    self.string.split(split)
  }
  pub fn split_space(&self) -> SplitWhitespace {
    self.string.split_whitespace()
  }
  pub fn to_upper(&self) -> Self {
    Self {
      string: self.string.to_uppercase()
    }
  }
  pub fn to_lower(&self) -> Self {
    Self {
      string: self.string.to_lowercase()
    }
  }
} // impl DevStr

// utility
impl DevStr {
  pub fn roman_to_integer(&self) -> Self {
    todo!()
  }
  pub fn integer_to_roman(&self) -> Self {
    todo!()
  }
}
