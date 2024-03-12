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
fn stalin_sort_test() {
    let data: &mut Vec<i32> = &mut [1,5,-1,2,-100,2,3,4,5,6,-200].to_vec();
    stalin_sort(data);
    let result: &mut Vec<i32> = &mut [1,5,5,6].to_vec();
    assert_eq!(data, result);
}

#[test]
fn binary_search_test() {
    assert_eq!(binary_search(&SORTED_DATA, 2), 1);
}

#[test]
fn linear_search_test() {
    assert_eq!(linear_search(&UNSORTED_DATA, 345), 2);
}


