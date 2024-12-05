package easysort_test

// 冒泡排序
func bubble_sort(nums []int32) {
	end := len(nums) - 1

	for i := 0; i < len(nums); i++ {
		isSwap := false
		for j := 0; j < end; j++ {
			if nums[j] > nums[j+1] {
				nums[j], nums[j+1] = nums[j+1], nums[j]
				isSwap = true
			}
		}
		if !isSwap {
			return
		}
		end -= 1
	}
}


// 