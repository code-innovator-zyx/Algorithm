from typing import List
class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        times = self.gcd(len(nums),k)
        begin = 0
        for _ in range (0,times):
            tmp =nums[begin] 
            current =begin
            while True:
                next = (current+k)%len(nums)
                tmp,nums[next] = nums[next],tmp
                if next ==begin:
                    begin+=1
                    break
                current=next


    # 最大公约数
    def gcd(self,a ,b):
        if b==0:
            return a 
        return self.gcd(b,a%b)




solution = Solution()

nums = [1, 2, 3, 4, 5, 6, 7]   # [5,6,7,1,2,3,4]
k =3
solution.rotate(nums,k)
print(nums)