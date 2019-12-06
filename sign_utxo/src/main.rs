use bitcoin::{
    blockdata::opcodes::all::OP_RETURN,
    blockdata::script::{Builder, Script},
    blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut},
    network::constants::Network
};
use btc_transaction_utils::{
    p2wpk,
    test_data::{secp_gen_keypair_with_rng, btc_tx_from_hex},
    TxInRef
};
use rand::prelude::*;

fn main() {
    // Take a transaction with the unspent P2WPK output.
    let prev_tx = btc_tx_from_hex(
        "02000000000101beccab33bc72bfc81b63fdec8a4a9a4719e4418bdb7b20e47b0\
         2074dc42f2d800000000017160014f3b1b3819c1290cd5d675c1319dc7d9d98d5\
         71bcfeffffff02dceffa0200000000160014368c6b7c38f0ff0839bf78d77544d\
         a96cb685bf28096980000000000160014284175e336fa10865fb4d1351c9e18e7\
         30f5d6f90247304402207c893c85d75e2230dde04f5a1e2c83c4f0b7d93213372\
         746eb2227b068260d840220705484b6ec70a8fc0d1f80c3a98079602595351b7a\
         9bca7caddb9a6adb0a3440012103150514f05f3e3f40c7b404b16f8a09c2c71ba\
         d3ba8da5dd1e411a7069cc080a004b91300",
    );
    // Take the corresponding key pair.
    let mut rng = thread_rng();
    let keypair = secp_gen_keypair_with_rng(&mut rng, Network::Testnet);
    // Create an unsigned transaction
    let mut transaction = Transaction {
        version: 2,
        lock_time: 0,
        input: vec![
            TxIn {
                previous_output: OutPoint {
                    txid: prev_tx.txid(),
                    vout: 1,
                },
                script_sig: Script::default(),
                sequence: 0xFFFFFFFF,
                witness: Vec::default(),
            },
        ],
        output: vec![
            TxOut {
                value: 0,
                script_pubkey: Builder::new()
                    .push_opcode(OP_RETURN)
                    .push_slice(b"Hello Exonum!")
                    .into_script(),
            },
        ],
    };
    // Create a signature for the given input.
    let mut signer = p2wpk::InputSigner::new(keypair.0, Network::Testnet);
    let signature = signer
        .sign_input(TxInRef::new(&transaction, 0), &prev_tx, &keypair.1.key)
        .unwrap();
    // Finalize the transaction.
    signer.spend_input(&mut transaction.input[0], signature);
}