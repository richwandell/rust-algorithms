extern crate rand;

pub mod utils;

use std::time::Instant;
use utils::generate_numbers;


fn merge(arr: &mut Vec<i32>, start: i32, mid: i32, end: i32, temp: &mut Vec<i32>) {
    let mut i = start;
    let mut j = mid + 1;
    let mut k = 0;

    while i <= mid && j <= end {
        if arr[i as usize] <= arr[j as usize] {
            temp[k as usize] = arr[i as usize];
            k += 1;
            i += 1;
        } else {
            temp[k as usize] = arr[j as usize];
            k += 1;
            j += 1;
        }
    }

    while i <= mid {
        temp[k as usize] = arr[i as usize];
        k += 1;
        i += 1;
    }

    while j <= end {
        temp[k as usize] = arr[j as usize];
        k += 1;
        j += 1;
    }

    for i in start..end+1 {
        arr[i as usize] = temp[(i - start) as usize];
    }
}

fn merge_sort(arr: &mut Vec<i32>, start: i32, end: i32, temp: &mut Vec<i32>) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(arr, start, mid, temp);
        merge_sort(arr, mid + 1, end, temp);
        merge(arr, start, mid, end, temp)
    }
}

fn main() {

    //unoptimized 58247402
    //optimized    1098721
    let mut nums = generate_numbers(10000000);
    let length: i32 = nums.len() as i32;

    let now = Instant::now();
    let mut temp = vec![0; length as usize];
    merge_sort(&mut nums, 0, length - 1, &mut temp);
    let elapsed = now.elapsed().as_micros();

    println!("{}", elapsed);
//    println!("{:?}", nums);
}
