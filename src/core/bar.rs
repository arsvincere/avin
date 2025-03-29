use crate::core::range::Range;
use chrono::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Bar {
    pub dt: DateTime<Utc>,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: u64,
}
impl Bar {
    pub fn display(&self) -> String {
        format!(
            "Bar: dt={} o={} h={} l={} c={} v={}",
            self.dt, self.o, self.h, self.l, self.c, self.v
        )
    }

    pub fn is_bear(&self) -> bool {
        self.o > self.c
    }
    pub fn is_bull(&self) -> bool {
        self.o < self.c
    }
    pub fn full(&self) -> Range {
        Range::new(self.l, self.h)
    }
    pub fn body(&self) -> Range {
        Range::new(self.o, self.c)
    }
    pub fn lower(&self) -> Range {
        if self.is_bull() {
            Range::new(self.l, self.o)
        } else {
            Range::new(self.l, self.c)
        }
    }
    pub fn upper(&self) -> Range {
        if self.is_bull() {
            Range::new(self.c, self.h)
        } else {
            Range::new(self.o, self.h)
        }
    }

    pub fn new(
        dt: DateTime<Utc>,
        o: f64,
        h: f64,
        l: f64,
        c: f64,
        v: u64,
    ) -> Result<Bar, Box<dyn Error>> {
        let bar = Bar { dt, o, h, l, c, v };
        Ok(bar)
    }
}

// class Bar:
//     class Type(enum.Flag):  # {{{
//         UNDEFINE = 0
//         # BEAR = 1
//         # BULL = 2
//         INSIDE = 4
//         OVERFLOW = 8
//         OUTSIDE = 16
//         EXTREMUM = 32
//
//     # }}}
//
//     def __contains__(self, price: float) -> bool:  # {{{
//         return self.low <= price <= self.high
//
//     # }}}
//
//     @property  # data  # {{{
//     def data(self) -> dict:
//         return self.__data
//
//     # }}}
//
//     def addFlag(self, flag: Bar.Type) -> None:  # {{{
//         assert isinstance(flag, Bar.Type)
//         self.__flags |= flag
//
//     # }}}
//     def delFlag(self, flag: Bar.Type) -> None:  # {{{
//         assert isinstance(flag, Bar.Type)
//         self.__flags &= ~flag
//
//     # }}}
//     def isInside(self) -> bool:  # {{{
//         return self.__flags & Bar.Type.INSIDE == Bar.Type.INSIDE
//
//     # }}}
//     def isOverflow(self) -> bool:  # {{{
//         return self.__flags & Bar.Type.OVERFLOW == Bar.Type.OVERFLOW
//
//     # }}}
//     def isOutside(self) -> bool:  # {{{
//         return self.__flags & Bar.Type.OUTSIDE == Bar.Type.OUTSIDE
//
//     # }}}
//     def isExtremum(self) -> bool:  # {{{
//         return self.__flags & Bar.Type.EXTREMUM == Bar.Type.EXTREMUM
//
//     # }}}
//
//     def to_df(self):  # {{{
//         return pl.DataFrame(self.__data)
//
//     # }}}
//
//     # }}}
//     @classmethod  # fromRecord  # {{{
//     def fromRecord(cls, record: asyncpg.Record, chart=None):
//         bar = cls(dict(record), chart)
//         return bar
//
//     # }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohlcv() {
        let dt = Utc::now();
        let b = Bar::new(dt, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
        assert_eq!(b.dt, dt);
        assert_eq!(b.o, 10.0);
        assert_eq!(b.h, 11.1);
        assert_eq!(b.l, 9.9);
        assert_eq!(b.c, 10.5);
        assert_eq!(b.v, 5000);
    }
    #[test]
    fn bear_bull() {
        let dt = Utc::now();
        let b = Bar::new(dt, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
        assert!(b.is_bull());
        assert!(!b.is_bear());

        let b = Bar::new(dt, 10.0, 11.1, 9.0, 9.5, 5000).unwrap();
        assert!(!b.is_bull());
        assert!(b.is_bear());
    }
}
