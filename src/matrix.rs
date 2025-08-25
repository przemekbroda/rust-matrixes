use std::fmt::Formatter;
use std::num::NonZero;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
use crate::one::One;
use crate::zero::Zero;

#[derive(Debug, Clone)]
pub struct MatrixError {
    message: String,
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone)]
pub struct Matrix<T: Element> {
    v: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

pub trait Element: Copy + Add<Output = Self> + AddAssign + Sub<Output = Self> + Mul<Output = Self> + MulAssign + Zero + One + Send + Sync {}
impl<T: Copy + Add<Output=T> + AddAssign + Sub<Output=T> + Mul<Output=T> + MulAssign + Zero + One + Send + Sync> Element for T {}

impl<T: Element> Matrix<T> {
    pub fn new(mut v: Vec<Vec<T>>) -> Matrix<T> {
        let rows = v.len();

        let mut cols = 0;
        for row in &v {
            if row.len() > cols {
                cols = row.len();
            }
        }

        for row in &mut v {
            row.resize(cols, T::zero());
        }

        Matrix {
            v,
            rows,
            cols
        }
    }

    pub fn new_with_zeros(rows: usize, cols: usize) -> Matrix<T> {
        let v = vec![vec![T::zero(); cols]; rows];

        Matrix {
            v,
            rows,
            cols
        }
    }

    pub fn new_with_values(rows: usize, cols: usize, value: T) -> Matrix<T> {
        let v = vec![vec![value; cols]; rows];

        Matrix {
            v,
            rows,
            cols
        }
    }

    pub fn identity_matrix(size: usize) -> Matrix<T> {
        let v = vec![vec![T::one(); size]; size];

        Matrix {
            v,
            rows: size,
            cols: size
        }
    }

    pub fn can_add_to(&self, rhs: &Matrix<T>) -> bool {
        same_size(&self, rhs)
    }

    pub fn can_subtract_from(&self, rhs: &Matrix<T>) -> bool {
        same_size(&self, rhs)
    }

    pub fn can_multiply_by(&self, rhs: &Matrix<T>) -> bool {
        self.cols == rhs.rows
    }

    pub fn add_to(self, rhs: Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        self.add(rhs)
    }

    pub fn subtract_from(self, rhs: Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        self.sub(rhs)
    }

    pub fn multiply_by(self, rhs: Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        self.mul(rhs)
    }

    pub fn multiply_by_scalar(&self, scalar: T) -> Matrix<T> {
        /* This one is about 100 ms slower than the one below in case of 10000x10000 matrices
        let result = self.v
            .iter()
            .map(|r| r
                .iter()
                .map(|n| *n * scalar)
                .collect())
            .collect();
         */

        let mut result = self.v.clone();

        for row in &mut result {
            for col in row {
                *col *= scalar;
            }
        }

        Matrix {
            v: result,
            rows: self.rows,
            cols: self.cols
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: T) -> Result<(), MatrixError> {
        if row < self.rows && col < self.cols {
            self.v[row][col] = value;
            Ok(())
        } else {
            Err(MatrixError { message: "Row or column out of bounds".to_string() })
        }
    }

    pub fn set_value_with_modifier_function(&mut self, row: usize, col: usize, modifier: fn (T) -> T) -> Result<(), MatrixError> {
        if row < self.rows && col < self.cols {
            self.v[row][col] = modifier(self.v[row][col]);
            Ok(())
        } else {
            Err(MatrixError { message: "Row or column out of bounds".to_string() })
        }
    }

    pub fn grow_or_shrink(&mut self, rows: usize, cols: usize) {
        if self.rows == rows && self.cols == cols {
            return;
        }

        if self.rows != rows {
            self.v.resize(rows, vec![T::zero(); cols]);
        }
        if self.cols != cols {
            for row in &mut self.v {
                row.resize(cols, T::zero());
            }
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<T, MatrixError> {
        if row < self.rows && col < self.cols {
            Ok(self.v[row][col])
        } else {
            Err(MatrixError { message: "Row or column out of bounds".to_string() })
        }
    }

}

impl<T: Element> Add for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: Self) -> Self::Output {
        if !same_size(&self, &rhs) {
            return Err(MatrixError {
                message: "Matrices must be the same size".to_string()
            })
        }

        let mut addition_result = vec![vec![T::zero(); self.cols]; self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                addition_result[row][col] = self.v[row][col] + rhs.v[row][col];
            }
        }

        Ok(Self {
            v: addition_result,
            rows: self.rows,
            cols: self.cols
        })
    }
}

impl<T: Element> Sub for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !same_size(&self, &rhs) {
            return Err(MatrixError {
                message: "Matrices must be the same size".to_string()
            })
        }

