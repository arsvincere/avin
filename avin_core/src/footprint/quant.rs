/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Direction, Tic};

/// Metrics calculated on tics grouped by price.
///
/// # ru
/// Метрики рассчитанные на тиках по одной конкретной цене.
#[derive(Debug, Clone)]
pub struct Quant {
    pub price: f64,
    pub vol_b: u64,
    pub vol_s: u64,
    pub val_b: f64,
    pub val_s: f64,
    pub cdf_b: Option<f64>,
    pub cdf_s: Option<f64>,
}
impl Quant {
    pub fn new(price: f64) -> Self {
        Self {
            price,
            val_b: 0.0,
            val_s: 0.0,
            vol_b: 0,
            vol_s: 0,
            cdf_b: None,
            cdf_s: None,
        }
    }

    pub fn add(&mut self, tic: &Tic) {
        assert!(self.price == tic.price);

        if tic.direction == Direction::Buy {
            self.vol_b += tic.lots as u64;
            self.val_b += tic.value;
        } else {
            self.vol_s += tic.lots as u64;
            self.val_s += tic.value;
        }
    }
    pub fn vol(&self) -> u64 {
        self.vol_b + self.vol_s
    }
    pub fn val(&self) -> f64 {
        self.val_b + self.val_s
    }
    pub fn cdf_b(&self) -> Option<f64> {
        self.cdf_b
    }
    pub fn cdf_s(&self) -> Option<f64> {
        self.cdf_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quant() {
        let price = 320.5;

        let mut quant = Quant::new(price);
        assert_eq!(quant.price, price);
        assert_eq!(quant.val_b, 0.0);
        assert_eq!(quant.val_s, 0.0);

        let b = Tic::new(100500, Direction::Buy, 1, 320.5, 320.5);
        let s = Tic::new(100500, Direction::Sell, 1, 320.5, 320.5);
        quant.add(&b);
        quant.add(&s);
        quant.add(&s);
        assert_eq!(quant.price, price);
        assert_eq!(quant.vol_b, 1);
        assert_eq!(quant.vol_s, 2);
        assert_eq!(quant.val_b, 320.5);
        assert_eq!(quant.val_s, 641.0);
        assert_eq!(quant.vol(), 3);
        assert_eq!(quant.val(), 320.5 + 641.0);

        quant.add(&b);
        assert_eq!(quant.price, price);
        assert_eq!(quant.vol_b, 2);
        assert_eq!(quant.vol_s, 2);
        assert_eq!(quant.val_b, 641.0);
        assert_eq!(quant.val_s, 641.0);
        assert_eq!(quant.vol(), 4);
        assert_eq!(quant.val(), 641.0 + 641.0);
    }
}
