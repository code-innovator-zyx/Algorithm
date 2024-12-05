pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut i: usize = 0;
    let mut j = height.len() - 1;
    let mut weight = 0;
    while i < j {
        weight = j - i;
        let min_h = {
            if height[i] < height[j] {
                i += 1;
                height[i - 1]
            } else {
                j-=1;
                height[j+1]
            }
        };
        max = match weight as i32 * min_h {
            tmp if tmp > max => tmp,
            _ => max,
        };
    }
    max
}
