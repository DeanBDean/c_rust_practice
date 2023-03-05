#[repr(C)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[repr(u32)]
pub enum Foo {
    A = 1,
    B,
    C
}
