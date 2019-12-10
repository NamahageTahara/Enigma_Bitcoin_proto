extern crate secp256k1;
extern crate bitcoin;
extern crate hex;

use bitcoin::network::constants::Network;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
use secp256k1::Secp256k1;

fn main() {

    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let seed = hex::decode("000102030405060a08090a0b0c0d0e0f").unwrap();
    let sk = ExtendedPrivKey::new_master(network, &seed).unwrap();
    let pk = ExtendedPubKey::from_private(&secp, &sk);
    let cn_privkey = sk.ckd_priv(&secp, ChildNumber::from_normal_idx(1).unwrap()).unwrap();
    let ch_privkey = sk.ckd_priv(&secp, ChildNumber::from_hardened_idx(1).unwrap()).unwrap();
    let cn_pubkey = ExtendedPubKey::from_private(&secp, &cn_privkey);
    let ch_pubkey = ExtendedPubKey::from_private(&secp, &ch_privkey);

    println!("m_priv: {:}", sk);
    println!("m_pub: {:}", pk);
    println!("cn_priv: {:}", cn_privkey);
    println!("cn_pub: {:}", cn_pubkey);
    println!("ch_priv: {:}", ch_privkey);
    println!("ch_pub: {:}", ch_pubkey);

}