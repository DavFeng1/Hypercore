use rsa::RsaPublicKey;

pub struct Transaction {
  pub amount: i64,
  pub payer: RsaPublicKey, // public key
  pub payee: RsaPublicKey, // public key
}

impl Transaction {
  pub fn create(amount: i64, payer: RsaPublicKey, payee: RsaPublicKey) -> Self {
    println!("created transaction!");
    Self {
      payer: payer,
      payee: payee,
      amount: amount,
    }
  }

  pub fn to_string(&self) -> String {
    String::from("This transaction as a string")
  }
}
