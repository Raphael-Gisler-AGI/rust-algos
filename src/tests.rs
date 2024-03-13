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

#[test]
fn maze_solver_test() {
    let matrix = Matrix {
        rows: [
            [1,1,1,0,0],
            [0,0,1,1,0],
            [0,2,0,1,0],
            [0,1,1,1,0],
            [0,0,0,0,0],
        ]
    };
    let path: Vec<Point> = maze_solver(matrix);
    let result_path: Vec<Point> = vec![Point(0,0), Point(0,1), Point(0,2), Point(1,2), Point(1,3), Point(2,3), Point(3,3), Point(3,2), Point(3,1), Point(2,1)];

    assert_eq!(path, result_path);
}
