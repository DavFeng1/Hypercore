use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct Hypercore {
  pub blocks: Vec<Block>,
  pub wallets: Vec<Wallet>,
}

impl Hypercore {
  pub fn create() -> Self {
    println!("💎 Initializing hypercore...");
    Self {
      blocks: vec![Block::generate_genesis()],
      wallets: vec![],
    }
  }

  pub fn add_wallet(&mut self, wallet_slug: String) -> Wallet {
    let wallet = Wallet::create(wallet_slug);
    self.wallets.push(wallet.clone());
    wallet
  }

  pub fn add_block_to_chain(&mut self, block: Block) {
    if self.is_block_valid(block.clone()) {
      self.blocks.push(block.clone());
      println!("📦 new block added: {}", block);
    } else {
      log::error!("could not add a block - block invalid")
    }
  }

  pub fn send_money(
    &mut self,
    amount: u64,
    mut from_wallet: Wallet,
    to_wallet: Wallet,
  ) {
    println!("🧾 creating transaction for {} ", amount);

    let signature =
      from_wallet.create_signature(amount, to_wallet.clone());

    let transaction = Transaction::create(
      amount,
      from_wallet.public_key,
      to_wallet.public_key,
      signature,
    );

    let latest_block = self.get_latest_block();

    let transaction_block =
      Block::create(latest_block.hash.clone(), transaction);

    self.add_block_to_chain(transaction_block.clone());
  }

  pub fn is_block_valid(&self, block: Block) -> bool {
    let latest_block = self.get_latest_block();

    if block.previous_hash != latest_block.hash {
      log::warn!(
        "block with hash {} has invalid previous_hash",
        block.hash
      );

      false;
    }

    true
  }

  pub fn get_latest_block(&self) -> Block {
    let last_block = self.blocks.last().expect("chain is invalid");
    last_block.clone()
  }
}
