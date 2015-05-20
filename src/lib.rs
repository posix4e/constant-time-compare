
use std::vec::Vec;

fn is_equal(a: Vec<u8>, b: Vec<u8>) -> bool {
    number_mash(a, b) == 0
}

#[inline(never)] // This should be inlined to prevent another function from optimizing it to not
                 // be constant time.
fn number_mash(a: Vec<u8>, b: Vec<u8>) -> u8 {
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

#[test]
fn test_number_mash_works_as_expected() {
    assert!(0 == number_mash(vec![1,2,3,4], vec![1,2,3,4]));
    assert!(1 == number_mash(vec![1,2,3,4], vec![1,2,3,5]));
    assert!(250 == number_mash(vec![1,2,3,4], vec![1,2,3,254]));
    assert!( 0 == number_mash(vec![1], vec![1]));
}

#[test]
#[should_panic]
fn number_mash_will_panic_for_different_sized_arrays() {
    number_mash(vec![1,2,3], vec![1,2,3,5]);
}


#[test]
fn test_is_equal() {
    assert!(is_equal(vec![1,2,3,4], vec![1,2,3,4]));
    assert!(is_equal(vec![1], vec![1]));

}

#[test]
#[should_panic]
fn eq_will_panic_for_different_sized_arrays() {
    is_equal(vec![1,2,3,4], vec![1,2,3,]);
}

#[test]
#[should_panic]
fn eq_with_null_veq_will_panic() {
    is_equal(vec![], vec![1]);
}
