pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max: i32 = 0;
    let mut min_price = prices[0];

    for i in 1..prices.len() {
        max = max.max(prices[i] - min_price);
        if prices[i] < min_price {
            min_price = prices[i];
        }
    }

    max
}

#[cfg(test)]
mod test {
    use super::max_profit;

    #[test]
    fn easy_t2_solve() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max = max_profit(prices);
        assert_eq!(max, 5)
    }
}
