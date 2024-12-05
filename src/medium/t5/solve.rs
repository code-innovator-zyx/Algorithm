pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();
    let n = nums.len();
    for i in 0..n - 2 {
        let a = nums[i];
        if i != 0 && a == nums[i - 1] {
            continue;
        }
        // 三个最小的数相加都>0，没有更小的结果了
        if a + nums[i + 1] + nums[i + 2] > 0 {
            return result;
        }
        // 如果和最大的两个数相加都小于0，当前循环不会有更大的结果了
        if a + nums[n - 1] + nums[n - 2] < 0 {
            continue;
        }
        let (mut j, mut k) = (i + 1, n - 1);
        while j < k {
            match a + nums[j] + nums[k] {
                sum if sum > 0 => {
                    k -= 1;
                }
                sum if sum < 0 => {
                    j += 1;
                }
                _ => {
                    result.push(vec![a, nums[j], nums[k]]);
                    while j < k && nums[j + 1] == nums[j] {
                        j += 1;
                    }
                    while j < k && nums[k - 1] == nums[k] {
                        k -= 1;
                    }
                    j += 1;
                    k -= 1;
                }
            }
        }
    }
    result
}

// 二分查找
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, nums.len() as i32 - 1);
    let mut result = vec![-1, -1];
    // 二分查找寻找目标
    while left <= right {
        // 中间数
        let mid = left + (right - left) / 2;
        match nums[mid as usize] {
            num if num == target => {
                // 找到目标了
                result[0] = mid;
                // 向左找可能存在的最小值
                right = mid - 1
            }
            num if num < target => {
                // 小于目标，在右侧
                left = mid + 1
            }
            _ => {
                right = mid - 1
                // 大于目标，在右边
            }
        }
    }
    (left, right) = (0, nums.len() as i32 - 1);
    while left <= right {
        // 中间数
        let mid = left + (right - left) / 2;
        match nums[mid as usize] {
            num if num == target => {
                // 找到目标了
                result[1] = mid;
                // 向右找可能存在的最大值
                left = mid + 1
            }
            num if num < target => {
                // 小于目标，在左侧
                left = mid + 1
            }
            _ => {
                right = mid - 1
                // 大于目标，在右边
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = search_range(nums, target);
        assert_eq!(result, vec![3, 4])
    }
}
