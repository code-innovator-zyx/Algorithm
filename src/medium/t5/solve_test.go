package main

import (
	"fmt"
	"testing"
)

func Test_threeSum(t *testing.T) {
	var nums = []int{-1, 0, 1, 2, -1, -4}
	fmt.Println(threeSum(nums))
}
