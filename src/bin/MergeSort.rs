extern crate rand;

use rand::distributions::{Distribution, Uniform};

fn generate_numbers() -> Vec<i32> {
    /// Returns a vector with randomly generated numbers
    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(1..100);

    let mut nums : Vec<i32> = Vec::new();

    for x in 0..100 {
        let num = distribution.sample(&mut rng);
        nums.push(num);
    }
    return nums;
}

fn merge(arr: &mut Vec<i32>, start: i32, mid: i32, end: i32) {
    let mut temp = vec![0; (end - start + 1) as usize];

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

fn merge_sort(arr: &mut Vec<i32>, start: i32, end: i32) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(arr, start, mid);
        merge_sort(arr, mid + 1, end);
        merge(arr, start, mid, end)
    }
}

fn main() {

    let mut nums = generate_numbers();
    let length: i32 = nums.len() as i32;

    merge_sort(&mut nums, 0, length - 1);

    for num in nums {
        println!("{}", num)
    }
}
