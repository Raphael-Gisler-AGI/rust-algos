
#[cfg(test)]
mod tests;

#[derive(Debug,PartialEq)]
struct Point (usize, usize);

struct Matrix {
    rows: [[u8;5]; 5]
}
impl Matrix {
    fn get(&self, point: &Point) -> Option<u8> {
        self.rows.get(point.0)?.get(point.1).copied()
    }
    fn set(&mut self, point: &Point, value: u8) {
        *self.rows.get_mut(point.0).expect("").get_mut(point.1).expect("") = value;
    }
}

fn main() {
    println!("Hello World!");
}

// QUICKSORT
fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, (arr.len() - 1) as i32)
}
fn qs(arr: &mut [i32], low: i32, high: i32) {
    if low >= high {
        return;
    }

    let pivot_idx: i32 = pivot(arr, low as usize, high as usize);

    qs(arr, pivot_idx + 1, high);
    qs(arr, low, pivot_idx - 1);
}
fn pivot(arr: &mut [i32], low: usize, high: usize) -> i32 {
    let pivot = arr[high];

    let mut idx: i32 = low as i32 - 1;

    for i in low..high {
        if pivot > arr[i] {
            idx += 1;

            let temp = arr[idx as usize];
            arr[idx as usize] = arr[i];
            arr[i] = temp;
        }
    }

    idx += 1;

    arr[high] = arr[idx as usize];
    arr[idx as usize] = pivot;

    idx
}


// BUBBLE SORT
fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}


// STALIN SORT
fn stalin_sort(arr: &mut Vec<i32>) {
    let mut idx: usize = 0;
    for _i in 0..arr.len() - 1 {
        idx += 1;
        if arr[idx - 1] > arr[idx] {
            arr.remove(idx);
            idx -= 1;
        }
    }
}


// BINARY SEARCH
fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut high: usize = arr.len();
    let mut low: usize = 0;

    while high > low {
        let mid: usize = low + (high - low) / 2;
        if arr[mid] == target {
            return mid as i32;
        }
        if arr[mid] > target {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    -1
}


// LINEAR SEARCH
fn linear_search(arr: &[i32], target: i32) -> i32 {
    for (i, item) in arr.iter().enumerate() {
        if item == &target {
            return i as i32;
        }
    }

    -1
}

// MAZE SOLVER
fn maze_solver(mut maze: Matrix) -> Vec<Point> {
    let mut path: Vec<Point> = vec![];

    walk(&mut maze, Point(0,0), &mut path);

    path
}
fn walk(maze: &mut Matrix, point: Point, path: &mut Vec<Point>) -> bool {
    if let Some(value) = maze.get(&point) {
        if value == 0 {
            return false;
        }

        path.push(Point(point.0, point.1));

        if value == 2 {
            return true;
        }
    } else {
        return false;
    }

    maze.set(&point, 0);

    if walk(maze, Point(point.0, point.1 + 1), path) {
        return true;
    }
    if walk(maze, Point(point.0, point.1 - 1), path) {
        return true;
    }
    if walk(maze, Point(point.0 + 1, point.1), path) {
        return true;
    }
    if walk(maze, Point(point.0 - 1, point.1), path) {
        return true;
    }

    false
}

