
#[cfg(test)]
mod tests;

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

