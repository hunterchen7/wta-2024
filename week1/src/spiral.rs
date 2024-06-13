// Given a 2D array, print the elements of the array in a clockwise spiral from starting from top lef
//
// Example 1:
// Input:
// matrix = [[1,    2,   3,   4],
//                 [5,    6,   7,   8],
//                 [9,   10,  11,  12],
//                 [13,  14,  15,  16]]
// Output:
// 1 2 3 4 8 12 16 15 14 13 9 5 6 7 11 10
// Explanation:
// The output is matrix in spiral format.
//
// Example 2:
// Input:
// matrix =  [[1,   2,   3,   4,  5,   6],
//                  [7,   8,   9,  10,  11,  12],
//                  [13,  14,  15, 16,  17,  18]]
// Output:
// 1 2 3 4 5 6 12 18 17 16 15 14 13 7 8 9 10 11

pub fn spiral(matrix: &[Vec<i32>]) -> Vec<i32> {
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
        let matrix3 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        
        assert_eq!(
            spiral(&matrix3),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }
    
    #[test]
    fn test_spiral_4() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        
        assert_eq!(
            spiral(&matrix),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_spiral_empty() {
        assert_eq!(spiral(&[]), vec![]);
    }

    #[test]
    fn test_spiral_single() {
        assert_eq!(spiral(&[vec![1]]), vec![1]);
    }

    #[test]
    fn test_spiral_mega_matrix() {
        let mega_matrix = create_matrix(1000);
        assert_eq!(spiral(&mega_matrix).len(), 1000 * 1000);
    }
}
