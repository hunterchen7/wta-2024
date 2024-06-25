/// compute the number of sub-arrays within nums that sum to k
///
/// # Inputs
///
/// - nums: a slice of i64, representing the array of integers
/// - k: an i64, representing the target sum
///
/// # Output
///
/// - u32: the number of sub-arrays that sum to k
///
/// # Examples
///
/// ```
/// assert_eq!(subarray_sums(&[1, 0, 1, 1, 0, 1], 2), 6);
/// // Explanation: 6 different sub-arrays that sum to 2
///
/// assert_eq!(subarray_sums(&[1, 2, 3], 3), 2);
/// // Explanation: The sub-arrays that sum to 3 are [1, 2], [3]
///
/// assert_eq!(subarray_sums(&[0, 0, 0, 0], 0), 10);
/// // Explanation: All combinations of 0s sum to 10
///
/// assert_eq!(subarray-sums(&[-1, 1, -1, 1], 0), 4);
/// // Explanation: The sub-arrays that sum to 0 are [-1, 1], [1, -1], [-1, 1] and [-1, 1, -1, 1]
/// ```
/// # Approach
///
/// Initialize a count variable to 0, a sum variable to 0, and a hashmap to store the sum and the
/// number of times it has been seen. Iterate over the nums, adding the current number to the sum.
/// If the sum - k is in the hashmap, increment the count by the value of the sum - k. Add the sum
/// to the hashmap, incrementing the value if it already exists. Return the count.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n) nums is iterated through exactly once. The space complexity is O(n)
/// since the hashmap can store up to n sums.
///
pub fn subarray_sums(nums: &[i64], k: i64) -> u32 {
    let mut count = 0;
    let mut sum = 0;
    let mut sum_map = std::collections::HashMap::new();
    sum_map.insert(0, 1);

    for &num in nums {
        sum += num;
        if let Some(&val) = sum_map.get(&(sum - k)) {
            count += val;
        }
        *sum_map.entry(sum).or_insert(0) += 1;
    }
    count
}

/// compute the number of sub-arrays within nums that sum to k, only allows non-negative numbers
///
/// # Inputs
///
/// - nums: a slice of u32, representing the array of integers
/// - k: an u64, representing the target sum
///
/// # Output
///
/// - u32: the number of sub-arrays that sum to k
///
/// # Examples
///
/// ```
/// assert_eq!(subarray_sums_2ptr(&[1, 0, 1, 1, 0, 1], 2), 6);
/// // Explanation: 6 different sub-arrays that sum to 2
///
/// assert_eq!(subarray_sums_2ptr(&[1, 2, 3], 3), 2);
/// // Explanation: The sub-arrays that sum to 3 are [1, 2], [3]
///
/// assert_eq!(subarray_sums_2ptr(&[0, 0, 0, 0], 0), 10);
/// // Explanation: All combinations of 0s sum to 10
///
/// assert_eq!(subarray_sums_2ptr(&[0, 0, 0, 0, 0, 0, 0], 1), 0);
/// // Explanation: No combination of 0s sum to 1
/// ```
/// # Approach
///
/// Initialize a left pointer at 0, a current sum variable to 0, and a count variable to 0. Iterate
/// over the nums, adding the current number to the sum. If the sum is greater than k, move the left
/// pointer to the right until the sum is less than or equal to k. If the sum is equal to k, increment
/// the count. If the sum is equal to k, increment the count and move the left pointer to the right
/// until the sum is no longer 0. Return the count.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n) nums is iterated through exactly once. The space complexity is O(1).
///
pub fn subarray_sums_2ptr(nums: &[u32], k: u32) -> u32 {
    let mut left = 0;
    let mut curr_sum = 0;
    let mut count = 0;

    for right in 0..nums.len() {
        curr_sum += nums[right];
        while curr_sum > k && left < right {
            curr_sum -= nums[left];
            left += 1;
        }
        if curr_sum == k {
            count += 1;
            let mut temp = left;
            while temp < right && nums[temp] == 0 {
                count += 1;
                temp += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarray_sums_1() {
        assert_eq!(subarray_sums(&[1, 0, 1, 1, 0, 1], 2), 6);
    }

    #[test]
    fn test_subarray_sums_2() {
        assert_eq!(subarray_sums(&[1, 2, 3], 3), 2);
    }

    #[test]
    fn test_subarray_sums_3() {
        assert_eq!(subarray_sums(&[0, 0, 0, 0], 0), 10);
    }

    #[test]
    fn test_subarray_sums_4() {
        assert_eq!(subarray_sums(&[0, 0, 0, 0, 0, 0, 0], 1), 0);
    }

    #[test]
    fn test_subarray_sums_negatives() {
        assert_eq!(subarray_sums(&[-1, -1, 1], 0), 1);
    }

    #[test]
    fn test_subarray_sums_mixed() {
        assert_eq!(subarray_sums(&[-1, 1, 2, 1, -1, 3], 3), 6);
        // [-1, 1, 2], [1, 2], [2, 1], [3], [1, -1, 3], [-1, 1, 2, 1]
    }

    #[test]
    fn test_subarray_sums_2tpr_1() {
        assert_eq!(subarray_sums_2ptr(&[1, 0, 1, 1, 0, 1], 2), 6);
    }

    #[test]
    fn test_subarray_sums_2tpr_2() {
        assert_eq!(subarray_sums_2ptr(&[1, 2, 3], 3), 2);
    }

    #[test]
    fn test_subarray_sums_2tpr_3() {
        assert_eq!(subarray_sums_2ptr(&[0, 0, 0, 0], 0), 10);
    }

    #[test]
    fn test_subarray_sums_2tpr_4() {
        assert_eq!(subarray_sums_2ptr(&[0, 0, 0, 0, 0, 0, 0], 1), 0);
    }
}
