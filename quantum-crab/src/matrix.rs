//! Implements matrix data structure.

use crate::complex::Complex;
use core::fmt;
use num::{One, Zero};
use std::{
    fmt::{Debug, Display, Write},
    ops::{Add, Mul},
};

/// Represents matrix data structure.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone + Default + Debug> Matrix<T> {
    /// Constructs a new matrix with elements being initialized using
    /// [`Default::default()`].
    pub fn new_with_default_elems(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: vec![Default::default(); rows * cols],
        }
    }

    /// Constructs a new matrix and populate it with the contents.
    ///
    /// ```
    /// use quantum_crab::matrix::Matrix;
    ///
    /// let matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]);
    /// assert_eq!(matrix.get(0, 0), 1);
    /// assert_eq!(matrix.get(1, 0), 3);
    /// ```
    pub fn new(rows: usize, cols: usize, contents: Vec<T>) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: contents,
        }
    }

    /// Gets the element in a given `row` and `col`.
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col].clone()
    }

    /// Sets the element in a given `row` and `col` to have value `value`.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    /// Amount of rows in the matrix contents.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Amount of columns in the matrix contents.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Transposes the matrix.
    ///
    /// ```
    /// use quantum_crab::matrix::Matrix;
    ///
    /// let matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]);
    /// assert_eq!(matrix.transpose(), Matrix::new(2, 2, vec![1, 3, 2, 4]));
    /// ```
    pub fn transpose(&self) -> Matrix<T> {
        let mut result = Matrix::new_with_default_elems(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }

        result
    }

    /// Calculates the dot product.
    pub fn dot_product(&self, rhs: &Matrix<T>) -> Matrix<T>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        assert_eq!(self.cols(), rhs.rows());

        let mut result = Matrix::new_with_default_elems(self.rows, rhs.cols());

        for i in 0..self.rows {
            for j in 0..rhs.cols() {
                let mut sum = Default::default();

                for k in 0..self.cols {
                    sum = sum + self.get(i, k) * rhs.get(k, j);
                }

                result.set(i, j, sum);
            }
        }

        result
    }

    /// Calculates the tensor product.
    pub fn tensor_product(&self, other: &Matrix<T>) -> Matrix<T>
    where
        T: Mul<Output = T>,
    {
        let mut result =
            Matrix::new_with_default_elems(self.rows() * other.rows(), self.cols() * other.cols());

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                for k in 0..other.rows() {
                    for l in 0..other.cols() {
                        let value = self.get(i, j).clone() * other.get(k, l).clone();
                        result.set(i * other.rows() + k, j * other.cols() + l, value);
                    }
                }
            }
        }

        result
    }

    /// Embeds the matrix into row `i`, column `j` (these are the coordinates of the beginning
    /// of given matrix).
    pub fn embed(&mut self, matrix: &Matrix<T>, i: usize, j: usize) {
        assert!(i + matrix.rows() <= self.rows());
        assert!(j + matrix.cols() <= self.cols());

        for delta_i in 0..matrix.rows() {
            for delta_j in 0..matrix.cols() {
                self.set(i + delta_i, j + delta_j, matrix.get(delta_i, delta_j));
            }
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display + Clone + Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[\n")?;

        for i in 0..self.rows {
            f.write_char('\t')?;
            for j in 0..self.cols {
                write!(f, "{} ", self.get(i, j))?;
            }
            f.write_char('\n')?;
        }
        f.write_str("]\n")?;

        Ok(())
    }
}

impl Matrix<Complex> {
    /// Calculates the hermitian transpose (can also be referred to as conjugate transpose) of
    /// the given matrix.
    ///
    /// It just takes every element in the matrix, and calculates its complex conjugate.
    /// Then transposes the matrix.
    ///
    /// ```
    /// use quantum_crab::{
    ///   matrix,
    ///   matrix::Matrix,
    ///   complex::Complex
    /// };
    ///
    /// let matrix = matrix![[Complex::new(1, 1), Complex::new(1, -1)]];
    /// assert_eq!(
    ///   matrix.hermitian_transpose(),
    ///   matrix![
    ///     [Complex::new(1, -1)],
    ///     [Complex::new(1, 1)]
    ///   ]
    /// );
    /// ```
    pub fn hermitian_transpose(&self) -> Self {
        let mut result = Matrix::new_with_default_elems(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j).conjugate());
            }
        }

        result
    }
}

