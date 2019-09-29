pub mod utils;
use std::time::Instant;
use utils::generate_numbers;

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];
    let mut i = low - 1;
    for j in low..high {
        // If current element is smaller than the pivot
        if arr[j as usize] < pivot {
            i += 1;    // increment index of smaller element
            let temp = arr[i as usize];
            arr[i as usize] = arr[j as usize];
            arr[j as usize] = temp;
        }
    }
    let temp = arr[(i+1) as usize];
    arr[(i+1) as usize] = arr[high as usize];
    arr[high as usize] = temp;
    return i + 1;
}

fn quick_sort(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let pi = partition(arr, low, high);

        quick_sort(arr, low, pi - 1);  // Before pi
        quick_sort(arr, pi + 1, high); // After pi
    }
}

fn main() {
    let mut nums = generate_numbers(1000000);
//    let mut nums = vec![5, 6, 10, 50, 1];
    let length: i32 = nums.len() as i32;
    let now = Instant::now();
    quick_sort(&mut nums, 0, length - 1);
    let elapsed = now.elapsed().as_micros();
    println!("{}", elapsed);
//    println!("{:?}", nums);

}