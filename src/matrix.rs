use crate::tuple::Tuple;

#[derive(Debug, Clone)]
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

    pub fn identity(size: usize) -> Matrix {
        let data = (0..size)
            .map(|y| {
                (0..size)
                    .map(|x| if x == y { 1.0 } else { 0.0 })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Matrix::new(data)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }
        self.data == other.data
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

use std::ops::Mul;

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self {
        let width = rhs.width;
        let height = self.height;

        assert!(self.width == rhs.height);

        let data = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let mut sum: f64 = 0.0;
                        for i in 0..self.width {
                            sum += self.data[y][i] * rhs.data[i][x];
                        }
                        sum
                    })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        Matrix::new(data)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, t: Tuple) -> Tuple {
        assert_eq!(self.width, 4);
        assert_eq!(self.height, 4);

        let data = (0..4)
            .map(|y| {
                self.data[y][0] * t.x
                    + self.data[y][1] * t.y
                    + self.data[y][2] * t.z
                    + self.data[y][3] * t.w
            })
            .collect::<Vec<_>>();

        Tuple::new(data[0], data[1], data[2], data[3])
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
    #[test]
    fn multiply_two_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let result = Matrix::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, result);
    }
    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(a * b, result);
    }
    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix::new(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(a.clone() * Matrix::identity(4), a);
    }
}
