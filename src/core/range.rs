/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::utils;

#[derive(Debug)]
pub struct Range {
    pub from: f64,
    pub till: f64,
}
impl Range {
    pub fn min(&self) -> f64 {
        if self.from < self.till {
            return self.from;
        } else {
            return self.till;
        }
    }
    pub fn max(&self) -> f64 {
        if self.from > self.till {
            return self.from;
        } else {
            return self.till;
        }
    }
    pub fn mid(&self) -> f64 {
        let min = self.min();
        let max = self.max();
        let half = (max - min) / 2.0;

        min + half
    }

    /// abs of range
    pub fn abs(&self) -> f64 {
        self.max() - self.min()
    }
    /// normalized abs of range
    pub fn abs_n(&self) -> f64 {
        let mn = self.min();
        let mx = self.max();

        (mx - mn) / mx
    }
    /// percent abs of range
    pub fn abs_p(&self) -> f64 {
        let mn = self.min();
        let mx = self.max();

        let value = (mx - mn) / mx * 100.0;

        utils::round(value, 2)
    }
    /// delta
    pub fn delta(&self) -> f64 {
        self.till - self.from
    }
    /// normalized delta
    pub fn delta_n(&self) -> f64 {
        (self.till - self.from) / self.from
    }
    /// percent delta
    pub fn delta_p(&self) -> f64 {
        let value = (self.till - self.from) / self.from * 100.0;

        utils::round(value, 2)
    }

    pub fn new(from: f64, till: f64) -> Self {
        Range { from, till }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.max(), 110.0);
    }
    #[test]
    fn min() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.min(), 100.0);
    }
    #[test]
    fn mid() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.mid(), 105.0);
    }
    #[test]
    fn abs() {
        let r = Range::new(5000.0, 4000.0);
        assert_eq!(r.abs(), 1000.0);
        assert_eq!(r.abs_n(), 0.2);
        assert_eq!(r.abs_p(), 20.0);
    }
    #[test]
    fn delta() {
        let r = Range::new(5000.0, 4000.0);
        assert_eq!(r.delta(), -1000.0);
        assert_eq!(r.delta_n(), -0.2);
        assert_eq!(r.delta_p(), -20.0);
    }
}
