#[derive(Debug)]
pub struct M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy + From<i32>,
{
    elements: [[T; 4]; 4],
}

impl<T> M4<T>
where
    T: std::ops::Mul + std::ops::Add + std::marker::Copy + From<i32>,
{
    pub fn new() -> M4<T> {
        M4 {
            elements: [[T::from(0); 4]; 4],
        }
    }
}
