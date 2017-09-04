extern crate tiny_keccak;

use tiny_keccak::Keccak;

// Concat two byte arrays [u8; 32] produced by keccak256 hashing some data.
#[inline]
pub fn concat(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // create the result buffer
    let mut result = [0u8; 64];
    result[0..32].copy_from_slice(&a);
    result[32..64].copy_from_slice(&b);
    result
}

// Autohash
pub fn keccak_hash(input: Vec<u8>) -> [u8;32] {
    let mut sha3 = Keccak::new_sha3_256();
    let data: Vec<u8> = input.clone();
    sha3.update(&data);

    let mut res: [u8;32] = [0u8;32];
    sha3.finalize(&mut res);
    res
}

// Hash of a merkle node. This is the resultant node of two children blocks. 
pub fn merkle_node_hash(left: [u8;32], right: [u8;32]) -> [u8;32] {
    keccak_hash(concat(left, right).to_vec())
}

// Calculate the merkle root by hashing each data hash into a row,
// then hashing that row recursively to build the tree up to it's 
// root hash.
pub fn merkle_root(hashes: Vec<[u8;32]>) -> [u8;32] {
    if hashes.len() == 1 {
        return hashes[0];
    }

    let mut row = Vec::with_capacity(hashes.len() / 2);
    let mut i = 0;
    while i + 1 < hashes.len() {
        row.push(merkle_node_hash(hashes[i], hashes[i + 1]));
        i += 2;
    }

    // Duplicate the last element if the number of nodes is odd
    if hashes.len() % 2 == 1 {
        let last = hashes[hashes.len() - 1];
        row.push(merkle_node_hash(last, last));
    }

    merkle_root(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        /// TODO
    }    
}