pub fn print_sth(s: String) {
    println!("{}", s);
}

#[derive(Debug)]
pub enum DivError {
    DivZero,
}

pub fn div(f1: f32, f2: f32) -> Result<f32, DivError> {
    if f2 == 0. {
        return Err(DivError::DivZero);
    }
    let result = f1 / f2;
    Ok(result)
}

pub fn multi(a: usize, b: usize) -> usize {
    a.checked_mul(b).unwrap_or_default()
}
