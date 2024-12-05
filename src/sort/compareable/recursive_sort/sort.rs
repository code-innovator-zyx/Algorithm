// 归并排序 分治算法
fn merge_sort(nums: &mut [i32]) {
    // 已经是有序的了
    if nums.len() <= 1 {
        return;
    }
    // 存放排序数据
    let mut result = Vec::with_capacity(nums.len());
    let mid = nums.len() / 2;
    let (left, right) = nums.split_at_mut(mid);
    // 将左边的排序好
    merge_sort(left);
    // 将后边的排序好
    merge_sort(right);
    // 合并排序
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    nums.copy_from_slice(&result)
}
// 快排
fn fast_sort(nums: &mut [i32]) {}
// 堆排序
fn heap_sort(nums: &mut [i32]) {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        merge_sort(&mut nums);
        println!("insert_sort {:?}", nums);
    }

    #[test]
    fn test_fast_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        fast_sort(&mut nums);
        println!("insert_sort {:?}", nums);
    }

    #[test]
    fn test_heap_sort() {
        let mut nums = vec![1, 4, 7, 2, 5, 8, 3, 6, 9, 0];
        heap_sort(&mut nums);
        println!("insert_sort {:?}", nums);
    }
}
