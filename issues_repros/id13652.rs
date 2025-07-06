//ISSUE #13652 - C-bug, I-false-positive
fn main() {}

impl<T: Colorspace> core::ops::Div<f32> for PremulColor<T> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * rhs.recip()
    }
}
