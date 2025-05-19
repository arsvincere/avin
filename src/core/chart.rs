/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::DateTime;
use chrono::Utc;

use crate::data::IID;
use crate::data::Manager;
use crate::extra::Extremum;
use crate::extra::ExtremumKind;
use crate::extra::Term;
use crate::extra::Trend;

use super::bar::Bar;
use super::timeframe::TimeFrame;

#[derive(Debug)]
pub struct Chart {
    iid: IID,
    tf: TimeFrame,
    bars: Vec<Bar>,
    now: Option<Bar>,

    feat_extremum: bool,
    t1: Vec<Extremum>,
    t2: Vec<Extremum>,
    t3: Vec<Extremum>,
    t4: Vec<Extremum>,
    t5: Vec<Extremum>,
    t1_now: Option<Extremum>,
    t2_now: Option<Extremum>,
    t3_now: Option<Extremum>,
    t4_now: Option<Extremum>,
    t5_now: Option<Extremum>,
}
impl Chart {
    pub fn new(iid: &IID, tf: &TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            iid: iid.clone(),
            tf: tf.clone(),
            bars,
            now: None,

            // XXX: extra features
            feat_extremum: false,
            t1: Vec::new(),
            t2: Vec::new(),
            t3: Vec::new(),
            t4: Vec::new(),
            t5: Vec::new(),
            t1_now: None,
            t2_now: None,
            t3_now: None,
            t4_now: None,
            t5_now: None,
        }
    }
    pub fn empty(iid: &IID, tf: &TimeFrame) -> Self {
        Self::new(iid, tf, Vec::new())
    }
    pub fn load(
        iid: &IID,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        match Manager::request(&iid, &tf.to_market_data(), begin, end) {
            Ok(df) => {
                let bars = Bar::from_df(df).unwrap();
                let chart = Self::new(iid, tf, bars);

                Ok(chart)
            }
            Err(e) => {
                log::warn!("{}, using empty chart", e);
                Ok(Self::empty(iid, tf))
            }
        }
    }

    /// Return chart instrument id
    pub fn iid(&self) -> &IID {
        &self.iid
    }
    /// Return chart timeframe
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
    }
    /// Return bars of chart
    pub fn bars(&self) -> &Vec<Bar> {
        &self.bars
    }
    /// Return fist historical bar of chart
    pub fn first(&self) -> Option<&Bar> {
        self.bars.first()
    }
    /// Return last historical bar of chart
    pub fn last(&self) -> Option<&Bar> {
        self.bars.last()
    }
    /// Return real-time bar of chart
    pub fn now(&self) -> Option<&Bar> {
        self.now.as_ref()
    }
    /// Return real-time bar close
    pub fn last_price(&self) -> Option<f64> {
        match &self.now {
            Some(bar) => Some(bar.c),
            None => None,
        }
    }
    /// Select bars in closed range [from, till]
    pub fn select(&self, from: i64, till: i64) -> &[Bar] {
        assert!(from <= till);

        let f = Chart::bisect_left(&self.bars, &from);
        let t = Chart::bisect_left(&self.bars, &till);

        &self.bars[f..=t]
    }

    /// Swallow new bar
    /// Depending on datetime of 'new_bar' this function do:
    ///  - only update real-time bar
    ///  - add new historical bar and update real-time
    pub fn swallow_bar(&mut self, new_bar: Bar) {
        match self.now.take() {
            None => {
                // receive first real time bar
                self.now = Some(new_bar);
            }
            Some(old_bar) => {
                // only update now bar
                if old_bar.ts_nanos == new_bar.ts_nanos {
                    self.now = Some(new_bar);
                // new historical bar and update now bar
                } else if old_bar.ts_nanos < new_bar.ts_nanos {
                    self.bars.push(old_bar.clone());
                    self.now = Some(new_bar);

                    // XXX: extra features
                    if self.feat_extremum {
                        self.upd_extr(&old_bar);
                    }
                }
                // old_bar.ts_nanos > new_bar.ts_nanos
                // Тинькофф бывает прокидывает в дата стриме
                // исторические бары законченные уже после новых
                // реал-тайм баров. По факту же - последний
                // реал-тайм бар который был в потоке как незаконченный
                // он равен этому законченному историческому бару
                // так что в моем алгоритме приема баров он не нужен, игнор.
                else {
                    return;
                }
            }
        }
    }

    // private
    fn bisect_left(bars: &Vec<Bar>, ts: &i64) -> usize {
        let mut left = 0;
        let mut right = bars.len();
        let mut mid;

        // начальные проверки на пустой вектор и значение за пределами
        if right == 0 {
            return 0;
        } else if ts < &bars.first().unwrap().ts_nanos {
            return 0; // искомый меньше всех в векторе
        } else if ts > &bars.last().unwrap().ts_nanos {
            return right; // искомый больше всех в векторе
        }

        while left < right {
            mid = left + (right - left) / 2;
            let current = &bars[mid].ts_nanos;

            if current < ts {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}
impl AsRef<Chart> for Chart {
    fn as_ref(&self) -> &Chart {
        &self
    }
}

// XXX: extra features
use ExtremumKind::Max;
use ExtremumKind::Min;
use Term::T1;
use Term::T2;
use Term::T3;
use Term::T4;
use Term::T5;
pub enum Features {
    Extremum,
}
impl Chart {
    pub fn features(&mut self, feat: Features, enable: bool) {
        match feat {
            Features::Extremum => {
                self.feat_extremum = enable;
                if self.feat_extremum {
                    self.calc_extr();
                }
            }
        };
    }
    pub fn t1(&self) -> &Vec<Extremum> {
        &self.t1
    }
    pub fn t2(&self) -> &Vec<Extremum> {
        &self.t2
    }
    pub fn t3(&self) -> &Vec<Extremum> {
        &self.t3
    }
    pub fn t4(&self) -> &Vec<Extremum> {
        &self.t4
    }
    pub fn t5(&self) -> &Vec<Extremum> {
        &self.t5
    }
    pub fn t1_now(&self) -> Option<&Extremum> {
        self.t1_now.as_ref()
    }
    pub fn t2_now(&self) -> Option<&Extremum> {
        self.t2_now.as_ref()
    }
    pub fn t3_now(&self) -> Option<&Extremum> {
        self.t3_now.as_ref()
    }
    pub fn t4_now(&self) -> Option<&Extremum> {
        self.t4_now.as_ref()
    }
    pub fn t5_now(&self) -> Option<&Extremum> {
        self.t5_now.as_ref()
    }

    pub fn extr(&self, term: &Term, n: usize) -> Option<&Extremum> {
        if n == 0 {
            match term {
                T1 => return self.t1_now.as_ref(),
                T2 => return self.t2_now.as_ref(),
                T3 => return self.t3_now.as_ref(),
                T4 => return self.t4_now.as_ref(),
                T5 => return self.t5_now.as_ref(),
            };
        };

        let extremums = match term {
            T1 => &self.t1,
            T2 => &self.t2,
            T3 => &self.t3,
            T4 => &self.t4,
            T5 => &self.t5,
        };

        if n > extremums.len() {
            return None;
        }

        let n = extremums.len() - n;
        extremums.get(n)
    }
    pub fn trend(&self, term: &Term, n: usize) -> Option<Trend> {
        let e1 = self.extr(term, n + 1);
        let e2 = self.extr(term, n);

        if e1.is_some() && e2.is_some() {
            let e1 = e1.unwrap();
            let e2 = e2.unwrap();
            let bars = self.select(e1.ts_nanos, e2.ts_nanos);
            let t = Trend::new(e1, e2, bars);
            return Some(t);
        }

        None
    }
    pub fn all_trends(&self, term: &Term) -> Vec<Trend> {
        let mut all_trends = Vec::new();
        let mut n = 1;
        let mut trend = self.trend(term, n);

        while trend.is_some() {
            all_trends.push(trend.unwrap());
            n += 1;
            trend = self.trend(term, n);
        }

        all_trends
    }

    fn calc_extr(&mut self) {
        let (t1, t1_now) = self.calc_extr_t1();
        let (t2, t2_now) = self.calc_extr_next(&t1, T2);
        let (t3, t3_now) = self.calc_extr_next(&t2, T3);
        let (t4, t4_now) = self.calc_extr_next(&t3, T4);
        let (t5, t5_now) = self.calc_extr_next(&t4, T5);

        self.t1 = t1;
        self.t2 = t2;
        self.t3 = t3;
        self.t4 = t4;
        self.t5 = t5;

        self.t1_now = t1_now;
        self.t2_now = t2_now;
        self.t3_now = t3_now;
        self.t4_now = t4_now;
        self.t5_now = t5_now;
    }
    fn calc_extr_t1(&mut self) -> (Vec<Extremum>, Option<Extremum>) {
        // if chart is empty
        let mut out_extr = Vec::new();
        let mut out_now;
        if self.bars.is_empty() {
            return (out_extr, None);
        }

        // start extremum kind (Max | Min) depends on first bar (bull | bear)
        let mut prev = &self.bars[0];
        let bars = &self.bars[1..];
        if prev.is_bull() {
            out_now = Extremum::new(prev.ts_nanos, T1, Max, prev.h);
        } else {
            out_now = Extremum::new(prev.ts_nanos, T1, Min, prev.l);
        }

        // cacl extremums Term::T1
        for cur in bars.iter() {
            if out_now.is_max() {
                if cur.h > prev.h {
                    out_now = Extremum::new(cur.ts_nanos, T1, Max, cur.h);
                } else {
                    out_extr.push(out_now);
                    out_now = Extremum::new(cur.ts_nanos, T1, Min, cur.l);
                }
            } else {
                if cur.l < prev.l {
                    out_now = Extremum::new(cur.ts_nanos, T1, Min, cur.l);
                } else {
                    out_extr.push(out_now);
                    out_now = Extremum::new(cur.ts_nanos, T1, Max, cur.h);
                }
            }
            prev = cur;
        }

        (out_extr, Some(out_now))
    }
    fn calc_extr_next(
        &mut self,
        in_extr: &Vec<Extremum>,
        out_term: Term,
    ) -> (Vec<Extremum>, Option<Extremum>) {
        // if input extremum list is empty
        let mut out_extr = Vec::new();
        if in_extr.is_empty() {
            return (out_extr, None);
        }

        let mut in_prev = &in_extr[0];
        let mut out_now = &in_extr[0];
        let in_extr = &in_extr[1..];

        // cacl extremums high term
        for in_cur in in_extr.iter() {
            if in_cur.kind != out_now.kind {
                in_prev = in_cur;
                continue;
            }

            if out_now.is_max() {
                if in_cur.price > out_now.price {
                    out_now = in_cur;
                } else {
                    out_extr.push(out_now.clone());
                    out_now = in_prev;
                    in_prev = in_cur;
                }
            }

            if out_now.is_min() {
                if in_cur.price < out_now.price {
                    out_now = in_cur;
                } else {
                    out_extr.push(out_now.clone());
                    out_now = in_prev;
                    in_prev = in_cur;
                }
            }
        }

        // replace Term
        for i in out_extr.iter_mut() {
            i.term = out_term.clone();
        }
        let mut out_now = out_now.clone();
        out_now.term = out_term.clone();

        (out_extr, Some(out_now))
    }
    fn upd_extr(&mut self, bar: &Bar) {
        let mut has_new = self.upd_extr_t1(bar);
        if !has_new {
            return;
        }

        has_new = self.upd_extr_next(T2);
        if !has_new {
            return;
        }

        has_new = self.upd_extr_next(T3);
        if !has_new {
            return;
        }

        has_new = self.upd_extr_next(T4);
        if !has_new {
            return;
        }

        self.upd_extr_next(T5);
    }
    fn upd_extr_t1(&mut self, bar: &Bar) -> bool {
        let mut now_extr = self.t1_now.take().unwrap();
        let mut has_new_extr = false;

        // if now extremum is max
        if now_extr.is_max() {
            if bar.h > now_extr.price {
                now_extr = Extremum::new(bar.ts_nanos, T1, Max, bar.h);
                self.t1_now = Some(now_extr);
            } else {
                has_new_extr = true;
                self.t1.push(now_extr);
                now_extr = Extremum::new(bar.ts_nanos, T1, Min, bar.l);
                self.t1_now = Some(now_extr);
            }
        }
        // if now extremum is min
        else {
            if bar.l < now_extr.price {
                now_extr = Extremum::new(bar.ts_nanos, T1, Min, bar.l);
                self.t1_now = Some(now_extr);
            } else {
                has_new_extr = true;
                self.t1.push(now_extr);
                now_extr = Extremum::new(bar.ts_nanos, T1, Max, bar.h);
                self.t1_now = Some(now_extr);
            }
        };

        has_new_extr
    }
    fn upd_extr_next(&mut self, out_term: Term) -> bool {
        let in_last;
        let in_prev;
        let out_extr;
        let mut out_now;
        let mut has_new_extr = false;
        match out_term {
            T1 => panic!(),
            T2 => {
                in_last = self.t1[self.t1.len() - 1].clone();
                in_prev = self.t1[self.t1.len() - 2].clone();
                out_now = self.t2_now.clone().take().unwrap();
                out_extr = &mut self.t2;
            }
            T3 => {
                in_last = self.t2[self.t2.len() - 1].clone();
                in_prev = self.t2[self.t2.len() - 2].clone();
                out_now = self.t3_now.clone().take().unwrap();
                out_extr = &mut self.t3;
            }
            T4 => {
                in_last = self.t3[self.t3.len() - 1].clone();
                in_prev = self.t3[self.t3.len() - 2].clone();
                out_now = self.t4_now.clone().take().unwrap();
                out_extr = &mut self.t4;
            }
            T5 => {
                in_last = self.t4[self.t4.len() - 1].clone();
                in_prev = self.t4[self.t4.len() - 2].clone();
                out_now = self.t5_now.clone().take().unwrap();
                out_extr = &mut self.t5;
            }
        }

        // если текущий младший тип != текущий старший тип -> делать ничего
        if in_last.kind != out_now.kind {
            return has_new_extr;
        }

        // if now extremum is max
        if out_now.is_max() {
            if in_last.price > out_now.price {
                out_now = in_last;
            } else {
                has_new_extr = true;
                out_extr.push(out_now);
                out_now = in_prev;
            }
        }
        // if now extremum is min
        else {
            if in_last.price < out_now.price {
                out_now = in_last;
            } else {
                has_new_extr = true;
                out_extr.push(out_now);
                out_now = in_prev;
            }
        }

        // replace Term
        out_now.term = out_term;

        // wrap & put back now extremum
        match out_now.term {
            T1 => panic!(),
            T2 => self.t2_now = Some(out_now),
            T3 => self.t3_now = Some(out_now),
            T4 => self.t4_now = Some(out_now),
            T5 => self.t5_now = Some(out_now),
        };

        has_new_extr
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::{TimeZone, Utc};

    use crate::*;

    #[test]
    fn new() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "SHARE".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);

        let tf = TimeFrame::new("D");
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let df = Manager::request(&iid, &tf.to_market_data(), &begin, &end)
            .unwrap();
        let bars = Bar::from_df(df).unwrap();

        let chart = Chart::new(&iid, &tf, bars);
        assert_eq!(chart.iid, iid);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 256);
        assert!(chart.now.is_none());
    }
    #[test]
    fn empty() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);
        let tf = TimeFrame::new("D");

        let chart = Chart::empty(&iid, &tf);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
        assert!(chart.now.is_none());
    }
    #[test]
    fn load() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "SHARE".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2023-08-01");
        let end = Usr::date("2023-09-01");

        let chart = Chart::load(&iid, &tf, &begin, &end).unwrap();
        assert_eq!(chart.tf(), &tf);
        assert_eq!(chart.bars().len(), 23);
        assert!(chart.now().is_none());

        assert_eq!(chart.first().unwrap().dt(), begin);
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap(),
        )
    }
    #[test]
    fn bisect_left() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2024-12-20");
        let end = Usr::date("2025-01-01");
        let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

        let v = chart.bars();
        // for i in v.iter() {
        //     println!("{:?}", i);
        // }
        // Bar { ts_nanos: 1734642000000000000, ...
        // Bar { ts_nanos: 1734901200000000000, ...  // from
        // Bar { ts_nanos: 1734987600000000000, ...
        // Bar { ts_nanos: 1735074000000000000, ...  // till
        // Bar { ts_nanos: 1735160400000000000, ...
        // Bar { ts_nanos: 1735246800000000000, ...
        // Bar { ts_nanos: 1735333200000000000, ...
        // Bar { ts_nanos: 1735506000000000000, ...

        let from = Chart::bisect_left(v, &1734901200000000000);
        let till = Chart::bisect_left(v, &1735074000000000000);
        let s = &v[from..=till];
        assert_eq!(s.len(), 3);

        // test other values
        assert_eq!(Chart::bisect_left(v, &1734642000000000000), 0); // x == 0
        assert_eq!(Chart::bisect_left(v, &1734901200000000000), 1); // x == 1
        assert_eq!(Chart::bisect_left(v, &1734987600000000000), 2); // x == 2
        assert_eq!(Chart::bisect_left(v, &1735074000000000000), 3); // x == 3
        assert_eq!(Chart::bisect_left(v, &1735160400000000000), 4); // x == 4
        assert_eq!(Chart::bisect_left(v, &1735246800000000000), 5); // x == 5
        assert_eq!(Chart::bisect_left(v, &1735333200000000000), 6); // x == 6
        assert_eq!(Chart::bisect_left(v, &1735506000000000000), 7); // x == 7

        // test out of vector values
        assert_eq!(Chart::bisect_left(v, &1000000000000000000), 0); // x < 0
        assert_eq!(Chart::bisect_left(v, &1734642000000000001), 1); // 0<x<1
        assert_eq!(Chart::bisect_left(v, &1999999999999999999), 8); // 7 < x
    }
    #[test]
    fn select() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2024-12-20");
        let end = Usr::date("2025-01-01");
        let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

        let from = Usr::date("2024-12-23").timestamp_nanos_opt().unwrap();
        let till = Usr::date("2024-12-25").timestamp_nanos_opt().unwrap();

        let selected = chart.select(from, till);
        assert_eq!(selected.len(), 3);
    }

    #[test]
    fn extremum_t1() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2024-12-20");
        let end = Usr::date("2025-01-01");
        share.load_chart_period(&tf, &begin, &end).unwrap();

        let chart = share.mut_chart(&tf).unwrap();
        chart.features(Features::Extremum, true);

        // one real-time extremum
        let extr = chart.extr(&T1, 0).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 279.49);

        // 4 historical extremum
        assert_eq!(chart.t1().len(), 4);
        let extr = chart.extr(&T1, 1).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Min);
        assert_eq!(extr.price, 268.57);
        let extr = chart.extr(&T1, 2).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 274.25);
        let extr = chart.extr(&T1, 3).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Min);
        assert_eq!(extr.price, 260.31);
        let extr = chart.extr(&T1, 4).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 270.0);
    }
    #[test]
    fn trend_t1() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2024-12-20");
        let end = Usr::date("2025-01-01");
        share.load_chart_period(&tf, &begin, &end).unwrap();

        let chart = share.mut_chart(&tf).unwrap();
        chart.features(Features::Extremum, true);

        // last 3 extremums
        let e2 = chart.extr(&T1, 2).unwrap();
        let e1 = chart.extr(&T1, 1).unwrap();
        let e0 = chart.extr(&T1, 0).unwrap();

        // trend 0 = real-time trend
        let trend = chart.trend(&T1, 0).unwrap();
        assert_eq!(trend.period(), 3);
        assert_eq!(trend.begin(), e1);
        assert_eq!(trend.end(), e0);

        // trend 1 = last historical trend
        let trend = chart.trend(&T1, 1).unwrap();
        assert_eq!(trend.period(), 2);
        assert_eq!(trend.begin(), e2);
        assert_eq!(trend.end(), e1);

        // trend 2
        let trend = chart.trend(&T1, 2).unwrap();
        assert_eq!(trend.period(), 2);
        assert_eq!(trend.end(), e2);

        // trend 3
        let trend = chart.trend(&T1, 3).unwrap();
        assert_eq!(trend.period(), 3);
    }
}
