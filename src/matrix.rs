use std::fmt::Display;
pub trait Number:
    std::ops::Mul + std::ops::Add + std::marker::Copy + Default + std::fmt::Display + From<i32>
{
}
impl<
        T: std::ops::Mul + std::ops::Add + std::marker::Copy + Default + std::fmt::Display + From<i32>,
    > Number for T
{
}

pub trait Diag<T: Number> {
    type Output<U>;
    fn diag(d: T) -> Self::Output<T>;
}

pub trait Eye<T: Number>: Diag<T> {
    fn eye() -> Self::Output<T> {
        Self::diag(T::from(1))
    }
}

#[derive(Default)]
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

impl<T: Number> Eye<T> for M4<T> {
    fn eye() -> Self::Output<T> {
        Self::diag(T::from(1))
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
        todo!()
    }

    /// .
    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        if i > 3 || j > 3 {
            return None;
        }
        Some(self.elements[i][j])
    }
}
