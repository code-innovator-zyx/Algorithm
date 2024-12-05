use std::mem;
pub fn rotate(nums: &mut [i32], k: i32) {
    let k = k as usize;
    let n = nums.len();
    // 需要循环的次数

    // 开始的索引
    let mut begin: usize = 0;
    let mut count: usize = 0;
    loop {
        // 被替换的占位
        let mut tmp = nums[begin];
        // 当前所在的索引
        let mut current = begin;
        loop {
            // 计算下一次所在的位置
            let next = (current + k) % n;
            // rust 中的互换   a,b = b ,a
            mem::swap(&mut tmp, &mut nums[next]);
            count += 1;
            if next == begin {
                begin += 1;
                break;
            }
            current = next;
        }
        if count == n {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::rotate;

    #[test]
    fn medium_t1_solve() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4])
    }
}
