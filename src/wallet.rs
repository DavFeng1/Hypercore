use crate::transaction::Transaction;
use rand::rngs::OsRng;
use rsa::{pkcs8::ToPublicKey, RsaPrivateKey, RsaPublicKey};

pub struct Wallet {
  pub slug: String,
  pub public_key: rsa::RsaPublicKey,
  private_key: rsa::RsaPrivateKey,
}

impl Wallet {
  pub fn create(slug: String) -> Self {
    println!("💳 generating wallet for {}...", slug);
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    Self {
      public_key: public_key,
      private_key: private_key,
      slug: slug,
    }
    // Signing
    // key pair , format: PEM,
  }

  pub fn send_money(&self, amount: &i64, payee_public_key: &RsaPublicKey) {
    let key_pem = ToPublicKey::to_public_key_pem(payee_public_key).expect("failed to generate pem");
    println!("💸 initiating transaction of {}", amount);

    // Create transaction
    let transaction =
      Transaction::create(*amount, self.public_key.clone(), payee_public_key.clone());

    // Sign the key

    // create the block

    // Add block to chain
  }
}

impl Clone for Wallet {
  fn clone(&self) -> Wallet {
    Wallet {
      private_key: self.private_key.clone(),
      public_key: self.public_key.clone(),
      slug: self.slug.clone(),
    }
  }
}
