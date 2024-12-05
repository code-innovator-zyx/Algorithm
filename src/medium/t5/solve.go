package main

import (
	"sort"
)

func threeSum(nums []int) [][]int {
	// 为了保证不重复，先从小到大排序
	sort.Ints(nums)
	// [-4,-1,-1,0,1,2]
	var results = [][]int{}
	n := len(nums)
	// 固定 最小的数， 后两个数指针游动
	for i := 0; i < n-2; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			// 保证不重复
			continue
		}
		// 三数相加等于0，如果最小的三个数就大于0,没有更小的数了
		if nums[i]+nums[i+1]+nums[i+2] > 0 {
			return results
		}
		// 如果最小的一个值加最大的两个值都小于0，终止循环
		if nums[i]+nums[n-2]+nums[n-1] < 0 {
			continue
		}
		//  第三个数的游标指针
		var k = n - 1
		// 对第二个数的指针进行滑动，每次前进一格
		var j = i + 1
		for j < k {
			var sum = nums[j] + nums[k] + nums[i]
			// 如果三个数相加大于0，将最大的数往左移动一位
			if sum > 0 {
				k--

			} else if sum < 0 {
				j++
			} else {
				// 否则就等于0了
				results = append(results, []int{nums[i], nums[j], nums[k]})
				// 继续下一个指针数
				for j < k && nums[j+1] == nums[j] {
					j++
				}
				for j < k && nums[k-1] == nums[k] {
					k--
				}
				j++
				k--
			}
		}
	}
	return results
}
