
fn main() {
    let data: Vec<i32> = [1,-4,20,-1,500,4141,-3232].to_vec();
    let sorted = bubble_sort(data.clone());
    let result = bs(sorted, 20).unwrap();
    println!("{:?}", result);
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
