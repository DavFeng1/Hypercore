use rand::rngs::OsRng;
use rsa::{
  pkcs1::ToRsaPublicKey, PaddingScheme, PublicKey, RsaPrivateKey,
  RsaPublicKey,
};
use serde_json::json;

pub struct Wallet {
  pub slug: String,
  pub public_key: rsa::RsaPublicKey,
  private_key: rsa::RsaPrivateKey,
}

impl Wallet {
  pub fn create(slug: String) -> Self {
    println!("💳 generating wallet for {}...", slug);

    let bits = 2048;

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits)
      .expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    Self {
      public_key: public_key,
      private_key: private_key,
      slug: slug,
    }
  }

  pub fn create_signature(
    &mut self,
    amount: u64,
    payee: Wallet,
  ) -> Vec<u8> {
    let mut rng = OsRng;
    let payee_pem =
      ToRsaPublicKey::to_pkcs1_pem(&payee.public_key).expect("got pem");

    let json_value = json!({
        "payee_public_key": payee_pem,
        "amount": amount,
    })
    .to_string();

    println!("Encrypting {:?} with private key", json_value);

    let encrypted_data = self
      .private_key
      .encrypt(
        &mut rng,
        PaddingScheme::new_pkcs1v15_encrypt(),
        &json_value.as_bytes()[..],
      )
      .expect("failed to encrypt data");
    encrypted_data
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
