mod matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use matrix::*;
    #[test]
    fn creates_m4() {
        let m: M4<f64> = M4::new();
        println!("Value : {:?}", m)
    }
}
