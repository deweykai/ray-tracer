use crate::tuple::Tuple;

type MatrixData<const W: usize, const H: usize> = [[f64; W]; H];

#[derive(Debug, Clone)]
pub struct Matrix<const W: usize, const H: usize> {
    data: MatrixData<W, H>,
}

pub type SquareMatrix<const D: usize> = Matrix<D, D>;
pub type Matrix4 = SquareMatrix<4>;

impl<const W: usize, const H: usize> Matrix<W, H> {
    pub fn new(data: MatrixData<W, H>) -> Matrix<W, H> {
        Matrix { data }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.data[x][y]
    }

    pub fn transpose(&self) -> Matrix<H, W> {
        let data = (0..W)
            .map(|x| {
                (0..H)
                    .map(|y| self.data[y][x])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Matrix::new(data.try_into().unwrap())
    }
}

impl<const D: usize> SquareMatrix<D> {
    pub fn identity(size: usize) -> SquareMatrix<D> {
        let data = (0..D)
            .map(|y| {
                (0..size)
                    .map(|x| if x == y { 1.0 } else { 0.0 })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Matrix::new(data.try_into().unwrap())
    }
}

impl SquareMatrix<2> {
    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

macro_rules! inverse_matrix_ops {
    ($($D:literal)*) => ($(
        impl SquareMatrix<$D> {
            pub fn determinant(&self) -> f64 {
                let mut sum = 0.;
                for i in 0..$D {
                    sum += self.data[0][i] * self.cofactor(0, i);
                }
                sum
            }
            pub fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<{$D -1}> {
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
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                Matrix::new(data.try_into().unwrap())
            }

            pub fn minor(&self, row: usize, col: usize) -> f64 {
                self.submatrix(row, col).determinant()
            }

            pub fn cofactor(&self, row: usize, col: usize) -> f64 {
                self.minor(row, col) * if (row + col) % 2 == 1 { -1. } else { 1. }
            }

            pub fn invertible(&self) -> bool {
                let det = self.determinant();
                if det != 0. {
                    true
                } else {
                    false
                }
            }

            pub fn inverse(&self) -> Result<SquareMatrix<$D>, &'static str> {
                if !self.invertible() {
                    return Err("matrix not invertible");
                }

                let cofactors = Matrix::new(
                    (0..$D)
                        .map(|y| {
                            (0..$D)
                                .map(|x| self.cofactor(y, x))
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );

                let det = self.determinant();

                Ok(cofactors.transpose() / det)
            }
        }
    )*)
}

inverse_matrix_ops!( 4 3 );

#[macro_export]
macro_rules! matrix {
    ($([$($x:expr),+ $(,)?]),+ $(,)?) => {
        $crate::matrix::Matrix::new([$(
            [
                $( $x as f64, )+
            ],
        )+])
    }
}

impl<const W: usize, const H: usize> PartialEq for Matrix<W, H> {
    fn eq(&self, other: &Matrix<W, H>) -> bool {
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

impl<const W: usize, const H: usize> Index<(usize, usize)> for Matrix<W, H> {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &f64 {
        let (x, y) = index;
        &self.data[x][y]
    }
}

use std::ops::{Div, Mul};

impl<const W: usize, const H: usize, const L: usize> Mul<Matrix<H, L>> for Matrix<W, H> {
    // TODO: make matrix use references
    type Output = Matrix<W, L>;

    fn mul(self, rhs: Matrix<H, L>) -> Self::Output {
        let width = W;
        let height = L;

        let data = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let mut sum: f64 = 0.0;
                        for i in 0..W {
                            sum += self.data[y][i] * rhs.data[i][x];
                        }
                        sum
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Matrix::<W, L>::new(data)
    }
}

impl<const W: usize, const H: usize> Mul<f64> for Matrix<W, H> {
    type Output = Matrix<W, H>;
    fn mul(self, rhs: f64) -> Self::Output {
        let data = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x * rhs)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Matrix::new(data)
    }
}

impl<const W: usize, const H: usize> Div<f64> for Matrix<W, H> {
    type Output = Matrix<W, H>;
    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.recip()
    }
}

impl<T> Mul<T> for &SquareMatrix<4>
where
    T: Into<Tuple>,
{
    type Output = Tuple;
    fn mul(self, t: T) -> Tuple {
        let t = t.into();

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

impl<T> Mul<T> for SquareMatrix<4>
where
    T: Into<Tuple>,
{
    type Output = Tuple;
    fn mul(self, t: T) -> Tuple {
        (&self) * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_4x4_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
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
        let m = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }
    #[test]
    fn construxt_2x2_matrix() {
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }
    #[test]
    fn compare_similar_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(a, b);
    }
    #[test]
    fn compare_disimilar_matrix() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }
    #[test]
    fn multiply_two_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let result = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, result);
    }
    #[test]
    fn multiply_matrix_by_tuple() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(a * b, result);
    }
    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
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
        let a = SquareMatrix::<4>::identity(4);
        assert_eq!(a.transpose(), a);
    }

    #[test]
    fn determinant_2x2_matrix() {
        let a = matrix!([1, 5], [-3, 2]);
        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        let a = matrix!([1, 5, 0], [-3, 2, 7], [0, 6, -3]);
        assert_eq!(a.submatrix(0, 2), matrix!([-3, 2], [0, 6]));
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        let a = matrix!([-6, 1, 1, 6], [-8, 5, 8, 6], [-1, 0, 8, 2], [-7, 1, -1, 1]);
        let b = matrix!([-6, 1, 6], [-8, 8, 6], [-7, -1, 1]);

        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn calculate_minor_of_3x3_matrix() {
        let a = matrix!([3, 5, 0], [2, -1, 7], [6, -1, 5]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn calculate_cofactor_of_3x3_matrix() {
        let a = matrix!([3, 5, 0], [2, -1, -7], [6, -1, 5]);

        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn determinant_3x3_matrix() {
        let a = matrix!([1, 2, 6], [-5, 8, -4], [2, 6, 4]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn determinant_4x4_matrix() {
        let a = matrix!([-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]);
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn test_invertible_matrix_invertibility() {
        let a = matrix!([6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]);
        assert_eq!(a.determinant(), -2120.);
        assert!(a.invertible());
    }

    #[test]
    fn test_noninvertible_matrix_invertibility() {
        let a = matrix!([-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]);
        assert_eq!(a.determinant(), 0.);
        assert!(!a.invertible());
    }

    #[test]
    fn calculate_inverse_4x4_matrix() {
        // test 1
        let a = matrix!([-5, 2, 6, -8], [1, -5, 1, 8], [7, 7, -6, -7], [1, -3, 7, 4]);
        let b = a.inverse().unwrap();

        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b.get(3, 2), -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
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
