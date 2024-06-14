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
