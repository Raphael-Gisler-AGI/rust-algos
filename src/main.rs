
fn main() {
    let mut data: Vec<i32> = [1,-4,20,-1,500,4141,-3232].to_vec();
    println!("{:?}", data);
    quicksort(&mut data);
    println!("{:?}", data);
}

fn quicksort(arr: &mut Vec<i32>) {
    qs(arr, 0, (arr.len() - 1) as i32)
}
fn qs(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low >= high {
        return;
    }

    let pivot_idx: i32 = pivot(arr, low as usize, high as usize);

    qs(arr, pivot_idx + 1, high);
    qs(arr, low, pivot_idx - 1);
}
fn pivot(arr: &mut Vec<i32>, low: usize, high: usize) -> i32 {
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


fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }

    arr
}

fn bs(arr: Vec<i32>, target: i32) -> Result<usize, i32> {
    let mut high: usize = arr.len();
    let mut low: usize = 0;

    while high > low {
        let tmp: f32 = (low + (high - low) / 2) as f32;
        let mid: usize = tmp.floor() as usize;
        if arr[mid] == target {
            return Ok(mid);
        }
        if arr[mid] > target {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    Err(-1)
}
