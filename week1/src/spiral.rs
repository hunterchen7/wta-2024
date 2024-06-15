/// Constructs a spiral represented as a Vec<T> from a 2D matrix of T, where T is a PartialOrd and
/// Copy type, such as an integer like u32 or i32.
///
/// # Arguments
///
/// * `matrix` - A 2D matrix of T
///
/// # Returns
///
/// A Vec<T> representing the spiral of the matrix, i.e. the order it was traversed
///
/// # Example
///
/// ```
/// let matrix = vec![
///     vec![1, 2, 3, 4],
///     vec![5, 6, 7, 8],
///     vec![9, 10, 11, 12],
///     vec![13, 14, 15, 16],
/// ];
///
/// assert_eq!(
///     spiral(&matrix),
///     vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
/// );
/// ```
///
/// # Approach
///
/// We keep track of the top, bottom, left, and right borders of the matrix, and the direction we
/// are currently moving in. We then extend the spiral in that direction, and change direction when
/// we reach the border of the matrix, making sure the borders shrink as we go along. We continue
/// until either the top and bottom borders, or left and right borders converge.
///
/// # Time and Space Complexity
///
/// The time complexity is O(n), where n is the number of elements in the matrix since each element
/// is visited exactly once. The space complexity is also O(n) since the result vector is the same
/// size as the matrix. It could be done in O(1) relatively trivially, where instead of extending
/// the result vector, we print the elements directly. But I don't like printing inside of
/// functions, so I'm not going to do that.
// pub fn wave_sort<T: PartialOrd + Copy>(nums: &mut [T]) -> &mut [T]
pub fn spiral<T: PartialOrd + Copy>(matrix: &[Vec<T>]) -> Vec<T> {
    if matrix.is_empty() {
        return Vec::new();
    }

    // this shaves off a lot of time as there is no need to reallocate memory in the future
    let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());

    let (mut top, mut bottom) = (0, matrix.len() - 1);
    let (mut left, mut right) = (0, matrix[0].len() - 1);

    let mut direction = 0; // i considered using an enum but that's too extra

    while top <= bottom && left <= right {
        match direction {
            0 => {
                result.extend((left..=right).map(|i| matrix[top][i]));
                top += 1;
            }
            1 => {
                result.extend((top..=bottom).map(|i| matrix[i][right]));
                right -= 1;
            }
            2 => {
                result.extend((left..=right).rev().map(|i| matrix[bottom][i]));
                bottom -= 1;
            }
            3 => {
                result.extend((top..=bottom).rev().map(|i| matrix[i][left]));
                left += 1;
            }
            _ => unreachable!(),
        }
        direction = (direction + 1) % 4;
    }

    result
}

/// Creates a matrix of size x size, with elements from 0 to size^2 - 1
///
/// # Arguments
///
/// * `size` - The width and length of the generated matrix
///
/// # Returns
///
/// A Vec<Vec<i32>> representing the matrix
///
/// # Panics
///
/// Panics if size is less than or equal to 0
///
/// # Example
///
/// ```
/// let matrix = create_matrix(3);
/// assert_eq!(matrix, vec![
///     vec![0, 1, 2],
///     vec![3, 4, 5],
///     vec![6, 7, 8],
/// ]);
/// ```
pub fn create_matrix(size: i32) -> Vec<Vec<i32>> {
    assert!(size > 0);

    (0..size)
        .map(|i| (0..size).map(|j| i * size + j).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral1() {
        let matrix1 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        assert_eq!(
            spiral(&matrix1),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }

    #[test]
    fn test_spiral2() {
        let matrix2 = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18],
        ];

        assert_eq!(
            spiral(&matrix2),
            vec![1, 2, 3, 4, 5, 6, 12, 18, 17, 16, 15, 14, 13, 7, 8, 9, 10, 11]
        );
    }

    #[test]
    fn test_spiral_3() {
        let matrix3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(spiral(&matrix3), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_spiral_4() {
        let matrix = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];

        assert_eq!(spiral(&matrix), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_spiral_empty() {
        let empty_matrix: Vec<Vec<i32>> = Vec::new();
        assert_eq!(spiral(&empty_matrix), vec![]);
    }

    #[test]
    fn test_spiral_single() {
        assert_eq!(spiral(&[vec![1]]), vec![1]);
    }

    #[test]
    fn test_spiral_float() {
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        assert_eq!(
            spiral(&matrix),
            vec![1.0, 2.0, 3.0, 6.0, 9.0, 8.0, 7.0, 4.0, 5.0]
        );
    }

    #[test]
    fn test_spiral_mega_matrix() {
        let mega_matrix = create_matrix(1000);
        assert_eq!(spiral(&mega_matrix).len(), 1000 * 1000);
    }
}
