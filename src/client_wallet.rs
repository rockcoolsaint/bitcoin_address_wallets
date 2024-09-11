use std::str::FromStr;

use bdk::{bitcoin::{bip32::ExtendedPrivKey, Address, Network}, blockchain::{Blockchain, ElectrumBlockchain}, database::MemoryDatabase, electrum_client::Client, template::Bip84, wallet::AddressIndex, FeeRate, KeychainKind, SignOptions, SyncOptions, Wallet};
// use bitcoin::bip32::Xpriv;

use crate::bitcoin_keys;

pub struct WalletContext {
  wallet_state: Wallet<MemoryDatabase>,
  blockchain: ElectrumBlockchain
}

impl WalletContext {
  pub fn new(seed: Option<String>) -> WalletContext {
    let key = bitcoin_keys::BitcoinKeys::new(seed.to_owned());

    let master_key = ExtendedPrivKey::from_str(&key.master_key).unwrap();

    let network = Network::from_magic(key.network).unwrap();
    
    let descriptor = Bip84(master_key, KeychainKind::External);

    let wallet_state = Wallet::new(
      descriptor, None, network, MemoryDatabase::default()
    ).unwrap();

    let blockchain = ElectrumBlockchain::from(
      Client::new("ssl://electrum.blockstream.info:60002").unwrap()
    );
    
    WalletContext { wallet_state, blockchain }
  }

  pub fn get_balance(&self) {
    self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();
    let receive_address = self.wallet_state.get_address(AddressIndex::LastUnused).unwrap();
    let balance = self.wallet_state.get_balance().unwrap();
    println!("bitcoin address is {}", receive_address.address);
    println!("the balance is {}", balance);
  }

  pub fn send_coins(&self, send_address: &str, sats: u64) {
    self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();
    let address = Address::from_str(send_address).unwrap();
    let mut builder = self.wallet_state.build_tx();
    builder.drain_wallet().fee_rate(FeeRate::from_sat_per_vb(2.0))
    .drain_to(address.payload.script_pubkey());
    // .drain_wallet();
    // .add_recipient(address.payload.script_pubkey(), sats);
    let (mut psbt, details) = builder.finish().unwrap();

    let is_valid = self.wallet_state.sign(&mut psbt, SignOptions::default())
    .unwrap();

  println!("is transaction valid {}", is_valid);
  let tx = psbt.clone().extract_tx();
  println!("transaction id. {}", &tx.txid());
  self.blockchain.broadcast(&tx).unwrap();
  println!("broadcasted successfully!");
  }
}