mod block;
mod hypercore;
mod transaction;
mod wallet;

fn main() {
  let mut hypercore = hypercore::Hypercore::init();

  // Create three wallets and do some transactions
  let satoshi_slug = String::from("Satoshi");
  let satoshi = hypercore.add_wallet(satoshi_slug);

  let alice_slug = String::from("Alice");
  let alice = hypercore.add_wallet(alice_slug);

  satoshi.send_money(&50, &alice.public_key);
  // bob.send_money(23, &alice.public_key);
  // alice.send_money(5, &bob.public_key);

  // Output latest block
  let genesis_block = &hypercore.blocks.first().expect("there exists a first core");
  println!("Preview of genesis block: {}", &genesis_block);
}
