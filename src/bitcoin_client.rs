use bitcoin::{address, key::{Keypair, Secp256k1}, CompressedPublicKey, Network, XOnlyPublicKey};
// use bitcoin::{address, secp256k1::{Keypair, Secp256k1}, CompressedPublicKey, Network, XOnlyPublicKey};
use rand::{self, thread_rng};

// master node
// purpose node = p2pk/p2wpkh/p2sh/p2tr
// cointype = testnet/mainet/regtest
pub fn generate_wallet() {
  // Generator curve function
  // G(x,y) = y^2=x^3+b mode s
  // public key = G(x,y)*private key
  let elliptic_curve_fun = Secp256k1::new();
  let network = Network::Testnet;

  let (secret_key, public_key) = &elliptic_curve_fun.generate_keypair(&mut thread_rng()); // generate a random point

  let key_pair = Keypair::from_secret_key(&elliptic_curve_fun, &secret_key); // derived a keypair from the schnorr algorithm

  let x_only_public_key = XOnlyPublicKey::from_keypair(&key_pair); //verify the schnorr signature and serialize our public key

  let tap_address = address::Address::p2tr(&elliptic_curve_fun, x_only_public_key.0, None, network);

  let p2wpkh_address = address::Address::p2wpkh(&CompressedPublicKey(*public_key), network); // derives a p2wpkh (pay to witness public key hash) public address

  println!("pay to witness public key hash {}", p2wpkh_address.to_qr_uri().to_ascii_lowercase());

  println!("pay to taproot address {}", tap_address.to_qr_uri().to_ascii_lowercase());
}