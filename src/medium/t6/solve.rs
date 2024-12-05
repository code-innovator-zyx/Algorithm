use std::{cmp::min, i32::MAX};

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut start = 0_usize;
    let mut size = MAX;
    let mut sum = 0;
    for end in 0..nums.len() {
        sum += nums[end];
        while sum >= target {
            size = min(size, (end - start + 1) as i32);
            sum -= nums[start];
            start += 1;
        }
    }
    if size == MAX {
        size = 0
    }
    size
}
