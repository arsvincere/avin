/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

pub enum Item {
    Point(Point),
    Line(Line),
    Rect(Rect),
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub struct Line {
    pub a: Point,
    pub b: Point,
}
impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

pub struct Rect {
    pub o: Point,
    pub w: f64,
    pub h: f64,
}
impl Rect {
    pub fn new(o: Point, width: f64, height: f64) -> Self {
        Self {
            o,
            w: width,
            h: height,
        }
    }
}
