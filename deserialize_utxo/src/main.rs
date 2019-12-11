extern crate bitcoin;
extern crate hex;

use bitcoin::consensus::encode::serialize;
use bitcoin::consensus::encode::deserialize;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::blockdata::transaction::TxIn;
use bitcoin::blockdata::transaction::TxOut;
use hex::FromHex;

const RBF: u32 = 0xffffffff - 2;

fn main() {
    // UTXOのHEX形式からデコード
    let hex_tx = Vec::<u8>::from_hex(
        "02000000000101beccab33bc72bfc81b63fdec8a4a9a4719e4418bdb7b20e47b0\
         2074dc42f2d800000000017160014f3b1b3819c1290cd5d675c1319dc7d9d98d5\
         71bcfeffffff02dceffa0200000000160014368c6b7c38f0ff0839bf78d77544d\
         a96cb685bf28096980000000000160014284175e336fa10865fb4d1351c9e18e7\
         30f5d6f90247304402207c893c85d75e2230dde04f5a1e2c83c4f0b7d93213372\
         746eb2227b068260d840220705484b6ec70a8fc0d1f80c3a98079602595351b7a\
         9bca7caddb9a6adb0a3440012103150514f05f3e3f40c7b404b16f8a09c2c71ba\
         d3ba8da5dd1e411a7069cc080a004b91300",
    ).unwrap();
    let tx: Transaction = deserialize(&hex_tx).unwrap();
    println!("transaction: {:}", tx.txid());
}
