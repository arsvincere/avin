/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Range;

#[derive(Debug)]
pub enum Size {
    XS = 1,
    S = 2,
    M = 3,
    L = 4,
    XL = 5,
}
impl Size {
    pub fn range(&self) -> Range {
        match self {
            Self::XS => Range::new(0.0, 10.0),
            Self::S => Range::new(10.0, 30.0),
            Self::M => Range::new(30.0, 70.0),
            Self::L => Range::new(70.0, 90.0),
            Self::XL => Range::new(90.0, 100.0),
        }
    }
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::XS => write!(f, "XS"),
            Self::S => write!(f, "S"),
            Self::M => write!(f, "M"),
            Self::L => write!(f, "L"),
            Self::XL => write!(f, "XL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let s = Size::M;
        let range = s.range();

        assert_eq!(range.min(), 30.0);
        assert_eq!(range.max(), 70.0);
    }
}
