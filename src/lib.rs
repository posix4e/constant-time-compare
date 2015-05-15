
use std::vec::Vec;

fn is_equal(a: Vec<u8>, b :Vec<u8>) -> bool {
    if a.len() != b.len() {
        false
    } else {
        let mut result = 0;
        for (&x, &y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}

#[test]
fn test_is_equal() {
    assert!(is_equal(vec![1,2,3,4], vec![1,2,3,4]));
    assert!(!is_equal(vec![1,2,3,4], vec![1,2,3,]));
    assert!(is_equal(vec![1], vec![1]));
    assert!(!is_equal(vec![], vec![1]));
}
