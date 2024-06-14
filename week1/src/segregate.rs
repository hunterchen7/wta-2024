use std::collections::LinkedList;

/// Segregate 0s, 1s, and 2s linked list such that all 0s segregate to head side,
/// 2s at the end, and 1s in between 0s and 2s. Sorts in-place.
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
/// We create three new linked lists to store the 0s, 1s, and 2s. We then iterate over the
/// original linked list and push the values to the respective lists. Then we append the
/// 1s to the 0s and the 2s to the 1s. We then append the 0s to the original linked list.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n), where n is the length of the linked list, since we iterate
/// over the linked list exactly once. The space complexity is also O(n) since we create
/// three new linked lists to store the 0s, 1s, and 2s.
///
/// However, we can reduce the space complexity to O(1) by shifting the links in place.
///
/// There is also an alternate approach of counting the number of 0s, 1s, and 2s in the linked
/// list and then setting the values accordingly. This would also have a time complexity of O(n)
/// and a space complexity of O(1) but supposing that the linked lists store more information, this
/// approach would be incorrect. And it's also kind of cheating.
///
/// # Panics
///
/// Panics if there is an invalid value in the linked list, i.e. not 0, 1, or 2.
pub fn segregate(head: &mut LinkedList<u8>) {
    let mut zero = LinkedList::new();
    let mut one = LinkedList::new();
    let mut two = LinkedList::new();

    while let Some(x) = head.pop_front() {
        match x {
            0 => zero.push_back(x),
            1 => one.push_back(x),
            2 => two.push_back(x),
            _ => panic!("Invalid value in linked list"),
        }
    }

    zero.append(&mut one);
    zero.append(&mut two);
    head.append(&mut zero);
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
