extern crate tiny_keccak;

use tiny_keccak::Keccak;


pub struct Keccak256 {
    hash: Sha3,
}

impl Default for Keccak256 {
    fn default() -> Self {
        Keccak256 {
            hash: Sha3::keccak256(),
        }
    }
}

impl Digest for Keccak256 {
    fn input(&mut self, i: &[u8]) {
        self.hash.input(i);
    }

    fn result(&mut self, o: &mut [u8]) {
        self.hash.result(o);
    }

    fn reset(&mut self) {
        self.hash.reset();
    }

    fn output_bits(&self) -> usize {
        256
    }

    fn block_size(&self) -> usize {
        64
    }
}
