
use std::vec::Vec;

pub fn is_equal(a: Vec<u8>, b: Vec<u8>) -> bool {
    number_mash(a, b) == 0
}

#[inline(never)] // This should be inlined to prevent another function from optimizing it to not
             // be constant time.
pub fn number_mash(a: Vec<u8>, b: Vec<u8>) -> u8 {
    if a.len() != b.len() {
            panic!("Number Mash can only compare equal sized arrays");
        } else {
            let mut result = 0;
            for (&x, &y) in a.iter().zip(b.iter()) {
                result |= x ^ y;
                }
            result
        }
}
