/// Sorts an array into a "wave" array: nums[0] >= nums[1] <= nums[2] >= nums[3] <= nums[4] >= ...
/// This is done in-place. The time complexity is O(n), where n is the length of the list.
///
/// This is one of many possible answers, as there may be multiple possible wave arrays.
/// e.g. [2, 2, 2, 3, 3, 3] is a valid wave array, as is [3, 2, 3, 2, 3, 2].
///
/// # Arguments
///
/// * `nums` - A mutable slice of i32, the list to be sorted
///
/// # Returns
///
/// The sorted list
///
/// # Example
///
/// ```
/// let mut nums = [10, 5, 6, 3, 2, 20, 100, 80];
/// wave_sort(&mut nums);
/// assert_eq!(nums, [10, 5, 6, 2, 20, 3, 100, 80]);
///
/// let mut nums = [2, 2, 2, 3, 3, 3];
/// wave_sort(&mut nums);
/// assert_eq!(nums, [2, 2, 2, 3, 3, 3]);
/// ```
pub fn wave_sort(nums: &mut [i32]) -> &mut [i32] {
    if nums.is_empty() {
        return nums;
    }
    for i in 0..nums.len() - 1 {
        if (i % 2 == 0 && nums[i] < nums[i + 1]) || (i % 2 == 1 && nums[i] > nums[i + 1]) {
            nums.swap(i, i + 1);
        }
    }
    nums
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_valid_wave(nums: &[i32]) {
        if nums.is_empty() || nums.len() == 1 {
            return;
        }
        let case = nums[0] >= nums[1];
        for i in 1..nums.len() - 1 {
            if i % 2 == if case { 0 } else { 1 } {
                assert!(nums[i] >= nums[i + 1]);
            } else {
                assert!(nums[i] <= nums[i + 1]);
            }
        }
    }

    #[test]
    fn test_check_valid_wave() {
        let nums = [3, 2, 3, 2, 3, 2, 3];
        check_valid_wave(&nums);
    }

    #[test]
    fn test_check_valid_wave_down_up() {
        let nums = [3, 4, 3, 4, 3, 4, 3];
        check_valid_wave(&nums);
    }

    #[test]
    fn test_check_valid_wave_same() {
        let nums = [3; 8];
        check_valid_wave(&nums);
    }

    #[test]
    #[should_panic]
    fn test_check_valid_wave_invalid() {
        let nums = [1, 2, 3, 4, 5, 6];
        check_valid_wave(&nums);
    }

    #[test]
    fn test_wave_sort_1() {
        let mut nums = [10, 5, 6, 3, 2, 20, 100, 80];
        wave_sort(&mut nums);
        check_valid_wave(&nums);
    }

    #[test]
    fn test_wave_sort_2() {
        let mut nums = [20, 10, 8, 6, 4, 2];
        wave_sort(&mut nums);
        check_valid_wave(&nums);
    }

    #[test]
    fn test_wave_sort_sorted() {
        let mut nums = (0..10).collect::<Vec<i32>>();
        wave_sort(&mut nums);
        check_valid_wave(&nums);
    }

    #[test]
    fn test_wave_sort_sorted_same() {
        let mut nums = vec![3; 10];
        wave_sort(&mut nums);
        check_valid_wave(&nums);
    }

    #[test]
    fn test_wave_sort_empty() {
        let mut nums = Vec::new();
        wave_sort(&mut nums);
        assert!(nums.is_empty());
    }
}
