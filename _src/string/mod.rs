
pub struct Str {
  string: String
}

impl Str {
  pub fn new() -> Str {
    Str {
      string: String::new()
    }
  }
  pub fn from(str: &str) -> Str {
    Str {
      string: String::from(str)
    }
  }
  pub fn clone(&self) -> Str {
    Str {
      string: self.string.clone()
    }
  }
  pub fn str(&self) -> &str {
    &self.string
  }
  
}
