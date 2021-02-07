// 総当たり法(n^2)
pub fn search_all(arr: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    let mut max_sum = -(1 << 60);
    let mut max_low = 0;
    let mut max_high = 0;

    for i in low..high - 1 {
        let mut sum = 0;
        for j in i..high - 1 {
            sum += arr[j];
            if sum >= max_sum {
                max_sum = sum;
                max_low = i;
                max_high = j;
            }
        }
    }

    (max_low, max_high, max_sum)
}

#[test]
fn test_search_all() {
    let arr = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    let (max_low, max_high, max_sum) = search_all(&arr, 0, arr.len());
    assert_eq!(max_low, 7);
    assert_eq!(max_high, 10);
    assert_eq!(max_sum, 43);
}
