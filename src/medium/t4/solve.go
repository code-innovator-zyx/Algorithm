package main

import "fmt"

func maxArea(height []int) (max int) {
	var minH = 0

	for i, j := 0, len(height)-1; i < j; {
		weight := j - i
		if height[i] < height[j] {
			minH = height[i]
			i++
		} else {
			minH = height[j]
			j--
		}
		if weight*minH > max {
			max = weight * minH
		}
	}
	return
}

func main() {
	var height = []int{1, 8, 6, 2, 5, 4, 8, 3, 7}
	fmt.Print(maxArea(height))
}
