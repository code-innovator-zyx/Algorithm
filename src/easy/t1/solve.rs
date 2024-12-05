/*
* @Author: zouyx
* @Email: zouyx@knowsec.com
* @Date:   2024/11/8 下午6:05
* @Package:
 */
pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let mut pm = m - 1;
    let mut pn = n - 1;
    let mut p = m + n - 1;
    while pm >= 0 && pn >= 0 {
        if nums1[pm as usize] > nums2[pn as usize] {
            nums1[p as usize] = nums1[pm as usize];
            pm -= 1
        } else {
            nums1[p as usize] = nums2[pn as usize];
            pn -= 1
        }
        p -= 1
    }
    for i in 0..pn + 1 {
        nums1[i as usize] = nums2[i as usize]
    }
}

#[cfg(test)]
mod test {
    use super::merge;
    #[test]
    fn easy_t1_solve() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![4, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6])
    }
}
