package main

import "fmt"

func main() {
	nums := []int{1, 2, 3, 4, 5, 6, 7}
	k := 3
	rotate(nums, k)
	fmt.Println(nums) // [5,6,7,1,2,3,4]
}

func rotate(nums []int, k int) {
	// 循环最大公倍数次
	n := len(nums)
	begin := 0

	times := gcd(len(nums), k)
	for i := 0; i < times; i++ {
		current := begin
		tmp := nums[current]
		for {
			next := (current + k) % n

			tmp, nums[next] = nums[next], tmp
			if next == begin {
				begin++
				break
			}
			current = next
		}
	}
}

// 求最大公约数 4,2   2    7  3  1
func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}
