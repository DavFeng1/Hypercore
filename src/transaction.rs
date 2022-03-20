use rsa::RsaPublicKey;

#[derive(Clone)]
pub struct Transaction {
  pub amount: u64,
  pub payer: RsaPublicKey, // public key
  pub payee: RsaPublicKey, // public key
  pub signature: Vec<u8>,
}

impl Transaction {
  pub fn create(
    amount: u64,
    payer: RsaPublicKey,
    payee: RsaPublicKey,
    signature: Vec<u8>,
  ) -> Self {
    Self {
      payer: payer,
      payee: payee,
      amount: amount,
      signature: signature,
    }
  }
}
