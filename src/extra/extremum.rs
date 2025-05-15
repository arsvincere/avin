/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local, NaiveDateTime, Utc};

use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtremumKind {
    Max,
    Min,
}
impl std::fmt::Display for ExtremumKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Max => write!(f, ""),
            Self::Min => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Extremum {
    pub ts_nanos: i64,
    pub term: Term,
    pub kind: ExtremumKind,
    pub price: f64,
}
impl Extremum {
    pub fn new(
        ts_nanos: i64,
        term: Term,
        kind: ExtremumKind,
        price: f64,
    ) -> Self {
        Self {
            ts_nanos,
            term,
            kind,
            price,
        }
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts_nanos);
        let local: DateTime<Local> = DateTime::from(utc);

        local.naive_local()
    }
    pub fn is_min(&self) -> bool {
        self.kind == ExtremumKind::Min
    }
    pub fn is_max(&self) -> bool {
        self.kind == ExtremumKind::Max
    }
    pub fn is_t1(&self) -> bool {
        self.term == Term::T1
    }
    pub fn is_t2(&self) -> bool {
        self.term == Term::T2
    }
    pub fn is_t3(&self) -> bool {
        self.term == Term::T3
    }
    pub fn is_t4(&self) -> bool {
        self.term == Term::T4
    }
    pub fn is_t5(&self) -> bool {
        self.term == Term::T5
    }
}
impl std::fmt::Display for Extremum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Extremum={} {} {} {}",
            self.dt_local(),
            self.term,
            self.kind,
            self.price
        )
    }
}
