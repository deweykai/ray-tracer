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

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.data[x][y]
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

    pub fn transpose(&self) -> Matrix {
        let data = (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| self.data[y][x])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Matrix::new(data)
    }

    pub fn determinant(&self) -> Result<f64, &'static str> {
        if self.width != self.height {
            return Err("not a square matrix");
        }

        if self.width == 2 {
            Ok(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0])
        } else {
            let mut sum = 0.;
            for i in 0..self.width {
                sum += self.data[0][i] * self.cofactor(0, i).unwrap();
            }
            Ok(sum)
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Result<Matrix, &'static str> {
        if row >= self.height || col >= self.width {
            return Err("invalid index");
        }

        let data = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != row)
            .map(|(_, v)| v)
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != col)
                    .map(|(_, v)| *v)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Matrix::new(data))
    }

    pub fn minor(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        Ok(self.submatrix(row, col)?.determinant()?)
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        Ok(self.minor(row, col)? * if (row + col) % 2 == 1 { -1. } else { 1. })
    }

    pub fn invertible(&self) -> bool {
        if let Ok(det) = self.determinant() {
            if det != 0. {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn inverse(&self) -> Result<Matrix, &'static str> {
        if !self.invertible() {
            return Err("matrix not invertible");
        }

        let size = self.width;

        let cofactors = Matrix::new(
            (0..size)
                .map(|y| {
                    (0..size)
                        .map(|x| self.cofactor(y, x).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );

        let det = self.determinant().unwrap();

        Ok(cofactors.transpose() / det)
    }
}

#[macro_export]
macro_rules! matrix {
    ($([$($x:expr),+]),+) => {
        {
            use crate::matrix::Matrix;
            let mut data = Vec::new();
            $(
                let mut row = Vec::new();
                $(
                    row.push($x as f64);
                )*
                data.push(row);
            )*
            Matrix::new(data)
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                if (self.data[y][x] - other.data[y][x]).abs() > 0.00001 {
                    return false;
                }
            }
        }
        return true;
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

use std::ops::{Div, Mul};

impl Mul for Matrix {
    // TODO: make matrix use references
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Matrix {
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

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix {
        let data = self
            .data
            .iter()
            .map(|row| row.iter().map(|x| x * rhs).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Matrix::new(data)
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;
    fn div(self, rhs: f64) -> Matrix {
        self * rhs.recip()
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
    #[test]
    fn transpose_matrix() {
        let a = matrix!([0, 9, 3, 0], [9, 8, 0, 8], [1, 8, 5, 3], [0, 0, 5, 8]);
        let result = matrix!([0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 8, 3, 8]);

        assert_eq!(a.transpose(), result);
    }

    #[test]
    fn transpose_identity_matrix() {
        let a = Matrix::identity(4);
        assert_eq!(a.transpose(), a);
    }

    #[test]
    fn determinant_2x2_matrix() {
        let a = matrix!([1, 5], [-3, 2]);
        assert_eq!(a.determinant().unwrap(), 17.);
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        let a = matrix!([1, 5, 0], [-3, 2, 7], [0, 6, -3]);
        assert_eq!(a.submatrix(0, 2).unwrap(), matrix!([-3, 2], [0, 6]));
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        let a = matrix!([-6, 1, 1, 6], [-8, 5, 8, 6], [-1, 0, 8, 2], [-7, 1, -1, 1]);
        let b = matrix!([-6, 1, 6], [-8, 8, 6], [-7, -1, 1]);

        assert_eq!(a.submatrix(2, 1).unwrap(), b);
    }

    #[test]
    fn calculate_minor_of_3x3_matrix() {
        let a = matrix!([3, 5, 0], [2, -1, 7], [6, -1, 5]);
        let b = a.submatrix(1, 0).unwrap();
        assert_eq!(b.determinant().unwrap(), 25.);
        assert_eq!(a.minor(1, 0).unwrap(), 25.);
    }

    #[test]
    fn calculate_cofactor_of_3x3_matrix() {
        let a = matrix!([3, 5, 0], [2, -1, -7], [6, -1, 5]);

        assert_eq!(a.minor(0, 0).unwrap(), -12.);
        assert_eq!(a.cofactor(0, 0).unwrap(), -12.);
        assert_eq!(a.minor(1, 0).unwrap(), 25.);
        assert_eq!(a.cofactor(1, 0).unwrap(), -25.);
    }

    #[test]
    fn determinant_3x3_matrix() {
        let a = matrix!([1, 2, 6], [-5, 8, -4], [2, 6, 4]);
        assert_eq!(a.cofactor(0, 0).unwrap(), 56.);
        assert_eq!(a.cofactor(0, 1).unwrap(), 12.);
        assert_eq!(a.cofactor(0, 2).unwrap(), -46.);
        assert_eq!(a.determinant().unwrap(), -196.);
    }

    #[test]
    fn determinant_4x4_matrix() {
        let a = matrix!([-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]);
        assert_eq!(a.cofactor(0, 0).unwrap(), 690.);
        assert_eq!(a.cofactor(0, 1).unwrap(), 447.);
        assert_eq!(a.cofactor(0, 2).unwrap(), 210.);
        assert_eq!(a.cofactor(0, 3).unwrap(), 51.);
        assert_eq!(a.determinant().unwrap(), -4071.);
    }

    #[test]
    fn test_invertible_matrix_invertibility() {
        let a = matrix!([6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]);
        assert_eq!(a.determinant().unwrap(), -2120.);
        assert!(a.invertible());
    }

    #[test]
    fn test_noninvertible_matrix_invertibility() {
        let a = matrix!([-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]);
        assert_eq!(a.determinant().unwrap(), 0.);
        assert!(!a.invertible());
    }

    #[test]
    fn calculate_inverse_4x4_matrix() {
        // test 1
        let a = matrix!([-5, 2, 6, -8], [1, -5, 1, 8], [7, 7, -6, -7], [1, -3, 7, 4]);
        let b = a.inverse().unwrap();

        assert_eq!(a.determinant().unwrap(), 532.);
        assert_eq!(a.cofactor(2, 3).unwrap(), -160.);
        assert_eq!(b.get(3, 2), -160. / 532.);
        assert_eq!(a.cofactor(3, 2).unwrap(), 105.);
        assert_eq!(b.get(2, 3), 105. / 532.);

        assert_eq!(
            b,
            matrix!(
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            )
        );

        // test 2
        let a = matrix!([8, -5, 9, 2], [7, 5, 6, 1], [-6, 0, 9, 6], [-3, 0, -9, -4]);
        let b = a.inverse().unwrap();

        assert_eq!(
            b,
            matrix!(
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308]
            )
        );

        // test 3
        let a = matrix!([9, 3, 0, 9], [-5, -2, -6, -3], [-4, 9, 6, 4], [-7, 6, 6, 2]);

        let b = a.inverse().unwrap();

        assert_eq!(
            b,
            matrix!(
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333]
            )
        );
    }

    #[test]
    fn multiple_matrix_by_inverse() {
        let a = matrix!([3, -9, 7, 3], [3, -8, 2, -9], [-4, 4, 4, 1], [-6, 5, -1, 1]);

        let b = matrix!([8, 2, 2, 2], [3, -1, 7, 0], [7, 0, 5, 4], [6, -2, 0, 5]);

        let c = a.clone() * b.clone();

        assert_eq!(c * b.inverse().unwrap(), a);
    }
}
