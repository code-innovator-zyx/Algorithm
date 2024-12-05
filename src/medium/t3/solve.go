package main

import "fmt"

func main() {
	var nums = []int{2, 7, 11, 15}
	var target = 9
	result := twoSum(nums, target)
	fmt.Println(result)
}

func twoSum(numbers []int, target int) []int {
	var i, j = 0, len(numbers) - 1
	for i < j {
		if sum := numbers[i] + numbers[j]; sum == target {
			break
		} else if sum > target {
			j--
		} else {
			i++
		}
	}
	return []int{i + 1, j + 1}
}
