package main

import (
	"fmt"
	"unicode"
)

func main() {
	s := ".,"

	fmt.Println(isPalindrome(s))
}

func isPalindrome(s string) bool {
	for left, right := 0, len(s)-1; left < right; {
		for {
			if left >= right {
				break
			}
			if isalnum(s[left]) {
				break
			}
			left++
		}
		for {
			if left >= right {
				break
			}
			if isalnum(s[right]) {
				break
			}
			right--
		}
		if unicode.ToLower(rune(s[left])) != unicode.ToLower(rune(s[right])) {
			return false
		}
		left++
		right--
	}
	return true
}

// 字符判断
func isalnum(ch byte) bool {
	return (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9')
}
