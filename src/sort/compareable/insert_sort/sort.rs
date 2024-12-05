use core::num;

// 插入排序
fn insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        // 记录当前值
        let current = nums[i];
        // 记录游标索引
        let mut j = i;
        // 当值大于当前值，将值往后移动一位
        while j > 0 && nums[j - 1] > current {
            // 前进一位
            nums[j] = nums[j - 1];
            j -= 1
        }
        // 将当前值插入第一个小于当前值的后面
        nums[j] = current
    }
}

// 希尔排序(缩小增量排序)   插入排序的变种
fn shell_sort(nums: &mut [i32]) {
    // 设置初始步长
    let mut gap = nums.len() / 2;
    // 当步长为1 执行完后就退出
    while gap > 0 {
        for i in gap..nums.len() {
            let current = nums[i];
            let mut j = i;
            while j >= gap && nums[j - gap] > current {
                nums[j] = nums[j - gap];
                j -= gap
            }
            nums[j] = current
        }
        gap /= 2;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        insert_sort(&mut nums);
        println!("insert_sort {:?}", nums);
    }
    #[test]
    fn test_shell_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        shell_sort(&mut nums);
        println!("shell_sort {:?}", nums);
    }
}
