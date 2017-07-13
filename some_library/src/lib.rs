
#[derive(Debug)]
pub enum Shape {
    Circle { x: u32, y: u32, radius: u32},
    Rect { x: u32, y: u32, width: u32, height: u32}
}