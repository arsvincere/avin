/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

pub enum GItem {
    GPoint(Point),
    GLine(Line),
    GRect(Rect),
}

pub struct GPoint {
    pub x: f64,
    pub y: f64,
}
impl GPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub struct GLine {
    pub a: Point,
    pub b: Point,
}
impl GLine {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

pub struct GRect {
    pub o: Point,
    pub w: f64,
    pub h: f64,
}
impl GRect {
    pub fn new(o: Point, width: f64, height: f64) -> Self {
        Self {
            o,
            w: width,
            h: height,
        }
    }
}
