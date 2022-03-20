mod block;
mod hypercore;
mod transaction;
mod wallet;

fn main() {
  let mut hypercore = hypercore::Hypercore::create();

  // Create wallets
  let satoshi = hypercore.add_wallet(String::from("Satoshi"));
  let alice = hypercore.add_wallet(String::from("Alice"));

  // Send
  hypercore.send_money(50, satoshi, alice);

  println!("Number of blocks: {}", hypercore.blocks.len());
}
