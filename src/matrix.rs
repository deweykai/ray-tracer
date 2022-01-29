#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Matrix {
        let height = data.len();
        let width = data[0].len();
        Matrix {
            data,
            width,
            height,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        return self.data == self.data;
    }
}

use std::ops::Index;

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &f64 {
        let (x, y) = index;
        &self.data[x][y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }
    #[test]
    fn construct_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }
    #[test]
    fn construxt_2x2_matrix() {
        let m = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }
    #[test]
    fn compare_similar_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }
    #[test]
    fn compare_disimilar_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }
}
