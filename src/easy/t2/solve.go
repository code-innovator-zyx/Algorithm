package main

import "fmt"

func main() {
	var prices = []int{7, 1, 5, 3, 6, 4}
	fmt.Println(maxProfit(prices))

}

func maxProfit(prices []int) int {
	max := 0
	min := prices[0] // 最便宜的一天
	for i := 1; i < len(prices); i++ {
		if prices[i] < min {
			// 买入啊
			min = prices[i]
		}
		if dif := prices[i] - min; dif > max {
			max = dif
		}

	}
	return max
}
