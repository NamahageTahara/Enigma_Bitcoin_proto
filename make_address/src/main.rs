extern crate secp256k1;
extern crate bitcoin;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use secp256k1::Secp256k1;
use secp256k1::rand::thread_rng;

fn main() {
     // Generate random key pair
    let s = Secp256k1::new();
    let keys = s.generate_keypair(&mut thread_rng());
    let public_key = key::PublicKey {
        compressed: true,
        key: keys.1,
    };
    let private_key = key::PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key: keys.0,
    };

     // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Your Address is {:}", address );
    println!("Your PKey is {:}", private_key);
}
