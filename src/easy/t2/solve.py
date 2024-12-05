from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        min_price = prices[0]
        for i in range(1, len(prices)):
            profit = max(profit, prices[i] - min_price)
            min_price = min(min_price, prices[i])
        return profit


solution = Solution()
prices = [7, 1, 5, 3, 6, 4]

print(solution.maxProfit(prices))
