extern crate secp256k1;
extern crate bitcoin;

use std::str::FromStr;
use serde::{Deserialize,Serialize};
use secp256k1::key::PublicKey;
use hex::{encode, FromHex};
use secp256k1::{Message, Secp256k1};
use bitcoin::network::constants::Network;
use bitcoin::consensus::encode::{serialize, deserialize};
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint, SigHashType};
use bitcoin::blockdata::script::{Script, Builder};
use bitcoin::util::key::{PrivateKey};
use bitcoin::util::address::Address;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};

const RBF: u32 = 0xffffffff - 1;

fn main() {
    //設定
    let secp = Secp256k1::new();
    let network = Network::Testnet;
    let mut address_len = 100;
    //seedからmasterkey、path生成してchild keyとscript_pubkey
    let seed = hex::decode("000102030405060a08090a0b0c0d0e0f").unwrap();
    let sk = ExtendedPrivKey::new_master(network, &seed).unwrap();
    
    //自分のアドレス
    let derive_path1 = get_path(address_len);
    let privkey = sk.derive_priv(&secp, &derive_path1).unwrap();
    let pubkey = ExtendedPubKey::from_private(&secp, &privkey);
    let address = Address::p2pkh(&pubkey.public_key, Network::Testnet);
    let script_pubkey = address.script_pubkey();
    address_len += 1;

    let derive_path2 = get_path(address_len);
    let privkey2 = sk.derive_priv(&secp, &derive_path1).unwrap();
    let pubkey2 = ExtendedPubKey::from_private(&secp, &privkey);
    let address2 = Address::p2pkh(&pubkey.public_key, Network::Testnet);
    let script_pubkey2 = address.script_pubkey();

    //送り先のアドレス設定
    let derive_path2 = get_path(address_len);
    let target_privkey = sk.derive_priv(&secp, &derive_path2).unwrap();
    let target_pubkey = ExtendedPubKey::from_private(&secp, &target_privkey);
    let target_address = Address::p2pkh(&target_pubkey.public_key, Network::Testnet);
    let target_script_pubkey = target_address.script_pubkey();
    println!("target address is {:} \n", target_address);

    
    //空のinputトランザクション生成　→　txidを導出
    let input_transactions = vec!["0200000000010116dc3a4ea3dbcf9b485052e55a6c79a85368a3938c72f52bbe43e2bbda1337f601000000171600142e6daba1bdd67c3d1a9c4eb0f031e8bfc2db5a66feffffff02c90b13000000000017a91493c11cbe0434cf71824412341807c3daef021e8387204e0000000000001976a914dee455afe7311fd31c707c0affb816a31549b68188ac02473044022014273b9df32e2de687fd853dd3aa0c4c807103a6f9f5f67d02c3e2c2c157be5c02202b30c80a8e20248eb60ed961b82d2cc31c1b70641c644aecf4728703113b27a2012103f374f8a5b389e332114c2071cb7674a7d51e86b51feefb4c9badcb4b9e99922017ed1800", 
    "02000000000101a2270bb4e84499cef1532427c71f2b6a2733f4b682e1a5a207fb7f80758e43c9000000001716001444c7cc536aa72081b1a0394acfb72aedd7f4a126feffffff027bd52b000000000017a914836ae7f53728bb57543e1fe98e5df61f613779c28710270000000000001976a914dee455afe7311fd31c707c0affb816a31549b68188ac0247304402207982a1550676379c1a7e67b54d261644dc2518094ea5a54aff3a059fdaa1465502207b132e0dff99d0d76f258b3f74d337241b378ed807ab2611ad986daaf34677e40121021ac9d7bf706d92342fd44bb5dac45b5901bab79a2400730c664628e4e2aaa07fdbee1800",];
    let addresses = vec![address.clone(), address.clone()];
    let secret_keys = vec![privkey.private_key.clone(), privkey.private_key];
    let script_pubkeys = vec![script_pubkey.clone(), script_pubkey];
    let pubkeys = vec![pubkey.public_key.key.clone(), pubkey.public_key.key];

    let vouts = vec![1,1];
    let sending_value = 27000;
    let mut value = 0;
    //空のoutputトランザクション生成
    let mut spending_transaction = Transaction {
        input: vec![],
        output: vec![TxOut {
            script_pubkey: target_script_pubkey,
            value: sending_value,
        },
        TxOut {
            script_pubkey: address.script_pubkey(),
            value: 0,
        }],
        lock_time: 0,
        version: 2,
    };


    for (i, transaction) in input_transactions.iter().enumerate(){
    //サインする
        let hex_tx = Vec::<u8>::from_hex(transaction).expect("panic in from_hex");
        let tx: Transaction = deserialize(&hex_tx).expect("panic in deserialize");
        let v = tx.output[vouts[i]].value;

        if value <= sending_value + 700 {
            spending_transaction.input.push(create_input(tx, vouts[i] as u32));
            value = value + v;
        }else{break;}
    }
    
    spending_transaction.output[1].value = value - sending_value - 700;
    let tx = sign(secp, spending_transaction, &script_pubkeys, &secret_keys, pubkeys);
    let serialized_transaction =  serialize(&tx);
    let encoded_serialized_transaction = encode(serialized_transaction);
    println!("Signed Transaction is {:}", encoded_serialized_transaction);

}

fn get_path(mut address_len: u64) -> DerivationPath {
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
    
    DerivationPath::from_str(&path).unwrap()
    //private, pub, address, script_pubkey
}

fn create_input(tx: Transaction, vout: u32) -> TxIn {
    TxIn {
        previous_output: OutPoint { txid: tx.txid(), vout: vout},
        sequence: RBF,
        witness: Vec::new(),
        script_sig: Script::new(),
    }
}

fn sign(secp: Secp256k1<secp256k1::All>, mut tx: Transaction, script_pubkey: &Vec<Script>, privkey: &Vec<PrivateKey>, pubkey: Vec<PublicKey>) -> Transaction {
    let txclone = tx.clone();
    for (ix, input) in tx.input.iter_mut().enumerate(){
        let sighash = txclone.signature_hash(
            ix,
            &script_pubkey[ix],
            SigHashType::All.as_u32(),
        );
        println!("sig hash: {:?}", sighash);
        let serialized_signature = secp.sign(&Message::from_slice(&sighash[..]).unwrap(), &privkey[ix].key).serialize_der();

        let mut with_hashtype = serialized_signature.to_vec();
        with_hashtype.push(SigHashType::All.as_u32() as u8);
        input.witness.clear();
        println!("with hash: {:?}", privkey[ix].key);
        let script_sig = Builder::new()
                            .push_slice(with_hashtype.as_slice())
                            .push_slice(serialize_pubkey(pubkey[ix]).as_slice())
                            .into_script();
        println!("scriptsig1: {:?}", script_sig);
        input.script_sig = script_sig;                   
                
    }

    return tx;
}

fn serialize_pubkey(key: PublicKey) -> Vec<u8>{
    key.serialize().to_vec()
}
