extern crate secp256k1;
extern crate bitcoin;

use std::str::FromStr;
use hex::{encode, FromHex};
use secp256k1::rand::thread_rng;
use secp256k1::{Message, Secp256k1};
use bitcoin::network::constants::Network;
use bitcoin::consensus::encode::{serialize, deserialize};
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint, SigHashType};
use bitcoin::blockdata::script::{Script, Builder};
use bitcoin::util::key;
use bitcoin::util::address::Address;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};

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
    
    let derive_path = DerivationPath::from_str(&path).unwrap();
    //private, pub, address, script_pubkey
    let privkey = sk.derive_priv(&secp, &derive_path).unwrap();
    let pubkey = ExtendedPubKey::from_private(&secp, &privkey);
    let address = Address::p2pkh(&pubkey.public_key, Network::Testnet);
    let script_pubkey = address.script_pubkey();
    println!("my address: {:}", address);

    //送り先のアドレス設定
    let target_keys = secp.generate_keypair(&mut rng);
    let target_public_key = key::PublicKey {
        compressed: true,
        key: target_keys.1,
    };
    let target_address = Address::p2pkh(&target_public_key, Network::Bitcoin);
    let target_script_pubkey = target_address.script_pubkey();

    
    //空のinputトランザクション生成　→　txidを導出
    let input_transactions = vec!["0200000001b7cce8f54406d0f14a25093bb1fe1ab1825b9b6ed253de86acdb1e167f657b55000000006b483045022100ab97b12063e818b03fdf42bb4f53e8264266919dbd4bf2b8b0c2f8302acc7b9e02202ae0de7a0d4a12c71a2e178f47d6f081d7064435736607966f71c6dc2309666b0121025f33712c8b1699d9c1b885644abfc6c0effa5b28f132b450b90d1c3c571c6413fdffffff0100f2052a010000001976a914edee46042ee9da761e70132da904356b93a66b2288ac00000000", 
    "0200000001b7cce8f54406d0f14a25093bb1fe1ab1825b9b6ed253de86acdb1e167f657b55000000006b483045022100ab97b12063e818b03fdf42bb4f53e8264266919dbd4bf2b8b0c2f8302acc7b9e02202ae0de7a0d4a12c71a2e178f47d6f081d7064435736607966f71c6dc2309666b0121025f33712c8b1699d9c1b885644abfc6c0effa5b28f132b450b90d1c3c571c6413fdffffff0100f2052a010000001976a914edee46042ee9da761e70132da904356b93a66b2288ac00000000",];

    let vouts = vec![0,0];
    let sending_value = 5000000000;
    let mut value = 0;
    //空のoutputトランザクション生成
    let mut spending_transaction = Transaction {
        input: vec![],
        output: vec![TxOut {
            script_pubkey: target_script_pubkey,
            value: sending_value,
        },
        TxOut {
            script_pubkey: script_pubkey,
            value: 0,
        }],
        lock_time: 0,
        version: 2,
    };


    for (i, transaction) in input_transactions.iter().enumerate(){
    //サインする
        let hex_tx = Vec::<u8>::from_hex(transaction).expect("panic in from_hex");
        let txclone: Transaction = deserialize(&hex_tx).expect("panic in deserialize");
        if txclone.output[vouts[i]].value + value > sending_value + 300 {
            break;
        }
        let mut input = TxIn {
            previous_output: OutPoint { txid: txclone.txid(), vout: vouts[i] as u32},
            sequence: RBF,
            witness: Vec::new(),
            script_sig: Script::new(),
        };
        let sighash = txclone.signature_hash(
            0,
            &address.script_pubkey(),
            SigHashType::All.as_u32(),
        );
        let signature = secp.sign(&Message::from_slice(&sighash[..]).unwrap(), &privkey.private_key.key);
        let serialized_signature = signature.serialize_der();
   
        let mut with_hashtype = serialized_signature.to_vec();
        with_hashtype.push(SigHashType::All.as_u32() as u8);
        input.witness.clear();
        input.script_sig = Builder::new()
        .push_slice(with_hashtype.as_slice())
        .push_slice(pubkey.public_key.to_bytes().as_slice())
        .into_script();
        println!("witness: {:}", signature);
        spending_transaction.input.push(input);
        //asserteq!
        //デコードしてreturn
        value += txclone.output[vouts[i]].value;
    }
    let serialized_transaction =  serialize(&spending_transaction);
    let encoded_serialized_transaction = encode(serialized_transaction);
    println!("\n Encoded transaction: {:}", encoded_serialized_transaction);

}
