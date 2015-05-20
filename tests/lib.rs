extern crate constant_time_compare;
use constant_time_compare::number_mash;
use constant_time_compare::is_equal;

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
