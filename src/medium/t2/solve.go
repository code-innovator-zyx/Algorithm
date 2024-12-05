package main

import "fmt"

func main() {
	prices := []int{7, 1, 5, 3, 6, 4}
	profit := maxProfit(prices)
	fmt.Println(profit)
}

func maxProfit(prices []int) int {
	total := 0
	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			total += prices[i] - prices[i-1]
		}
	}
	return total
}
