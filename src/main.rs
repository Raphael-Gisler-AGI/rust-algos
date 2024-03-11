fn main() {
    let data: Vec<i32> = [1,2,3,4,5,6].to_vec();
    let result = bs(data, 45).unwrap();
    println!("{}", result);
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
