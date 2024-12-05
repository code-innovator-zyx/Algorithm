fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i: usize = 0;
    let mut j: usize = numbers.len() - 1;

    while i < j {
        match numbers[i] + numbers[j] {
            tmp if tmp == target => break,
            tmp if tmp > target => j -= 1,
            _ => i += 1,
        }
    }
    vec![i as i32 + 1, j as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
