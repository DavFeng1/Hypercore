use crate::transaction::Transaction;
use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone)]
pub struct Block {
  pub hash: String,
  pub previous_hash: String,
  pub timestamp: i64,
  pub nonce: u64,
  pub transaction: Option<Transaction>,
}

impl Block {
  pub fn create(previous_hash: String, transaction: Transaction) -> Self {
    let timestamp = Utc::now().timestamp();
    let nonce = 2836;
    let hash =
      Block::generate_hash(timestamp, nonce, previous_hash.clone());

    Self {
      timestamp: timestamp,
      previous_hash: previous_hash,
      nonce: nonce,
      hash: hash,
      transaction: Some(transaction),
    }
  }

  pub fn generate_genesis() -> Self {
    let timestamp = Utc::now().timestamp();
    let nonce = 2837;
    let previous_hash = String::from("genesis");
    let hash =
      Block::generate_hash(timestamp, nonce, previous_hash.clone());

    Self {
      timestamp: timestamp,
      previous_hash: previous_hash,
      nonce: nonce,
      hash: hash,
      transaction: None,
    }
  }

  pub fn generate_hash(
    timestamp: i64,
    nonce: u64,
    previous_hash: String,
  ) -> String {
    let serialized_data = json!({
        "previous_hash": previous_hash,
        "timestamp": timestamp,
        "nonce": nonce
    });

    let mut hasher = Sha256::new();
    hasher.update(serialized_data.to_string().as_bytes());
    format!("{:X}", hasher.finalize())
  }
}

impl fmt::Display for Block {
  // This trait requires `fmt` with this exact signature.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "[ hash: {}, previous_hash: {}, timestamp: {}]",
      self.hash, self.previous_hash, self.timestamp
    )
  }
}
