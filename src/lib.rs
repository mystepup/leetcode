mod array;
use array::Array;

#[test]
fn find_min_test() {
    let result = Array::find_min(vec![1, 2]);

    assert_eq!(result, 1);
}

#[test]
fn search_test() {
    let result = Array::search(vec![4,5,6,7,0,1,2], 0);

    assert_eq!(result, 4);
}