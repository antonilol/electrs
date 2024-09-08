use std::fs;

use bitcoin::{hashes::Hash, BlockHash};
use bitcoin_slices::sha2::{Digest, Sha256};
use electrs::{
    db::{DBStore, WriteBatch},
    index::index_single_block,
};

fn main() {
    let mut batch = WriteBatch::default();

    for height in 180001..=182000 {
        let block = fs::read(format!("blocks/blk-{height}.dat")).unwrap();
        let hash = Sha256::digest(Sha256::digest(&block[0..80])).into();
        index_single_block(BlockHash::from_byte_array(hash), block, height, &mut batch);
        if height % 100 == 0 {
            dbg!(height);
        }
    }

    dbg!(batch.stats());

    println!("Without sorting");

    let store1 = DBStore::open("/tmp/db1".as_ref(), None, true).unwrap();
    store1.write(&batch);

    println!("With sorting");

    batch.sort();

    let store2 = DBStore::open("/tmp/db2".as_ref(), None, true).unwrap();
    store2.write(&batch);
}
