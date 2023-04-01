// Auto-derive
#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct PointImpl {
    pub x: i32,
    pub y: i32,
}

impl Default for PointImpl {
    fn default() -> Self {
        PointImpl { x: 0, y: 0 }
    }
}

impl Clone for PointImpl {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for PointImpl {}

#[derive(Clone, Debug)]
pub struct Polyline {
    pub set: Vec<Point>,
}

fn main() {
    println!("{:?}", Point { x: 1, y: 2 });
    println!("{:?}", PointImpl { x: 1, y: 2 });
    println!(
        "{:?}",
        Polyline {
            set: (0..100).map(|_| Point::default()).collect()
        }
    );
}
