use std::str::FromStr;

use bdk::bitcoin::{bip32::ExtendedPrivKey, network::Magic, secp256k1::SecretKey, Network};
use bitcoin::bip32::Xpriv;
// use bitcoin::{bip32::Xpriv, p2p::Magic, secp256k1::SecretKey, Network};
use rand::rngs::OsRng;

pub struct BitcoinKeys {
  pub master_key: String,
  pub network: Magic
}

impl BitcoinKeys {
  pub fn new(secret_seed: Option<String>) -> BitcoinKeys {
    let network = Network::Testnet;

    let seed = match secret_seed {
        Some(secret) => SecretKey::from_str(&secret).unwrap(),
        _=>SecretKey::new(&mut OsRng)
    };

    let master_key = ExtendedPrivKey::new_master(network, &seed.secret_bytes()).unwrap();

    println!("your seed is {}", seed.display_secret());
    BitcoinKeys { master_key: master_key.to_string(), network: network.magic() }
  }
}