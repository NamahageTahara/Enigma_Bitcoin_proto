extern crate secp256k1;
extern crate bitcoin;
extern crate bitcoin_hashes;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use secp256k1::Secp256k1;
use secp256k1::rand::thread_rng;
use secp256k1::{Message};
use bitcoin::consensus::encode::serialize;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::blockdata::transaction::TxIn;
use bitcoin::blockdata::transaction::TxOut;
use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::script::Builder;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::blockdata::transaction::SigHashType;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use hex::encode;
use bitcoin_hashes::sha256d;

use std::str::FromStr;



const RBF: u32 = 0xffffffff - 2;

fn main() {
    //設定
    let secp = Secp256k1::new();
    let network = Network::Testnet;
    let mut rng = thread_rng();
    //seedからmasterkey、path生成してchild keyとscript_pubkey
    let seed = hex::decode("000102030405060a08090a0b0c0d0e0f").unwrap();
    let sk = ExtendedPrivKey::new_master(network, &seed).unwrap();

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
    //private, pub, address, script_publkey
    let privkey = sk.derive_priv(&secp, &derive_path).unwrap();
    let pubkey = ExtendedPubKey::from_private(&secp, &privkey);
    let address = Address::p2pkh(&pubkey.public_key, Network::Testnet);
    let script_pubkey = address.script_pubkey();
    println!("my script pubkey: {:}", script_pubkey);

    //送り先のアドレス設定
    let target_keys = secp.generate_keypair(&mut rng);
    let target_public_key = key::PublicKey {
        compressed: true,
        key: target_keys.1,
    };
    let target_address = Address::p2pkh(&target_public_key, Network::Bitcoin);
    let target_script_pubkey = target_address.script_pubkey();

    println!("target script pubkey(in p2pkh): {:}", target_script_pubkey);

    //空のinputトランザクション生成　→　txidを導出
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

    let sighash = txclone.signature_hash(
        0,
        &address.script_pubkey(),
        SigHashType::All.as_u32(),
    );
    let signature = secp.sign(&Message::from_slice(&sighash[..]).unwrap(), &privkey.private_key.key);
    let serialized_signature = signature.serialize_der();
   
    let mut with_hashtype = serialized_signature.to_vec();
    with_hashtype.push(SigHashType::All.as_u32() as u8);
    spending_transaction.input[0].witness.clear();
    spending_transaction.input[0].script_sig = Builder::new()
    .push_slice(with_hashtype.as_slice())
    .push_slice(pubkey.public_key.to_bytes().as_slice())
    .into_script();
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
