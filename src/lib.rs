pub mod geometry;

use geometry::*;

type PointRs = self::Point;
type LineRs = self::Line;

#[cxx::bridge]
mod ffi {

    struct Point {
        rs: Box<PointRs>
    }
    struct Line {
        rs: Box<LineRs>
    }
    extern "Rust" {
        type PointRs;
        fn point_new(x: f32, y: f32, z: f32) -> Point;
        fn get_x(self: &Point) -> f32;
    }
    extern "Rust" {
        type LineRs;
        fn line_new(boundaries: Vec<Point>) -> Line;
        fn len(self: &Line) -> usize;
        fn get_boundaries(self: &Line) -> &Vec<Point>;
    }
}

fn point_new(x: f32, y: f32, z: f32) -> ffi::Point {
    let rs = Box::new(Point::new(x, y, z));
    ffi::Point { rs }
}

impl ffi::Point {
    fn get_x(&self) -> f32 {
        self.rs.get_x()
    }
}

fn line_new(boundaries: Vec<ffi::Point>) -> ffi::Line {
    let rs = Box::new(Line{boundaries});
    ffi::Line { rs }
}

impl ffi::Line {
    fn get_boundaries(&self) -> &Vec<ffi::Point> {
        self.rs.get_boundaries()
    }

    fn len(&self) -> usize {
        self.rs.len()
    }
}
