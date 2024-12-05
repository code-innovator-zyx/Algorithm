pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            total += prices[i] - prices[i - 1]
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::max_profit;

    #[test]
    fn easy_t1_solve() {}
}
