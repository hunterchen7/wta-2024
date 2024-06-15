use std::collections::LinkedList;

/// Segregate 0s, 1s, and 2s linked list such that all 0s segregate to head side,
/// 2s at the end, and 1s in between 0s and 2s. Sorts in-place.
///
/// I hate linked lists so much. easily top 3 contender for worst data structure.
///
/// # Arguments
///
/// * `head` - A mutable reference to a LinkedList<u8> containing only 0s, 1s, and 2s
///
/// # Examples
///
/// ```
/// // [1, 2, 2, 1, 2, 0, 2, 2] -> [0, 1, 1, 2, 2, 2, 2, 2]
/// let mut list = LinkedList::new();
/// [1, 2, 2, 1, 2, 0, 2, 2]
///     .iter()
///     .for_each(|&x| list.push_back(x));
///
/// segregate(&mut list);
///
/// let mut iter = list.iter();
///
/// [0, 1, 1, 2, 2, 2, 2, 2]
///     .iter()
///     .for_each(|&x| assert_eq!(Some(&x), iter.next()));
///
/// // [2, 2, 0, 1] -> [0, 1, 2, 2]
/// let mut list2 = LinkedList::new();
/// [2, 2, 0, 1]
///     .iter()
///     .for_each(|&x| list2.push_back(x));
///
/// segregate(&mut list2);
///
/// let mut iter2 = list2.iter();
/// [0, 1, 2, 2]
///     .iter()
///     .for_each(|&x| assert_eq!(Some(&x), iter2.next()));
/// ```
///
/// # Approach
///
/// Count the number of 0s, 1s, and 2s in the linked list, then set values of the original linked
/// list. Shifting the nodes in-place would the more "correct" solution, but I truly cannot be
/// bothered.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n), where n is the length of the linked list, since we iterate over
/// the list twice, once to count the numbers and again to set the values. The space complexity is
/// O(1) since we use a single length 3 static array to store the counts.
///
/// # Panics
///
/// Panics if there is an invalid value in the linked list, i.e. not 0, 1, or 2.
pub fn segregate(head: &mut LinkedList<u8>) {
    let counts = head.iter().fold([0; 3], |mut counts, &x| {
        if !(0u8..=2).contains(&x) {
            panic!("Invalid value in linked list: {}", x);
        }
        counts[x as usize] += 1;
        counts
    });

    let mut iter = head.iter_mut();

    (0..3).for_each(|i| {
        (0..counts[i]).for_each(|_| {
            *iter.next().unwrap() = i as u8;
        });
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segregate_1() {
        let mut list = LinkedList::new();
        [1, 2, 2, 1, 2, 0, 2, 2]
            .iter()
            .for_each(|&x| list.push_back(x));

        segregate(&mut list);

        let mut iter = list.iter();
        [0, 1, 1, 2, 2, 2, 2, 2]
            .iter()
            .for_each(|&x| assert_eq!(Some(&x), iter.next()));
    }

    #[test]
    fn test_segregate_2() {
        let mut list = LinkedList::new();
        [2, 2, 0, 1].iter().for_each(|&x| list.push_back(x));

        segregate(&mut list);

        let mut iter = list.iter();
        [0, 1, 2, 2]
            .iter()
            .for_each(|&x| assert_eq!(Some(&x), iter.next()));
    }

    #[test]
    fn test_segregate_no_0() {
        let mut list = LinkedList::new();
        [1, 2, 2, 1, 2, 2, 2]
            .iter()
            .for_each(|&x| list.push_back(x));

        segregate(&mut list);

        let mut iter = list.iter();
        [1, 1, 2, 2, 2, 2, 2]
            .iter()
            .for_each(|&x| assert_eq!(Some(&x), iter.next()));
    }

    #[test]
    fn test_segregate_no_2() {
        let mut list = LinkedList::new();
        [1, 1, 0, 1, 1, 1, 0]
            .iter()
            .for_each(|&x| list.push_back(x));

        segregate(&mut list);

        let mut iter = list.iter();
        [0, 0, 1, 1, 1, 1, 1]
            .iter()
            .for_each(|&x| assert_eq!(Some(&x), iter.next()));
    }
}
