// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // this will borrow references of variable A 
    // and take the range of values between 1 of 4.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
