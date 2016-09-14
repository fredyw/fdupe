use std::cmp;

fn edit_distance(str1: &str, str2: &str) -> usize {
    // let str1_size = str1.chars().count() + 1;
    // let str2_size = str2.chars().count() + 1;
    // let mut dp = vec![vec![0; str2_size]; str1_size];
    // for i in 0..str1_size {
    //     for j in 0..str2_size {
    //         if i == 0 {
    //             dp[i][j] = j;
    //         } else if j == 0 {
    //             dp[i][j] = i;
    //         } else if vec_str1[i - 1] == vec_str2[j - 1] {
    //             dp[i][j] = dp[i - 1][j - 1];
    //         } else {
    //             dp[i][j] = 1 + cmp::min(
    //                 dp[i][j - 1],
    //                 cmp::min(dp[i - 1][j],
    //                     dp[i - 1][j - 1]));
    //         }
    //     }
    // }
    // dp[str1_size][str2_size]
    0
}

#[cfg(test)]
mod test {
    use fdupe;

    #[test]
    fn test_edit_distance() {
        assert_eq!(1, fdupe::edit_distance("hello", "holla"));
    }
}