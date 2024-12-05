package easysort_test

import "testing"

func Test_bubble_sort(t *testing.T) {
	type args struct {
		nums []int32
	}
	tests := []struct {
		name string
		args args
	}{
		// TODO: Add test cases.
		{name: "test",
			args: args{
				nums: []int32{1, 3, 7, 2, 5, 8, 3, 6, 9, 0},
			}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			bubble_sort(tt.args.nums)
		})
		t.Log(tt.args.nums)
	}
}
