/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

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

        (mx - mn) / mx * 100.0
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
        (self.till - self.from) / self.from * 100.0
    }

    pub fn new(from: f64, till: f64) -> Self {
        Range { from, till }
    }
}

// class Range:
//     class Type(enum.Enum):  # {{{
//         FULL = 1
//         BODY = 2
//         UPPER = 3
//         LOWER = 4
//
//     # }}}
//
//     def __getitem__(self, slice_):  # {{{
//         """doc
//         Возвращает диапазон
//         [0:10] - от 0 до 10% исходного диапазона
//         [40:60] - от 40% до 60% исходного диапазона
//         --
//         Example:
//         bar.body[0:5] - нижние 5% тела бара
//         bar.upper[90:100] - верхние 10% верхней тени бара
//         """
//         assert isinstance(slice_, slice)
//         assert slice_.step is None
//         assert slice_.start >= 0
//         assert slice_.stop <= 100
//         assert slice_.start < slice_.stop
//
//         if slice_.start == 0:
//             start = self.__min
//         else:
//             tmp = (self.__max - self.__min) * slice_.start / 100
//             start = self.__min + tmp
//
//         if slice_.stop == 0:
//             stop = self.__max
//         else:
//             tmp = (self.__max - self.__min) * slice_.stop / 100
//             stop = self.__min + tmp
//
//         return Range(start, stop)
//
//     # }}}
//     def __contains__(self, price: float) -> bool:  # {{{
//         return self.__min <= price <= self.__max
//
//     # }}}
//     def __eq__(self, other):  # {{{
//         assert isinstance(other, Range)
//         return self.min == other.min and self.max == other.max
//
//     # }}}
//
//
//     def half(self, n) -> Range:  # {{{
//         """doc
//         Возвращает диапазон n-ой половины бара -> Range
//                #
//                ###
//                #        Это 2 половина <half2>
//                #
//         ----   #   ----
//                #
//                #        Это 1 половина <half1>
//              ###
//                #
//         """
//
//         assert n in (1, 2)
//         half = (self.__max - self.__min) / 2
//
//         if n == 1:
//             return Range(self.__min, self.__min + half)
//         # n == 2
//         return Range(self.__min + half, self.__max)
//
//     # }}}
//     def third(self, n) -> Range:  # {{{
//         """doc
//         Возвращает диапазон n-ой трети бара -> Range
//                #
//                ###      Это 3 треть
//         ----   #   ----
//                #
//                #        Это 2 треть
//         ----   #   ----
//              ###
//                #        Это 1 треть
//         """
//
//         assert n in (1, 2, 3)
//         third = (self.__max - self.__min) / 3
//
//         if n == 1:
//             return Range(self.__min, self.__min + third)
//         elif n == 2:
//             return Range(self.__min + third, self.__min + 2 * third)
//         # n == 3
//         return Range(self.__min + 2 * third, self.__max)
//
//     # }}}
//     def quarter(self, n) -> Range:  # {{{
//         """doc
//         Возвращает диапазон n-ой четверти бара -> Range
//                #
//                ###      Это 4 четверть
//         ----   #   ----
//                #        Это 3 четверть
//         ----   #   ----
//                #        Это 2 четверть
//         ----   #   ----
//              ###        Это 1 четверть
//                #
//         """
//         assert n in (1, 2, 3, 4)
//         quarter = (self.__max - self.__min) / 4
//
//         if n == 1:
//             return Range(self.__min, self.__min + quarter)
//         elif n == 2:
//             return Range(self.__min + quarter, self.__min + 2 * quarter)
//         elif n == 3:
//             return Range(self.__min + 2 * quarter, self.__min + 3 * quarter)
//         # n == 4
//         return Range(self.__min + 3 * quarter, self.__max)
//
//     # }}}

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
