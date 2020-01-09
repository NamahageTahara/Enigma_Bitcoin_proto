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
        "0200000000010116dc3a4ea3dbcf9b485052e55a6c79a85368a3938c72f52bbe43e2bbda1337f601000000171600142e6daba1bdd67c3d1a9c4eb0f031e8bfc2db5a66feffffff02c90b13000000000017a91493c11cbe0434cf71824412341807c3daef021e8387204e0000000000001976a914dee455afe7311fd31c707c0affb816a31549b68188ac02473044022014273b9df32e2de687fd853dd3aa0c4c807103a6f9f5f67d02c3e2c2c157be5c02202b30c80a8e20248eb60ed961b82d2cc31c1b70641c644aecf4728703113b27a2012103f374f8a5b389e332114c2071cb7674a7d51e86b51feefb4c9badcb4b9e99922017ed1800"
    ).unwrap();
    let tx: Transaction = deserialize(&hex_tx).unwrap();
    println!("transaction: {:}", tx.txid());
}
