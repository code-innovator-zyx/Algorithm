class Solution:
    def isPalindrome(self, s: str) -> bool:
        left = 0
        right =len(s)-1
        while left<right:
            while left<right:
                if s[left].isalnum():
                    break
                left+=1
            while left<right:
                if s[right].isalnum():
                    break
                right-=1
            if s[left].lower() !=s[right].lower():
                return False
            left+=1
            right-=1
        return True
