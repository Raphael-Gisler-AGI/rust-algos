use super::*;

const UNSORTED_DATA: [i32;7] = [5,1,345,1000,200,5342,2];
const SORTED_DATA: [i32;7] = [1,2,5,200,345,1000,5342];

#[test]
fn quick_sort_test() {
    let mut data = UNSORTED_DATA;
    quick_sort(&mut data);
    assert_eq!(data, SORTED_DATA);
}

#[test]
fn bubble_sort_test() {
    let mut data = UNSORTED_DATA;
    bubble_sort(&mut data);
    assert_eq!(data, SORTED_DATA);
}

#[test]
fn binary_search_test() {
    assert_eq!(binary_search(&SORTED_DATA, 2), 1);
}

#[test]
fn linear_search_test() {
    assert_eq!(linear_search(&SORTED_DATA, 2), 1);
}




