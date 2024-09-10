use std::str::FromStr;

use bdk::{bitcoin::{bip32::ExtendedPrivKey, Network}, blockchain::ElectrumBlockchain, database::MemoryDatabase, electrum_client::Client, template::Bip84, KeychainKind, Wallet};
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
}