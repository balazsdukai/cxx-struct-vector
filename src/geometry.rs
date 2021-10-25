pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

}


pub struct Line {
    pub boundaries: Vec<Point>
}

impl Line {
    pub fn new(boundaries: Vec<Point>) -> Self {
        Self { boundaries }
    }

    pub fn get_boundaries(&self) -> &Vec<Point> {
        &self.boundaries
    }

    pub fn len(&self) -> usize {
        self.boundaries.len()
    }
}
