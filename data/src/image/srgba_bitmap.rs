#[derive(Eq, PartialEq, Clone, Hash)]
pub struct SrgbaBitmap {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
}
