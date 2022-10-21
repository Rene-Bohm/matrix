use super::Matrix;
use std::ops::*;

macro_rules! impl_ops {
    ($trait:ident, $fn:ident, $assigntrait:ident, $assignfn:ident) => {
        // R O
        impl<T> $trait<&Matrix<T>> for Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Matrix<T>;
            fn $fn(mut self, rhs: &Matrix<T>) -> Self::Output {
                assert_eq!(self.column, rhs.column);
                assert_eq!(self.row, rhs.row);
                for i in 0..self.entries.len() {
                    self.entries[i] = self.entries[i].$fn(rhs.entries[i]);
                }
                self
            }
        }

        // O R
        impl<T> $trait<Matrix<T>> for &Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Matrix<T>;
            fn $fn(self, rhs: Matrix<T>) -> Self::Output {
                rhs.$fn(self)
            }
        }

        // O O
        impl<T> $trait<Matrix<T>> for Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Matrix<T>;
            fn $fn(self, rhs: Matrix<T>) -> Self::Output {
                self.$fn(&rhs)
            }
        }

        // R R
        impl<T> $trait<&Matrix<T>> for &Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Matrix<T>;
            fn $fn(self, rhs: &Matrix<T>) -> Self::Output {
                self.clone().$fn(rhs)
            }
        }

        // A
        impl<T> $assigntrait<Matrix<T>> for Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            fn $assignfn(&mut self, rhs: Matrix<T>) {
                *self = (&*self).$fn(rhs)
            }
        }
        // A R
        impl<T> $assigntrait<&Matrix<T>> for Matrix<T>
        where
            T: Copy + $trait<Output = T>,
        {
            fn $assignfn(&mut self, rhs: &Matrix<T>) {
                *self = (&*self).$fn(rhs)
            }
        }
    };
}

impl_ops!(Add, add, AddAssign, add_assign);
impl_ops!(Sub, sub, SubAssign, sub_assign);
impl_ops!(Mul, mul, MulAssign, mul_assign);
impl_ops!(Div, div, DivAssign, div_assign);

impl_ops!(Rem, rem, RemAssign, rem_assign);

impl_ops!(Shl, shl, ShlAssign, shl_assign);
impl_ops!(Shr, shr, ShrAssign, shr_assign);

impl_ops!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_ops!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_ops!(BitXor, bitxor, BitXorAssign, bitxor_assign);
