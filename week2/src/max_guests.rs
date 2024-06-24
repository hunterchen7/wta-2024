/// Given two arrays representing entry and exit times of guests at a party,
/// find the time at which the maximum number of guests are present.
///
/// # Inputs
///
/// - arrival: a mut ref to an u32 vec, which are the unsorted times of arrival of guests
/// - exits: a mut ref to an u32 vec, which are the unsorted times of when guests leave
///
/// # Output
///
/// - u32: the time when the maximum number of guests are present
///
/// # Examples
///
/// ```
/// let mut entry = vec![1,2,9,5,5];
/// let mut exit = vec![4,5,12,9,12];
///
/// assert_eq!(maximum_guests(&mut entry, &mut exit), 5);
/// ```
///
/// Explanation: The max number of guests (3) are present at time 5
///
/// ```
/// let mut entry = vec![3, 3, 9, 10, 8, 12];
/// let mut exit = vec![4, 6, 20, 15, 12, 16];
///
/// assert_eq!(maximum_guests(&mut entry, &mut exit), 12);
/// ```
///
/// Explanation: The max number of guests (4) are present at time 12
///
/// # Approach
///
/// Sort the arrival and exit times. Initialize variables to keep track of max guests, current
/// number of guests, and the time when there are the most guests present. Iterate over the arrival
/// and exit times, incrementing the current number of guests when one arrives, decrementing when
/// one leaves. The arrival iterator moves forward when the arrival time is less than or equal to
/// the exit time, and the exit iterator moves forward when the exit time is less than the arrival.
/// When a guest enters, check if the total count is greater than the previous and update it if so.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n log n) due to the sorting of the arrival and exit times. The space
/// complexity is O(1) since the space usage is constant with a few number variables.
pub fn maximum_guests(arrivals: &mut [u32], exits: &mut [u32]) -> u32 {
    arrivals.sort_unstable();
    exits.sort_unstable();

    let mut max_guests = 0;
    let mut cur_guests = 0;
    let mut time = 0;

    let mut arrival_iter = arrivals.iter();
    let mut exit_iter = exits.iter();

    let mut arrival = arrival_iter.next();
    let mut exit = exit_iter.next();

    while arrival.is_some() {
        if exit.is_none() || arrival.unwrap() <= exit.unwrap() {
            cur_guests += 1;
            if cur_guests > max_guests {
                max_guests = cur_guests;
                time = *arrival.unwrap();
            }
            arrival = arrival_iter.next();
        } else {
            cur_guests -= 1;
            exit = exit_iter.next();
        }
    }

    time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maximum_guests1() {
        let mut arrivals = vec![1, 2, 9, 5, 5];
        let mut exits = vec![4, 5, 12, 9, 12];
        assert_eq!(maximum_guests(&mut arrivals, &mut exits), 5);
    }

    #[test]
    fn test_maximum_guests2() {
        let mut arrivals = vec![3, 3, 9, 10, 8, 12];
        let mut exits = vec![4, 6, 20, 15, 12, 16];
        assert_eq!(maximum_guests(&mut arrivals, &mut exits), 12);
    }

    #[test]
    fn test_maximum_guests_no_exits() {
        let mut arrivals = vec![1, 2, 9, 5, 5, 5, 6, 7];
        let mut exits = vec![];
        assert_eq!(maximum_guests(&mut arrivals, &mut exits), 9);
    }

    #[test]
    fn test_maximum_guests_big_times() {
        let mut arrivals = vec![1, 100, 1000, 10000, u32::MAX];
        let mut exits = vec![2, 101, 1001, 10001, u32::MAX];
        assert_eq!(maximum_guests(&mut arrivals, &mut exits), 1);
    }

    #[test]
    fn test_maximum_guests_no_arrivals() {
        // don't think this is possible but i think it'd be 0 if it was
        let mut arrivals = vec![];
        let mut exits = vec![1, 2, 3, 4, 5];
        assert_eq!(maximum_guests(&mut arrivals, &mut exits), 0);
    }
}
