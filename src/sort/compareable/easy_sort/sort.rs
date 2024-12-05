//冒泡排序
fn bubble_sort(nums: &mut [i32]) {
    let mut end = nums.len().saturating_sub(1);

    for _ in 0..nums.len() {
        let mut is_sweap = false;
        for j in 0..end {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                is_sweap = true
            }
        }
        // 如果一轮比较没有一个交换位置，说明全是排序好的
        if !is_sweap {
            return;
        }
        // 一次结束，nums[end]一定是最大的,内循环少循环一次
        end = end.saturating_sub(1);
    }
}

// 选择排序
fn select_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        let mut min = i;
        // 每次找到最小的一个数
        for j in i + 1..nums.len() {
            if nums[j] < nums[min] {
                min = j
            }
        }
        // 将她与第一个数兑换位置
        nums.swap(i, min);
    }
}

//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        bubble_sort(&mut nums);
        println!("after sort {:?}", nums);
    }

    #[test]
    fn test_select_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        select_sort(&mut nums);
        println!("select_sort {:?}", nums);
    }
}
