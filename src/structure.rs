use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub trait EuclideanSpace<V: VecSpace> {
    fn dot(&self, other: &V) -> f32;
}

pub trait VecSpace
where
    Self: EuclideanSpace<Self>,
    Self: Clone + Copy,
    Self: Index<usize> + IndexMut<usize>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Mul<f32, Output = Self> + MulAssign<f32>,
    Self: Div<f32, Output = Self> + DivAssign<f32>,
    Self: Neg<Output = Self>,
{
    fn project_on(&self, on: &Self) -> Self {
        *on * (self.dot(on) / on.norm2())
    }

    fn reject_on(&self, on: &Self) -> Self {
        *self - self.project_on(on)
    }

    fn norm2(&self) -> f32 {
        self.dot(self)
    }

    fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.norm()
    }
}

pub trait Mat
where
    Self: Clone,
    Self: Index<usize, Output = Self::Column>,
    Self: IndexMut<usize, Output = Self::Column>,
{
    type Row;
    type Column;
    type Transpose: Mat<Row = Self::Column, Column = Self::Row>;
    const ZERO: Self;
    fn transpose(&self) -> Self::Transpose;
}

pub trait SquareMat
where
    Self: Mat<Row = Self::RowColumn, Column = Self::RowColumn, Transpose = Self>,
    Self: Mul<Self, Output = Self>,
    Self: Mul<Self::RowColumn, Output = Self::RowColumn>,
{
    type RowColumn;
    const IDENTITY: Self;
    fn from_diagonal(d: Self::RowColumn) -> Self;
    fn determinant(&self) -> f32;
    fn invert(&self) -> Option<Self>;
}