impl<T> Matrix<T>
where
    T: Clone + Default + Mul<Output = T> + PartialEq + One + Debug,
{
    /// Identity matrix, with size being the amount of columns/rows.
    pub fn identity(size: usize) -> Matrix<T> {
        let mut result = Matrix::new_with_default_elems(size, size);
        for i in 0..size {
            result.set(i, i, T::one());
        }
        result
    }
}

impl<'a, T> Add<&'a Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + Clone + Default + Debug,
{
    type Output = Matrix<T>;

    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows(), other.rows());
        assert_eq!(self.cols(), other.cols());

        let mut result = Matrix::new_with_default_elems(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) + other.get(i, j));
            }
        }

        result
    }
}

impl<'a, T> Mul<&'a Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone + Default + Debug,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'a Matrix<T>) -> Matrix<T> {
        self.dot_product(rhs)
    }
}

impl<'a, T> Mul<T> for Matrix<T>
where
    T: Mul<Output = T> + Clone + Default + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Matrix<T> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|e| *e * rhs).collect(),
        }
    }
}

#[macro_export]
macro_rules! m_one {
    ( $item:tt ) => {
        1
    };
}

#[macro_export]
macro_rules! m_rec {
    [[ $($row:tt),* ] [$($i:expr),*]] => ({
        let _rows = 0 $(+ $crate::m_one!($row) )*;
        let _cols = (0 $(+ $crate::m_one!($i))*) / _rows;
        $crate::matrix::Matrix::new(
            _rows,
            _cols,
            vec![$($i),*]
        )
    })
}

/// Macro used to construct a matrix.
///
/// # Example
/// ```
/// use quantum_crab::matrix;
///
/// let matrix = matrix![[1, 2], [3, 4], [5, 6]];
/// assert_eq!(matrix.get(1, 2), 5);
/// ```
#[macro_export]
macro_rules! matrix {
    ($([$( $i:expr ),*]),*) => ( $crate::m_rec!([$([$($i),*]),*] [$($($i),*),*]) )
}

#[macro_export]
macro_rules! matrix_real {
    ($([$( $i:expr ),*]),*) => ( $crate::m_rec!([$([$($crate::complex::Complex::from($i)),*]),*]
        [$($($crate::complex::Complex::from($i)),*),*]) )
}

#[test]
fn test_tensor_product() {
    let m1 = matrix![[1, 2], [3, 4]];
    let m2 = matrix![[5, 6], [7, 8]];
    let tensor_product = m1.tensor_product(&m2);
    let expected = matrix![
        [5, 6, 10, 12],
        [7, 8, 14, 16],
        [15, 18, 20, 24],
        [21, 24, 28, 32]
    ];
    assert_eq!(tensor_product, expected);
}

#[test]
fn test_dot_product() {
    let m1 = matrix![[1, 2], [3, 4]];
    let m2 = matrix![[5, 6], [7, 8]];
    let dot_product = m1.dot_product(&m2);
    let expected = matrix![[19, 22], [43, 50]];
    assert_eq!(dot_product, expected);
}

#[test]
fn test_dot_product2() {
    let ket0 = matrix![[1], [0]];
    let pauli_x = matrix![[0, 1], [1, 0]];
    let ket1 = matrix![[0], [1]];

    assert_eq!(pauli_x.dot_product(&ket0), ket1);
}

#[test]
fn test_matrix_addition() {
    let m1 = matrix![[1, 2], [3, 4]];
    let m2 = matrix![[5, 6], [7, 8]];
    let sum = m1.add(&m2);
    let expected = matrix![[6, 8], [10, 12]];
    assert_eq!(sum, expected);
}
