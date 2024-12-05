package main

import "fmt"

/*
* @Author: zouyx
* @Email:
* @Date:   2024/11/8 下午5:54
* @Package:
 */

func main() {
	num1 := []int{4, 5, 6, 0, 0, 0}
	m := 3
	num2 := []int{1, 2, 3}
	n := 3
	merge(num1, m, num2, n)
	fmt.Println(num1)
}

// 这一题是典型的双指针
func merge(nums1 []int, m int, nums2 []int, n int) {
	var (
		pm = m - 1
		pn = n - 1
	)
	// 最坏的情况下时间复杂度 O(M+N)  最好的情况下 O(N)
	for i := m + n - 1; pm >= 0 && pn >= 0; i-- {
		if nums1[pm] > nums2[pn] {
			nums1[i] = nums1[pm]
			pm--
		} else {
			nums1[i] = nums2[pn]
			pn--
		}
	}
	fmt.Println(pn)

	//  最坏的情况下 时间复杂度 O(M)  最好的情况下 O(o)
	for i := 0; i <= pn; i++ {
		nums1[i] = nums2[i]
	}
}
