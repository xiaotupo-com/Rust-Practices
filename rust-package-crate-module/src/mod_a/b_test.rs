
#[derive(Debug)]
pub struct Color {
    pub r: i8,
    pub g: i8,
    pub b: i8,
}

impl Color {
    pub fn new(r: i8, g: i8, b: i8) -> Self {
        Self {
            r: r,
            g: g,
            b: b
        }
    }

    pub fn info(&self) {
        println!("R: {}, G: {}, B: {}", self.r, self.g, self.b);
    }
}

pub fn test_sub(a: i32, b: i32) -> i32 {
    a - b
}
