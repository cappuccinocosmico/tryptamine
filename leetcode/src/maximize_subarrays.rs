struct Solution {}
impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        // first deal with the case where conflicting_pairs.len() < 2.
        // Namely the number of elements it should return should be n+1 choose 2, Which is n*(n+1)/2
        // How many
        let array: Vec<i32> = (1..n + 1).collect();
        let mut biggest_count = 0;
        for (removed_pair_index, removed_val) in conflicting_pairs.iter().enumerate() {
            let mut candidate_count = 0;
            for big_index in 1..n + 1 {
                for little_index in 0..big_index {
                    let mut is_slice_valid = true;
                    for (pair_index, pair_val) in conflicting_pairs.iter().enumerate() {
                        if pair_index != removed_pair_index {
                            let lower_bound = pair_val[0];
                            let upper_bound = pair_val[1];
                            is_slice_valid = is_slice_valid
                                && (little_index >= lower_bound || big_index < upper_bound)
                        }
                    }
                    if is_slice_valid {
                        println!(
                            "This should be valid, when {removed_val:?} is removed {:?}",
                            &array[little_index as usize..big_index as usize]
                        );
                    }
                    candidate_count += is_slice_valid as i64;
                }
            }
            biggest_count = biggest_count.max(candidate_count);
        }
        biggest_count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 4;
        let conflicting_pairs = vec![vec![2, 3], vec![1, 4]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_example_2() {
        let n = 5;
        let conflicting_pairs = vec![vec![1, 2], vec![2, 5], vec![3, 5]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_example_3() {
        let n = 10;
        let conflicting_pairs = vec![vec![10, 5], vec![3, 8]];
        let result = Solution::max_subarrays(n, conflicting_pairs);
        assert_eq!(result, 50);
    }
}
