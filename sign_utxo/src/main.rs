extern crate secp256k1;
extern crate bitcoin;
extern crate bitcoin_hashes;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use bitcoin::util::bip143;
use secp256k1::Secp256k1;
use secp256k1::rand::thread_rng;
use secp256k1::{Message};
use bitcoin::consensus::encode::serialize;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::blockdata::transaction::TxIn;
use bitcoin::blockdata::transaction::TxOut;
use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::blockdata::transaction::SigHashType;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey, ExtendedPubKey};
use hex::encode;
use bitcoin_hashes::sha256d;



const RBF: u32 = 0xffffffff - 2;

fn main() {
    //自分のアドレス設定
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let seed = hex::decode("000102030405060a08090a0b0c0d0e0f").unwrap();
    let sk = ExtendedPrivKey::new_master(network, &seed).unwrap();
    //let pk = ExtendedPubKey::from_private(&secp, &sk);

    let ch_privkey = sk.ckd_priv(&secp, ChildNumber::from_hardened_idx(1).unwrap()).unwrap();
    let ch_pubkey = ExtendedPubKey::from_private(&secp, &ch_privkey);

    let s = Secp256k1::new();
    let mut rng = thread_rng();
    //let keys = s.generate_keypair(&mut rng);
    let public_key = ch_pubkey.public_key;
    let private_key = ch_privkey.private_key;
    let address = Address::p2wpkh(&public_key, Network::Bitcoin);
    let script_pubkey = address.script_pubkey();
    println!("my script pubkey: {:}", script_pubkey);

    //送り先のアドレス設定
    let target_keys = s.generate_keypair(&mut rng);
    let target_public_key = key::PublicKey {
        compressed: true,
        key: target_keys.1,
    };
    let target_address = Address::p2wpkh(&target_public_key, Network::Bitcoin);
    let target_script_pubkey = target_address.script_pubkey();

    let _target_address_p2pkh = Address::p2pkh(&target_public_key, Network::Bitcoin);
    let target_script_pubkey_p2wpkh = target_address.script_pubkey();
    println!("target script pubkey(in p2wpkh): {:}", target_script_pubkey);

    println!("target script pubkey(in p2pkh): {:}", target_script_pubkey_p2wpkh);

    //空のinputトランザクション生成
    let input_transaction = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: sha256d::Hash::default(),
                vout: 0,
            },
            sequence: RBF,
            witness: Vec::new(),
            script_sig: Script::new(),
        }],
        output: vec![TxOut {
            script_pubkey: script_pubkey.clone(),
            value: 5000000000,
        }],
        lock_time: 0,
        version: 2,
    };
    let txid = input_transaction.txid();
    //空のoutputトランザクション生成
    let mut spending_transaction = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint { txid, vout: 0 },
            sequence: RBF,
            witness: Vec::new(),
            script_sig: Script::new(),
        }],
        output: vec![TxOut {
            script_pubkey: target_script_pubkey,
            value: 5000000000,
        }],
        lock_time: 0,
        version: 2,
    };

    //サインする
    let txclone = spending_transaction.clone();
    let mut bip143hasher: Option<bip143::SighashComponents> = None;
    let hasher = bip143hasher.unwrap_or(bip143::SighashComponents::new(&txclone));
    //input.script_sig = Script::new();

    let sighash = hasher.sighash_all(
        &txclone.input[0],
        &address.script_pubkey(),
        5000000000,
    );
    bip143hasher = Some(hasher);
    let signature = s.sign(&Message::from_slice(&sighash[..]).unwrap(), &private_key.key);
    let serialized_signature = signature.serialize_der();
   
    let mut with_hashtype = serialized_signature.to_vec();
    with_hashtype.push(SigHashType::All.as_u32() as u8);
    spending_transaction.input[0].witness.clear();
    spending_transaction.input[0].witness.push(with_hashtype);
    spending_transaction.input[0].witness.push(address.script_pubkey().to_bytes());
    println!("witness: {:}", signature);
    
    //asserteq!
    //デコードしてreturn
    let serialized_transaction =  serialize(&spending_transaction);
    let encoded_serialized_transaction = encode(serialized_transaction);
    println!("\n Encoded transaction: {:}", encoded_serialized_transaction);
}
/*
fn sign(digest: &[u8], key: &PrivateKey) -> Result<Signature, &'str> {
    Ok(s.sign(&Message::from_slice(digest), &key.key))
}
*/