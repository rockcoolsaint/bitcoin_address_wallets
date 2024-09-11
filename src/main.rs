// pub mod bitcoin_client;
pub mod bitcoin_keys;
pub mod client_wallet;

fn main() {
    // bitcoin_client::generate_wallet();
    // println!("Hello, world!");
    let from = "15ef485350c14a2e63fdd41057fb723c067e8200ad34d1bf70576d69571685b0";
    let wallet = client_wallet::WalletContext::new(Some(from.to_string()));
    wallet.get_balance();

    let to = "tb1qlj64u6fqutr0xue85kl55fx0gt4m4urun25p7q";
    wallet.send_coins(to, 100);
}

// tb1qha7taytjz3mz2g8yg0vvyygu52uvgxrl0wf6ln