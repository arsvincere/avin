/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

static POW_VEC: &'static [f64] = &[
    0.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
];

pub fn round(num: f64, precision: u8) -> f64 {
    assert!(precision <= 9);

    if precision == 0 {
        num.round()
    } else {
        let multiplier = POW_VEC[precision as usize];
        let tmp_value = (num * multiplier).round().abs() as u64;

        (tmp_value as f64 / multiplier) * num.signum()
    }
}
pub fn round_price(price: f64, step: f64) -> f64 {
    let price = (price * POW_VEC[9]).round() as u64;
    let step = (step * POW_VEC[9]).round() as u64;
    let frac = price % step;

    // если дробная часть меньше половины шага цены -> trunc
    let tmp = if frac < step / 2 {
        price - price % step
    }
    // если дробная часть больше половины шага цены -> trunc + step
    else {
        price - price % step + step
    };

    tmp as f64 / POW_VEC[9]
}
pub fn max<T: PartialOrd>(left: T, right: T) -> T {
    if left > right { left } else { right }
}
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left < right { left } else { right }
}
pub fn sum<T: std::ops::Add>(
    left: T,
    right: T,
) -> <T as std::ops::Add<T>>::Output {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_round() {
        let x: f64 = 123.456789;

        assert_eq!(round(x, 1), 123.5);
        assert_eq!(round(x, 2), 123.46);
        assert_eq!(round(x, 3), 123.457);
        assert_eq!(round(x, 4), 123.4568);
        assert_eq!(round(x, 5), 123.45679);
        assert_eq!(round(x, 6), 123.456789);

        assert_eq!(round(123.9_f64, 0), 124.0);
    }

    #[test]
    fn l_round() {
        let x: f64 = 123.111111111;

        assert_eq!(round(x, 0), 123.0);
        assert_eq!(round(x, 1), 123.1);
        assert_eq!(round(x, 2), 123.11);
        assert_eq!(round(x, 3), 123.111);
        assert_eq!(round(x, 4), 123.1111);
        assert_eq!(round(x, 5), 123.11111);
        assert_eq!(round(x, 6), 123.111111);
        assert_eq!(round(x, 7), 123.1111111);
        assert_eq!(round(x, 8), 123.11111111);
        assert_eq!(round(x, 9), 123.111111111);
    }

    #[test]
    fn rounding_prices() {
        let price = 88.0;

        // округление цены до шага цены 0.01
        let step = 0.01;

        let buy = price * 0.999;
        let rounded = round_price(buy, step);
        assert_eq!(buy, 87.912);
        assert_eq!(rounded, 87.91);

        let sell = price * 1.001;
        let rounded = round_price(sell, step);
        assert_eq!(sell, 88.088);
        assert_eq!(rounded, 88.09);

        // округление цены до шага цены 0.5
        let step = 0.5;

        let buy = price * 0.99;
        let rounded = round_price(buy, step);
        assert_eq!(buy, 87.12);
        assert_eq!(rounded, 87.0);

        let sell = price * 1.01;
        let rounded = round_price(sell, step);
        assert_eq!(sell, 88.88);
        assert_eq!(rounded, 89.0);
    }

    #[test]
    fn min_max_sum() {
        assert_eq!(8, max(2, 8));
        assert_eq!(8.0, max(2.0, 8.0));

        assert_eq!(2, min(2, 8));
        assert_eq!(2.0, min(2.0, 8.0));

        assert_eq!(10, sum(2, 8));
        assert_eq!(10.0, sum(2.0, 8.0));
    }
}
