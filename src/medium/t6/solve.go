package main

import "math"

// 双指针
func minSubArrayLen(target int, nums []int) int {
	// 头指针 尾指针
	var start, end = 0, 0
	// 记录头指针和尾指针区域的和
	var sum = 0
	// 记录最小长度
	var size = math.MaxInt
	// 尾指针走到最后为止
	for end < len(nums) {
		sum += nums[end]
		// 如果满足区域和>=target
		for sum >= target {
			// 计算区域距离
			size = min(size, end-start+1)
			// 头指针前进一位，区域和减去头指针位置
			sum -= nums[start]
			start++
		}
		end++
	}
	if size == math.MaxInt {
		size = 0
	}
	return size
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
