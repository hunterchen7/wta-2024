// Given an unsorted array of integers, sort the array into a wave array.
// An array nums[0..n-1] is sorted in wave form if:
// nums[0] >= nums[1] <= nums[2] >= nums[3] <= nums[4] >= ...
//
// Example 1:
// Input: nums = [10, 5, 6, 3, 2, 20, 100, 80]
// Output: [10, 5, 6, 2, 20, 3, 100, 80]
//
// Explanation: Here you can see the first element is larger than the second
// and the same thing is repeated again and again.
// It can be small element - larger element - small element too,
// all you need to maintain is the up-down fashion which represents a wave.
// There can be multiple answers.
//
// Example 2:
// Input: nums = [20, 10, 8, 6, 4, 2]
// Output: [20, 8, 10, 4, 6, 2]
// Explanation: 20 > 8 < 10 > 4 < 6 > 2

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
        for i in 0..nums.len() - 1 {
            if i % 2 == 0 {
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
