/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Bar, Chart, IID, TimeFrame, utils};

use super::extremum::Extremum;
use super::term::Term;

#[derive(Debug)]
pub struct Trend<'a> {
    e1: Extremum,
    e2: Extremum,
    chart: &'a Chart,
    bars: &'a [Bar],
}
impl<'a> Trend<'a> {
    pub fn new(e1: &Extremum, e2: &Extremum, chart: &'a Chart) -> Trend<'a> {
        assert!(e1.ts_nanos < e2.ts_nanos);

        Trend {
            e1: e1.clone(),
            e2: e2.clone(),
            chart,
            bars: chart.select(e1.ts_nanos, e2.ts_nanos),
        }
    }
    pub fn begin(&self) -> &Extremum {
        &self.e1
    }
    pub fn end(&self) -> &Extremum {
        &self.e2
    }
    pub fn tf(&self) -> &TimeFrame {
        self.chart.tf()
    }
    pub fn term(&self) -> &Term {
        return utils::min(&self.e1.term, &self.e2.term);
    }
    pub fn bars(&self) -> &[Bar] {
        self.bars
    }
    pub fn chart(&self) -> &Chart {
        self.chart
    }
    pub fn iid(&self) -> &IID {
        self.chart().iid()
    }

    pub fn is_bear(&self) -> bool {
        self.e2.price < self.e1.price
    }
    pub fn is_bull(&self) -> bool {
        self.e2.price > self.e1.price
    }
    pub fn len(&self) -> u32 {
        self.bars().len() as u32
    }
    pub fn abs(&self) -> f64 {
        (self.e2.price - self.e1.price).abs()
    }
    pub fn abs_n(&self) -> f64 {
        let abs = (self.e2.price - self.e1.price).abs();

        abs / self.e1.price
    }
    pub fn abs_p(&self) -> f64 {
        let abs = (self.e2.price - self.e1.price).abs();
        let percent = abs / self.e1.price * 100.0;

        utils::round(percent, 2)
    }
    pub fn speed(&self) -> f64 {
        let abs = (self.e2.price - self.e1.price).abs();

        abs / self.len() as f64
    }
    pub fn speed_n(&self) -> f64 {
        let abs = (self.e2.price - self.e1.price).abs();
        let abs_n = abs / self.e1.price;

        abs_n / self.len() as f64
    }
    pub fn speed_p(&self) -> f64 {
        let abs = (self.e2.price - self.e1.price).abs();
        let abs_p = abs / self.e1.price * 100.0;
        let speed_p = abs_p / self.len() as f64;

        utils::round(speed_p, 2)
    }
    pub fn vol_bear(&self) -> u64 {
        let mut bear_vol = 0;

        for bar in self.bars {
            if bar.is_bear() {
                bear_vol += bar.v;
            }
        }

        bear_vol
    }
    pub fn vol_bull(&self) -> u64 {
        let mut bull_vol = 0;

        for bar in self.bars {
            if bar.is_bull() {
                bull_vol += bar.v;
            }
        }

        bull_vol
    }
    pub fn vol_total(&self) -> u64 {
        let mut vol = 0;

        for bar in self.bars {
            vol += bar.v;
        }

        vol
    }
}
impl std::fmt::Display for Trend<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = if self.is_bull() { "+" } else { "-" };
        write!(
            f,
            "Trend: {}{} / {} = {} [ {}  {} = {}] ({} -> {})",
            kind,
            self.abs_p(),
            self.len(),
            self.speed_p(),
            self.vol_bull(),
            self.vol_bear(),
            self.vol_total(),
            self.e1.dt_local(),
            self.e2.dt_local(),
        )
    }
}