        let mut substraction_result = vec![vec![T::zero(); self.cols]; self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                substraction_result[row][col] = self.v[row][col] - rhs.v[row][col]
            }
        }

        Ok(Self {
            v: substraction_result,
            rows: self.rows,
            cols: self.cols
        })
    }
}

impl<T: Element> Mul for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            return Err(MatrixError {
                message: "Matrices must be the same size".to_string()
            })
        }

        let mut mut_result = vec![vec![T::zero(); rhs.cols]; self.rows];

        for col_v2 in 0..rhs.cols {
            for row_v1 in 0..self.rows {
                for col_v1 in 0..self.cols {
                    mut_result[row_v1][col_v2] += self.v[row_v1][col_v1] * rhs.v[col_v1][col_v2];
                }
            }
        }

        Ok(Self {
            v: mut_result,
            rows: self.rows,
            cols: rhs.cols
        })
    }
}

pub fn same_size<T: Element>(m1: &Matrix<T>, m2: &Matrix<T>) -> bool {
    m1.rows == m2.rows && m1.cols == m2.cols
}

impl<T> std::fmt::Display for Matrix<T> where T: Element + std::fmt::Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.v {
            write!(f, "[")?;

            let mut first = true;

            for col in row {
                let d = match first {
                    true => {" "}
                    false => {", "}
                };

                write!(f, "{}{}", d, col)?;
                first = false;
            }
            writeln!(f, " ]")?;
        }

        Ok(())
    }
}

pub trait AddMultiThreaded<Rhs = Self> {
    type Output;
    fn add_multithreaded(&self, rhs: &Rhs, threads: NonZero<usize>) -> Self::Output;
}

impl<T: Element> AddMultiThreaded for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn add_multithreaded(&self, rhs: &Self, threads: NonZero<usize>) -> Self::Output {
        if !same_size(&self, &rhs) {
            return Err(MatrixError {
                message: "Matrices must be the same size".to_string()
            })
        }

        let result = multithreaded_operation(&self, &rhs, threads, |a, b| a + b);
        Ok(result)
    }
}

pub trait SubMultiThreaded<Rhs = Self> {
    type Output;
    fn sub_multithreaded(&self, rhs: &Rhs, threads: NonZero<usize>) -> Self::Output;
}

impl<T: Element> SubMultiThreaded for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub_multithreaded(&self, rhs: &Self, threads: NonZero<usize>) -> Self::Output {
        if !same_size(&self, &rhs) {
            return Err(MatrixError {
                message: "Matrices must be the same size".to_string()
            })
        }

        let matrix = multithreaded_operation(&self, &rhs, threads, |a, b| a - b);
        Ok(matrix)
    }
}

fn multithreaded_operation<T: Element>(matrix1: &Matrix<T>, matrix2: &Matrix<T>, threads: NonZero<usize>, operation: fn (T, T) -> T) -> Matrix<T> {
    let chunk_size = matrix1.rows / threads + 1;
    let mut result = vec![vec![T::zero(); matrix1.cols]; matrix1.rows];

    let rows_v1 = matrix1.v.chunks(chunk_size);
    let rows_v2 = matrix2.v.chunks(chunk_size);

    std::thread::scope(|scope| {
        for (result_chunk, (self_chunk, rhs_chunk)) in result.chunks_mut(chunk_size).into_iter().zip(rows_v1.zip(rows_v2)) {
            scope.spawn(move || {
                for row in 0..result_chunk.len() {
                    for col in 0..result_chunk[row].len() {
                        result_chunk[row][col] = operation(self_chunk[row][col], rhs_chunk[row][col]);
                    }
                }
            });
        }
    });

    Matrix {
        v: result,
        rows: matrix1.rows,
        cols: matrix1.cols
    }
}

fn operation<T: Element>(value1: T, value2: T, operation: fn(T, T) -> T) -> T {
    operation(value1, value2)
}