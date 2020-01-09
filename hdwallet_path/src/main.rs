extern crate secp256k1;
extern crate bitcoin;
extern crate hex;

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use secp256k1::Secp256k1;
use std::str::FromStr;
use bitcoin::util::address::Address;

fn main() {
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let seed = hex::decode("000102030405060a08090a0b0c0d0e0f").unwrap();
    let sk = ExtendedPrivKey::new_master(network, &seed).unwrap();
    let pk = ExtendedPubKey::from_private(&secp, &sk);

    let mut address_len = 100;
    let first = (address_len % 59).to_string();
    let second = (address_len % 73).to_string();
    let mut path = "m/".to_string() + &first + "'/" + &second;

    if address_len < 200000000 {
        path += &address_len.to_string();
    }else {
         while address_len > 200000000{
            path += &200000000.to_string();
            address_len -= 200000000;
        }
        path += &address_len.to_string();
    }
    println!("path: {:}", path);

    
    let derive_path = DerivationPath::from_str(&path).unwrap();

    let privkey = sk.derive_priv(&secp, &derive_path).unwrap();
    let pubkey = ExtendedPubKey::from_private(&secp, &privkey);
    
    let address = Address::p2pkh(&pubkey.public_key, Network::Testnet);
    
    println!("ch_priv: {:}", privkey);
    println!("ch_pub: {:}", pubkey);
    println!("address: {:}", address);
    
    
}
