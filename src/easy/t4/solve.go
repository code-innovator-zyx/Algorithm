package main

func isSubsequence(s string, t string) bool {
	var m, n, i, j = len(s), len(t), 0, 0
	for {
		if i >= m || j >= n {
			break
		}
		if s[i] == t[j] {
			i++
		}
		j++
	}

	return i == m
}
