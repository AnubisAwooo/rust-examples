use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::pb::{Block, BlockHash};

// const PREFIX_ZERO: &[u8] = &[0, 0];

// pub fn do_pow(block: Block) -> Option<BlockHash> {
//     let hasher = blake3_hash_data(&block.data);
//     let difficult = block.difficult as usize;
//     let zeros = vec![0; difficult];
//     let hash = (0..u32::MAX)
//         .filter_map(|n| {
//             let hash = blake3_hash(hasher.clone(), n);
//             if &hash[..(block.difficult as usize)] == zeros {
//                 Some((n, hash))
//             } else {
//                 None
//             }
//         })
//         .find(|_| true);
//     hash.and_then(|(nonce, hash)| {
//         Some(BlockHash {
//             data: block.data,
//             nonce,
//             hash,
//         })
//     })
// }

pub fn do_pow(block: Block) -> Option<BlockHash> {
    let hasher = blake3_hash_data(&block.data);
    let difficult = block.difficult as usize;
    let zeros = vec![0; difficult];

    // use rayon::prelude::*;
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), *n);
        &hash[..(block.difficult as usize)] == zeros
    });
    println!("got nonce and hash: {:?}", nonce);
    nonce.and_then(|nonce| {
        Some(BlockHash {
            data: block.data,
            nonce,
            hash: blake3_hash(hasher.clone(), nonce),
        })
    })
}

// fn get_block_id(block: &Block) -> String {
//     format!("{:?}", block)
// }

fn blake3_hash(mut hasher: blake3::Hasher, nonce: u32) -> Vec<u8> {
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().as_bytes().to_vec()
}

fn blake3_hash_data(data: &[u8]) -> blake3::Hasher {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    hasher
}
