use std::fmt;

pub struct Block {
  pub id: u64,
  pub hash: String,
  pub previous_hash: String,
  pub timestamp: i64,
  pub nonce: u64,
}

impl Block {}

impl fmt::Display for Block {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "[ id: {}, hash: {}, previous_hash: {}, timestamp: {} ]",
      self.id, self.hash, self.previous_hash, self.timestamp
    )
  }
}
