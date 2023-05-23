use std::{fmt::Display, ops::Add};
pub trait Number:
    std::ops::Mul + Add + std::marker::Copy + Default + std::fmt::Display + From<i32>
{
}
impl<
        T: std::ops::Mul
            + Add<Output = T>
            + std::marker::Copy
            + Default
            + std::fmt::Display
            + From<i32>,
    > Number for T
{
}

pub trait Diag<T: Number> {
    type Output<U>;
    fn diag(d: T) -> Self::Output<T>;
}

pub trait Eye<T: Number>: Diag<T> {
    #[inline]
    fn eye() -> Self::Output<T> {
        Self::diag(T::from(1))
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct M4<T: Number> {
    elements: [[T; 4]; 4],
}

impl<T: Number> std::ops::Index<usize> for M4<T> {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T: Number> std::ops::IndexMut<usize> for M4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T: Number> From<[[T; 4]; 4]> for M4<T> {
    fn from(value: [[T; 4]; 4]) -> Self {
        M4 { elements: value }
    }
}

impl<T: Number> Display for M4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &row in &self.elements {
            for &el in &row {
                write!(f, "[{}] ", el)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Number> Add for M4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut temp_arr = [[T::default(); 4]; 4];

        for (i, (&row, &row2)) in self.elements.iter().zip(&rhs.elements).enumerate() {
            for (j, (&el1, &el2)) in row.iter().zip(&row2).enumerate() {
                temp_arr[i][j] = el1 + el2;
            }
        }
        self.clone()
    }
}

impl<T: Number> Diag<T> for M4<T> {
    type Output<U> = M4<T>;

    fn diag(d: T) -> Self::Output<T> {
        M4 {
            elements: [
                [d, T::default(), T::default(), T::default()],
                [T::default(), d, T::default(), T::default()],
                [T::default(), T::default(), d, T::default()],
                [T::default(), T::default(), T::default(), d],
            ],
        }
    }
}

impl<T: Number> Eye<T> for M4<T> {}

impl<T: Number + std::ops::Add<Output = T>> M4<T> {
    pub fn new() -> M4<T> {
        M4 {
            elements: [[T::default(); 4]; 4],
        }
    }

    pub fn trace(&self) -> T {
        self.elements[0][0] + self.elements[1][1] + self.elements[2][2] + self.elements[3][3]
    }

    pub fn inv(&self) -> Self {
        todo!()
    }

    pub fn transpose(&self) -> Self {
        let mut t = self.clone();
        unsafe {
            std::mem::swap(
                &mut *(&mut t.elements[1][0] as *mut T),
                &mut *(&mut t.elements[0][1] as *mut T),
            );
            std::mem::swap(
                &mut *(&mut t.elements[2][0] as *mut T),
                &mut *(&mut t.elements[0][2] as *mut T),
            );
            std::mem::swap(
                &mut *(&mut t.elements[2][1] as *mut T),
                &mut *(&mut t.elements[1][2] as *mut T),
            );
            std::mem::swap(
                &mut *(&mut t.elements[3][0] as *mut T),
                &mut *(&mut t.elements[0][3] as *mut T),
            );
            std::mem::swap(
                &mut *(&mut t.elements[3][1] as *mut T),
                &mut *(&mut t.elements[1][3] as *mut T),
            );
            std::mem::swap(
                &mut *(&mut t.elements[3][2] as *mut T),
                &mut *(&mut t.elements[2][3] as *mut T),
            );
        }
        t
    }

    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        if i > 3 || j > 3 {
            return None;
        }
        Some(self.elements[i][j])
    }
}
