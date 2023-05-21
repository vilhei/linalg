use std::{fmt::Display, primitive};

pub trait Diag<T> {
    type Output<U>;
    fn diag(d: T) -> Self::Output<T>;
}

pub trait Eye<T>: Diag<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy,
{
    fn eye() -> Self::Output<T> {
        Self::diag(1)
    }
}

pub struct M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy,
{
    elements: [[T; 4]; 4],
}

impl<T> std::ops::Index<usize> for M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy,
{
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> Diag<T> for M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy,
{
    type Output<U> = M4<T>;

    fn diag(d: T) -> Self::Output<T> {
        M4 {
            elements: [
                [d, T::from(0 as T), 0.into(), 0.into()],
                [0.into(), d, 0.into(), 0.into()],
                [0.into(), 0.into(), d, 0.into()],
                [0.into(), 0.into(), 0.into(), d],
            ],
        }
    }
}

impl<T> Eye<T> for M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy + std::fmt::Display,
{
    // type Output<U> = M4<T>;

    fn eye() -> Self::Output<T> {
        Self::diag(T::from(1))
    }
}

impl<T> Display for M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for &row in &self.elements {
            for &el in &row {
                write!(f, "[{}] ", el)?;
            }
            write!(f, "\n")?;
        })
    }
}

impl<T> M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy,
{
    pub fn new() -> M4<T> {
        M4 {
            elements: [[T::from(0); 4]; 4],
        }
    }
}
