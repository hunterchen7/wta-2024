pub struct NumMatrix {
    prefix_sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    pub fn new(matrix: &[Vec<i32>]) -> NumMatrix {
        NumMatrix {
            prefix_sums: {
                let rows = matrix.len();
                assert_ne!(rows, 0, "matrix cannot be empty!");
                let cols = matrix[0].len();
                let mut prefix_sums = vec![vec![0; cols + 1]; rows + 1];

                for row in 0..rows {
                    assert_eq!(
                        cols,
                        matrix[row].len(),
                        "every row must have the same length!"
                    );
                    for col in 0..cols {
                        prefix_sums[row + 1][col + 1] = matrix[row][col]
                            + prefix_sums[row + 1][col]
                            + prefix_sums[row][col + 1]
                            - prefix_sums[row][col];
                    }
                }
                prefix_sums
            },
        }
    }

    pub fn sum_region(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> i32 {
        assert!(row1 <= row2 && col1 <= col2);
        self.prefix_sums[row2 + 1][col2 + 1]
            - self.prefix_sums[row1][col2 + 1]
            - self.prefix_sums[row2 + 1][col1]
            + self.prefix_sums[row1][col1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_matrix_1() {
        let matrix = NumMatrix::new(&[
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn test_num_matrix_all1s() {
        let matrix = NumMatrix::new(&[vec![1; 5], vec![1; 5], vec![1; 5], vec![1; 5], vec![1; 5]]);

        assert_eq!(matrix.sum_region(0, 0, 1, 1), 4);
        assert_eq!(matrix.sum_region(0, 0, 3, 3), 16);
        assert_eq!(matrix.sum_region(0, 0, 4, 4), 25);
    }

    #[test]
    fn test_num_matrix_all0s() {
        let matrix = NumMatrix::new(&[vec![0; 5], vec![0; 5], vec![0; 5], vec![0; 5], vec![0; 5]]);

        assert_eq!(matrix.sum_region(0, 0, 3, 3), 0);
    }
}
