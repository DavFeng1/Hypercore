use crate::block::Block;
use crate::wallet::Wallet;
use chrono::Utc;

// Base
pub struct Hypercore {
  pub blocks: Vec<Block>,
  pub wallets: Vec<Wallet>,
}

impl Hypercore {
  pub fn init() -> Self {
    println!("💎 Initializing hypercore...");
    /*
     * Initialize the chain with a gensis block
     */
    let genesis_block = Block {
      id: 0,
      timestamp: Utc::now().timestamp(),
      previous_hash: String::from("genesis"),
      nonce: 2836,
      hash: String::from("000f2828694d5b4c43"),
    };

    Self {
      blocks: vec![genesis_block],
      wallets: vec![],
    }
  }

  pub fn add_wallet(&mut self, wallet_slug: String) -> Wallet {
    let wallet = Wallet::create(wallet_slug);
    self.wallets.push(wallet.clone());
    wallet
  }

  pub fn add_core(&mut self, block: Block) {
    /*
     * Attach a core to the current chain.
     */
    let latest_block = self.blocks.last().expect("there is at least one block");

    // Verifier = crypto.createVerify('sha256')
    // verifier update the transaction
    // Check if transsation is valid
    if self.is_core_valid(&block, latest_block) {
      self.blocks.push(block);
    } else {
      log::error!("could not add a block - block invalid")
    }
  }

  pub fn is_core_valid(&self, block: &Block, previous_block: &Block) -> bool {
    /*
     * Check whether an incoming core is valid
     * by validating the previous_hash field
     */
    if block.previous_hash != previous_block.hash {
      log::warn!("block with id {} has invalid previous_hash", block.id);
      false;
    }
    true
  }

  pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
    for i in 0..chain.len() {}
    true
  }
}
