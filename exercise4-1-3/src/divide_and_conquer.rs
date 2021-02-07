use crate::search_all::search_all;

/// 分割統治法(n*lg(n))
pub fn divide_and_conquer(arr: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if high - low <= 1 {
        return (low, high, arr[low]);
    }

    let mid = (low + high) / 2;
    let ans_left = divide_and_conquer(arr, low, mid);
    let ans_right = divide_and_conquer(arr, mid, high);
    let ans_cross = find_max_crossing_subarray(arr, low, mid, high);
    if ans_left.2 > ans_right.2 && ans_left.2 > ans_cross.2 {
        ans_left
    } else if ans_right.2 > ans_left.2 && ans_right.2 > ans_cross.2 {
        ans_right
    } else {
        ans_cross
    }
}

/// 分割統治法(n*lg(n))
/// ただし、要素数17以下の場合は総当り法を利用する
pub fn divide_and_conquer_improved(arr: &[i64], low: usize, high: usize) -> (usize, usize, i64) {
    if high - low <= 25 {
        return search_all(arr, low, high);
    }

    let mid = (low + high) / 2;
    let ans_left = divide_and_conquer(arr, low, mid);
    let ans_right = divide_and_conquer(arr, mid, high);
    let ans_cross = find_max_crossing_subarray(arr, low, mid, high);
    if ans_left.2 > ans_right.2 && ans_left.2 > ans_cross.2 {
        ans_left
    } else if ans_right.2 > ans_left.2 && ans_right.2 > ans_cross.2 {
        ans_right
    } else {
        ans_cross
    }
}

fn find_max_crossing_subarray(
    arr: &[i64],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i64) {
    let mut left_sum = -(1 << 60);
    let mut sum = 0;
    let mut max_left = 0;
    for i in (low..mid).rev() {
        sum = sum + arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = -(1 << 60);
    let mut sum = 0;
    let mut max_right = 0;
    for i in mid..high {
        sum = sum + arr[i];
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

#[test]
fn test_search_all() {
    let arr = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    let (max_low, max_high, max_sum) = divide_and_conquer(&arr, 0, arr.len());
    assert_eq!(max_low, 7);
    assert_eq!(max_high, 10);
    assert_eq!(max_sum, 43);
}

#[test]
fn test_search_all_improved() {
    let arr = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    let (max_low, max_high, max_sum) = divide_and_conquer_improved(&arr, 0, arr.len());
    assert_eq!(max_low, 7);
    assert_eq!(max_high, 10);
    assert_eq!(max_sum, 43);
}
