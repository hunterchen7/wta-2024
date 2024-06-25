use std::cmp::max;

/// Given a sorted array of houses and heaters, find the minimum radius of a heater such that
/// all houses are within the radius of a heater.
///
/// # Inputs
///
/// - houses: a slice of u32, representing the positions of houses
/// - heaters: a slice of u32, representing the positions of heaters
///
/// # Output
///
/// - u32: the minimum radius of a heater such that all houses are within the radius
///
/// # Examples
/// ```
/// let houses = vec![1,2,3];
/// let heaters = vec![2];
/// assert_eq!(min_header_radius(&houses, &heaters), 1);
/// ```
/// Explanation: The only heater is placed in position 2, heating all houses
/// ```
/// let houses = vec![1,2,3,4];
/// let heaters = vec![1,4];
/// assert_eq!(min_header_radius(&houses, &heaters), 1);
/// ```
/// Explanation: The heater in position 1 heats house 1 and 2, and the heater in position 4 heats house 3 and 4
///
/// # Approach
/// The minimum radius of a heater is the maximum distance a house is from a heater. To find this,
/// binary search for the minimum radius. The left bound is 0 and the right bound is the maximum
/// distance between the first house and the last heater, and the last house and the first heater.
/// Check if all houses can be heated by a heater with the current radius. If they can, move the
/// right bound to the current radius, otherwise move the left bound to the current radius + 1.
/// Continue until the left bound is equal to the right bound.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n log n) because of a O(log n) binary search on O(n) houses. The space
/// complexity is O(1) since the space usage is constant with a few number variables.
pub fn min_heater_radius(houses: &[u32], heaters: &[u32]) -> u32 {
    assert!(!houses.is_empty(), "houses cannot be empty!");
    assert!(!heaters.is_empty(), "heaters cannot be empty!");
    assert!(is_sorted(houses), "houses must be sorted!");
    assert!(is_sorted(heaters), "heaters must be sorted!");

    let can_heat_all = |radius: u32| -> bool {
        let mut i = 0;
        for &house in houses {
            while i < heaters.len() && heaters[i] + radius < house {
                i += 1;
            }
            if i == heaters.len() || heaters[i] > house + radius {
                return false;
            }
        }
        true
    };

    let mut left = 0;
    let mut right = max(
        // i32 conversion to prevent underflow
        houses[houses.len() - 1] as i32 - heaters[0] as i32,
        heaters[heaters.len() - 1] as i32 - houses[0] as i32,
    ) as u32;

    while left < right {
        let mid = (left + right) / 2;
        if can_heat_all(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

/// Check if an array is sorted in ascending order
fn is_sorted(arr: &[u32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_heater_radius1() {
        let houses = vec![1, 2, 3];
        let heaters = vec![2];
        assert_eq!(min_heater_radius(&houses, &heaters), 1);
    }

    #[test]
    fn test_min_heater_radius2() {
        let houses = vec![1, 2, 3, 4];
        let heaters = vec![1, 4];
        assert_eq!(min_heater_radius(&houses, &heaters), 1);
    }

    #[test]
    fn test_min_heater_radius3() {
        let houses = vec![1, 2, 3, 4, 5];
        let heaters = vec![1, 4];
        assert_eq!(min_heater_radius(&houses, &heaters), 1);
    }

    #[test]
    fn test_min_heater_radius4() {
        let houses = vec![1, 2, 3, 6, 8];
        let heaters = vec![1];
        assert_eq!(min_heater_radius(&houses, &heaters), 7);
    }

    #[test]
    fn test_min_heater_radius5() {
        let houses = vec![1, 2, 3, 6, 8];
        let heaters = vec![1, 8];
        assert_eq!(min_heater_radius(&houses, &heaters), 2);
    }
}
