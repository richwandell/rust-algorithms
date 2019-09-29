use rand::distributions::{Distribution, Uniform};

pub(crate) fn generate_numbers(num: i32) -> Vec<i32> {
    // Returns a vector with randomly generated numbers
    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(1..100);

    let mut nums: Vec<i32> = Vec::new();

    for _x in 0..num {
        let num = distribution.sample(&mut rng);
        nums.push(num);
    }
    return nums;
}
