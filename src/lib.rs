
use std::vec::Vec;

fn is_equal(a: Vec<u8>, b:Vec<u8>) -> bool {
    number_of_different_chars(a, b) == 0
}

#[inline(never)]
fn number_of_different_chars(a: Vec<u8>, b :Vec<u8>) -> i32 {
    if a.len() != b.len() {
        -1
    } else {
        let mut result: i32 = 0;
        for (&x, &y) in a.iter().zip(b.iter()) {
            let diff = x ^ y;
            result = result + ( diff as i32);
        }
        result
    }
}

#[test]
fn test_number_of_different_chars() {
    assert!(0 == number_of_different_chars(vec![1,2,3,4], vec![1,2,3,4]));
    assert!(1 == number_of_different_chars(vec![1,2,3,4], vec![1,2,3,5]));
    assert!( 0 == number_of_different_chars(vec![1], vec![1]));
    assert!(-1 == number_of_different_chars(vec![], vec![1]));
}


#[test]
fn test_is_equal() {
    assert!(is_equal(vec![1,2,3,4], vec![1,2,3,4]));
    assert!(!is_equal(vec![1,2,3,4], vec![1,2,3,]));
    assert!(is_equal(vec![1], vec![1]));
    assert!(!is_equal(vec![], vec![1]));
}
