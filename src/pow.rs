pub trait Pow {
    fn pow(&self, exp: i32) -> Self;
}

impl Pow for f32 {
    fn pow(&self, exp: i32) -> Self {
        self.powi(exp)
    }
}