/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use strum::EnumIter;

use crate::{Bar, Chart, Kind};
use ExtremumKind::Max;
use ExtremumKind::Min;
use Kind::{Bear, Bull};
use Term::T1;
use Term::T2;
use Term::T3;
use Term::T4;
use Term::T5;
use avin_utils::{self as utils, bisect_left, bisect_right};

use super::Indicator;

// random UUID, used as key in HashMap with indicators in struct Chart
const ID: &str = "9479c78b-d54e-4042-8893-19f7a2a9ed53";

// may be used later in gui for display human readable indicator name
const NAME: &str = "Extremum";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, EnumIter)]
pub enum Term {
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
}
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::T1 => write!(f, "T1"),
            Self::T2 => write!(f, "T2"),
            Self::T3 => write!(f, "T3"),
            Self::T4 => write!(f, "T4"),
            Self::T5 => write!(f, "T5"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtremumKind {
    Max,
    Min,
}
impl ExtremumKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Max => "",
            Self::Min => "",
        }
    }
}
impl std::fmt::Display for ExtremumKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Max => write!(f, "Max"),
            Self::Min => write!(f, "Min"),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Trend {
    e1: Extremum,
    e2: Extremum,
    len: usize,
    vol: u64,
    kind: Kind,
}
impl Trend {
    pub fn new(e1: &Extremum, e2: &Extremum, bars: &[Bar]) -> Trend {
        assert!(e1.ts_nanos < e2.ts_nanos);

        let mut vol = 0;
        for bar in bars.iter() {
            vol += bar.v;
        }

        Trend {
            e1: e1.clone(),
            e2: e2.clone(),
            len: bars.len(),
            vol,
            kind: if e1.price < e2.price { Bull } else { Bear },
        }
    }
    pub fn begin(&self) -> &Extremum {
        &self.e1
    }
    pub fn end(&self) -> &Extremum {
        &self.e2
    }
    pub fn term(&self) -> Term {
        utils::min(self.e1.term, self.e2.term)
    }
    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn is_bear(&self) -> bool {
        self.kind == Bear
    }
    pub fn is_bull(&self) -> bool {
        self.kind == Bull
    }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.len as u32
    }
    pub fn vol(&self) -> u64 {
        self.vol
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
}
impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sign = if self.is_bull() { "+" } else { "-" };
        write!(
            f,
            "Trend: {}{} / {} = {} [vol: {}  {} -> {}]",
            sign,
            self.abs_p(),
            self.len,
            self.speed_p(),
            self.vol,
            self.e1.dt_local(),
            self.e2.dt_local(),
        )
    }
}

// public interface for Chart
pub trait ExtremumIndicator {
    fn init(&mut self);
    fn extr(&self, term: Term, n: usize) -> Option<&Extremum>;
    fn trend(&self, term: Term, n: usize) -> Option<&Trend>;
    fn all_extr(&self, term: Term) -> &Vec<Extremum>;
    fn all_trend(&self, term: Term) -> &Vec<Trend>;
}
impl ExtremumIndicator for Chart {
    fn init(&mut self) {
        // todo!();
        // XXX: выпилен now бар из графика, и теперь текущий бар
        // идет в общем векторе bars, проверить не повлияет ли это
        // на алгоритмы тут
        self.add_ind(Indicator::Extremum(ExtremumData::default()));
    }
    fn extr(&self, term: Term, n: usize) -> Option<&Extremum> {
        // get indicator data
        let extr_data = match self.get_ind(ID) {
            Some(Indicator::Extremum(data)) => data,
            None => panic!("Chart don't have indicator {NAME}"),
        };

        extr_data.extr(term, n)
    }
    fn trend(&self, term: Term, n: usize) -> Option<&Trend> {
        // get indicator data
        let extr_data = match self.get_ind(ID) {
            Some(Indicator::Extremum(data)) => data,
            None => panic!("Chart don't have indicator {NAME}"),
        };

        extr_data.trend(term, n)
    }
    fn all_extr(&self, term: Term) -> &Vec<Extremum> {
        // get indicator data
        let extr_data = match self.get_ind(ID) {
            Some(Indicator::Extremum(data)) => data,
            None => panic!("Chart don't have indicator {NAME}"),
        };

        extr_data.all_extr(term)
    }
    fn all_trend(&self, term: Term) -> &Vec<Trend> {
        // get indicator data
        let extr_data = match self.get_ind(ID) {
            Some(Indicator::Extremum(data)) => data,
            None => panic!("Chart don't have indicator {NAME}"),
        };

        extr_data.all_trend(term)
    }
}

// private realization, but struct need to be pub
// for wrap in enum 'Indicator' in module '_indicator.rs'
#[derive(Debug, Default)]
pub struct ExtremumData {
    e_t1: Vec<Extremum>,
    e_t2: Vec<Extremum>,
    e_t3: Vec<Extremum>,
    e_t4: Vec<Extremum>,
    e_t5: Vec<Extremum>,
    e_t1_now: Option<Extremum>,
    e_t2_now: Option<Extremum>,
    e_t3_now: Option<Extremum>,
    e_t4_now: Option<Extremum>,
    e_t5_now: Option<Extremum>,

    t_t1: Vec<Trend>,
    t_t2: Vec<Trend>,
    t_t3: Vec<Trend>,
    t_t4: Vec<Trend>,
    t_t5: Vec<Trend>,
    t_t1_now: Option<Trend>,
    t_t2_now: Option<Trend>,
    t_t3_now: Option<Trend>,
    t_t4_now: Option<Trend>,
    t_t5_now: Option<Trend>,

    last_ts: i64,
}
impl ExtremumData {
    // indicator interface
    pub fn id(&self) -> &'static str {
        ID
    }
    pub fn name(&self) -> &'static str {
        NAME
    }
    pub fn init(&mut self, bars: &[Bar]) {
        self.calc_e1(bars);
        self.calc_en(T2);
        self.calc_en(T3);
        self.calc_en(T4);
        self.calc_en(T5);

        self.calc_trends(T1, bars);
        self.calc_trends(T2, bars);
        self.calc_trends(T3, bars);
        self.calc_trends(T4, bars);
        self.calc_trends(T5, bars);
    }
    pub fn update(&mut self, bars: &[Bar]) {
        // В тестере/сканере, после init на пустом графике нет ни одного
        // экстремума и нет исторических баров. Поэтому, первое смотрим на
        // наличие исторических баров в принципе.
        if bars.is_empty() {
            return;
        }

        // когда пришел первый бар надо инициализировать
        if bars.len() == 1 {
            self.init(bars);
        }

        // update вызывается на каждом тике,
        // а меня интересуют только исторические бары, поэтому чекаем ts
        // последнего исторического бара в графике
        let current = bars.last().unwrap();
        if self.last_ts == current.ts_nanos {
            return;
        }

        // вот теперь есть что обновлять
        self.upd_extr(current);
        self.upd_trends(bars);

        // сохраняем время последнего обработанного бара
        self.last_ts = current.ts_nanos;
    }

    // private
    fn extr(&self, term: Term, n: usize) -> Option<&Extremum> {
        if n == 0 {
            // real-time extremum, n == 0
            match term {
                T1 => self.e_t1_now.as_ref(),
                T2 => self.e_t2_now.as_ref(),
                T3 => self.e_t3_now.as_ref(),
                T4 => self.e_t4_now.as_ref(),
                T5 => self.e_t5_now.as_ref(),
            }
        } else {
            // historical extremum, n > 0
            let extremums = match term {
                T1 => &self.e_t1,
                T2 => &self.e_t2,
                T3 => &self.e_t3,
                T4 => &self.e_t4,
                T5 => &self.e_t5,
            };

            let index = extremums.len() - n;
            extremums.get(index)
        }
    }
    fn trend(&self, term: Term, n: usize) -> Option<&Trend> {
        if n == 0 {
            match term {
                T1 => self.t_t1_now.as_ref(),
                T2 => self.t_t2_now.as_ref(),
                T3 => self.t_t3_now.as_ref(),
                T4 => self.t_t4_now.as_ref(),
                T5 => self.t_t5_now.as_ref(),
            }
        } else {
            // n > 0
            let all_trends = match term {
                T1 => &self.t_t1,
                T2 => &self.t_t2,
                T3 => &self.t_t3,
                T4 => &self.t_t4,
                T5 => &self.t_t5,
            };

            let index = all_trends.len() - n;
            all_trends.get(index)
        }
    }
    fn all_extr(&self, term: Term) -> &Vec<Extremum> {
        match term {
            T1 => &self.e_t1,
            T2 => &self.e_t2,
            T3 => &self.e_t3,
            T4 => &self.e_t4,
            T5 => &self.e_t5,
        }
    }
    fn all_trend(&self, term: Term) -> &Vec<Trend> {
        match term {
            T1 => &self.t_t1,
            T2 => &self.t_t2,
            T3 => &self.t_t3,
            T4 => &self.t_t4,
            T5 => &self.t_t5,
        }
    }

    fn calc_e1(&mut self, bars: &[Bar]) {
        // if chart is empty
        if bars.is_empty() {
            self.e_t1 = Vec::new();
            self.e_t1_now = None;
            return;
        }

        // tmp variables
        let mut t1 = Vec::new();
        let mut t1_now;

        // start extremum kind (Max | Min) depends on first bar (bull | bear)
        let mut prev = &bars[0];
        let bars = &bars[1..];
        if prev.is_bull() {
            t1_now = Extremum::new(prev.ts_nanos, T1, Max, prev.h);
        } else {
            t1_now = Extremum::new(prev.ts_nanos, T1, Min, prev.l);
        }

        // cacl extremums Term::T1
        for cur in bars.iter() {
            if t1_now.is_max() {
                if cur.h > prev.h {
                    t1_now = Extremum::new(cur.ts_nanos, T1, Max, cur.h);
                } else {
                    t1.push(t1_now);
                    t1_now = Extremum::new(cur.ts_nanos, T1, Min, cur.l);
                }
            } else if t1_now.is_min() {
                if cur.l < prev.l {
                    t1_now = Extremum::new(cur.ts_nanos, T1, Min, cur.l);
                } else {
                    t1.push(t1_now);
                    t1_now = Extremum::new(cur.ts_nanos, T1, Max, cur.h);
                }
            }
            prev = cur;
        }

        self.e_t1 = t1;
        self.e_t1_now = Some(t1_now);
        self.last_ts = bars.last().unwrap().ts_nanos;
    }
    fn calc_en(&mut self, out_term: Term) {
        let in_extr = match out_term {
            T1 => panic!(),
            T2 => &self.e_t1,
            T3 => &self.e_t2,
            T4 => &self.e_t3,
            T5 => &self.e_t4,
        };

        // if input extremum list is empty -> return
        if in_extr.is_empty() {
            return;
        }

        let mut out_extr = Vec::new();
        let mut out_now = &in_extr[0];
        let mut in_prev = &in_extr[0];
        let in_extr = &in_extr[1..];

        // cacl extremums high term
        for in_cur in in_extr.iter() {
            // skip not equal kind
            if in_cur.kind != out_now.kind {
                in_prev = in_cur;
                continue;
            }

            // now bull trend
            if out_now.is_max() {
                if in_cur.price > out_now.price {
                    out_now = in_cur;
                } else {
                    out_extr.push(out_now.clone());
                    out_now = in_prev;
                    in_prev = in_cur;
                }
            }
            // now bear trend
            else if out_now.is_min() {
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
            i.term = out_term;
        }
        let mut out_now = out_now.clone();
        out_now.term = out_term;

        match out_term {
            T1 => panic!(),
            T2 => {
                self.e_t2 = out_extr;
                self.e_t2_now = Some(out_now);
            }
            T3 => {
                self.e_t3 = out_extr;
                self.e_t3_now = Some(out_now);
            }
            T4 => {
                self.e_t4 = out_extr;
                self.e_t4_now = Some(out_now);
            }
            T5 => {
                self.e_t5 = out_extr;
                self.e_t5_now = Some(out_now);
            }
        };
    }
    fn upd_extr(&mut self, bar: &Bar) {
        if !self.upd_extr_t1(bar) {
            return;
        }
        if !self.upd_extr_tn(T2) {
            return;
        }
        if !self.upd_extr_tn(T3) {
            return;
        }
        if !self.upd_extr_tn(T4) {
            return;
        }
        self.upd_extr_tn(T5);
    }
    fn upd_extr_t1(&mut self, bar: &Bar) -> bool {
        let mut now_extr = self.e_t1_now.take().unwrap();
        let mut updated = false;

        // if now extremum is max
        if now_extr.is_max() {
            if bar.h > now_extr.price {
                now_extr = Extremum::new(bar.ts_nanos, T1, Max, bar.h);
                self.e_t1_now = Some(now_extr);
            } else {
                updated = true;
                self.e_t1.push(now_extr);
                now_extr = Extremum::new(bar.ts_nanos, T1, Min, bar.l);
                self.e_t1_now = Some(now_extr);
            }
        } else if now_extr.is_min() {
            if bar.l < now_extr.price {
                now_extr = Extremum::new(bar.ts_nanos, T1, Min, bar.l);
                self.e_t1_now = Some(now_extr);
            } else {
                updated = true;
                self.e_t1.push(now_extr);
                now_extr = Extremum::new(bar.ts_nanos, T1, Max, bar.h);
                self.e_t1_now = Some(now_extr);
            }
        };

        updated
    }
    fn upd_extr_tn(&mut self, out_term: Term) -> bool {
        let in_last;
        let in_prev;
        let out_extr;
        let mut out_now;
        match out_term {
            T1 => panic!(),
            T2 => {
                in_last = self.e_t1[self.e_t1.len() - 1].clone();
                in_prev = self.e_t1[self.e_t1.len() - 2].clone();
                out_now = self.e_t2_now.clone().unwrap();
                out_extr = &mut self.e_t2;
            }
            T3 => {
                in_last = self.e_t2[self.e_t2.len() - 1].clone();
                in_prev = self.e_t2[self.e_t2.len() - 2].clone();
                out_now = self.e_t3_now.clone().unwrap();
                out_extr = &mut self.e_t3;
            }
            T4 => {
                in_last = self.e_t3[self.e_t3.len() - 1].clone();
                in_prev = self.e_t3[self.e_t3.len() - 2].clone();
                out_now = self.e_t4_now.clone().unwrap();
                out_extr = &mut self.e_t4;
            }
            T5 => {
                in_last = self.e_t4[self.e_t4.len() - 1].clone();
                in_prev = self.e_t4[self.e_t4.len() - 2].clone();
                out_now = self.e_t5_now.clone().unwrap();
                out_extr = &mut self.e_t5;
            }
        }

        // если текущий младший тип != текущий старший тип -> делать ничего
        let mut updated = false;
        if in_last.kind != out_now.kind {
            return updated;
        }

        // if now extremum is max
        if out_now.is_max() {
            if in_last.price > out_now.price {
                out_now = in_last;
            } else {
                updated = true;
                out_extr.push(out_now);
                out_now = in_prev;
            }
        } else if out_now.is_min() {
            if in_last.price < out_now.price {
                out_now = in_last;
            } else {
                updated = true;
                out_extr.push(out_now);
                out_now = in_prev;
            }
        }

        // replace Term
        out_now.term = out_term;

        // wrap & put back now extremum
        match out_now.term {
            T1 => panic!(),
            T2 => self.e_t2_now = Some(out_now),
            T3 => self.e_t3_now = Some(out_now),
            T4 => self.e_t4_now = Some(out_now),
            T5 => self.e_t5_now = Some(out_now),
        };

        updated
    }
    fn calc_trends(&mut self, term: Term, bars: &[Bar]) {
        let in_extr = match term {
            T1 => &self.e_t1,
            T2 => &self.e_t2,
            T3 => &self.e_t3,
            T4 => &self.e_t4,
            T5 => &self.e_t5,
        };
        let in_now = match term {
            T1 => &self.e_t1_now,
            T2 => &self.e_t2_now,
            T3 => &self.e_t3_now,
            T4 => &self.e_t4_now,
            T5 => &self.e_t5_now,
        };

        let out_trends = match term {
            T1 => &mut self.t_t1,
            T2 => &mut self.t_t2,
            T3 => &mut self.t_t3,
            T4 => &mut self.t_t4,
            T5 => &mut self.t_t5,
        };

        // calc historical trends
        let mut i = 1;
        while i < in_extr.len() {
            // get extremum begin / end
            let e1 = in_extr.get(i - 1).unwrap();
            let e2 = in_extr.get(i).unwrap();
            let trend = build_trend(e1, e2, bars);

            out_trends.push(trend);
            i += 1;
        }

        // calc real-time trend
        if in_extr.last().is_some() && in_now.is_some() {
            let e1 = in_extr.last().unwrap();
            let e2 = in_now.as_ref().unwrap();
            let trend = build_trend(e1, e2, bars);

            match term {
                T1 => self.t_t1_now = Some(trend),
                T2 => self.t_t2_now = Some(trend),
                T3 => self.t_t3_now = Some(trend),
                T4 => self.t_t4_now = Some(trend),
                T5 => self.t_t5_now = Some(trend),
            };
        }
    }
    fn upd_trends(&mut self, bars: &[Bar]) {
        self.upd_trends_tn(T1, bars);
        self.upd_trends_tn(T2, bars);
        self.upd_trends_tn(T3, bars);
        self.upd_trends_tn(T4, bars);
        self.upd_trends_tn(T5, bars);
    }
    fn upd_trends_tn(&mut self, term: Term, bars: &[Bar]) {
        let last_extr = match term {
            T1 => self.e_t1.last(),
            T2 => self.e_t2.last(),
            T3 => self.e_t3.last(),
            T4 => self.e_t4.last(),
            T5 => self.e_t5.last(),
        };
        let now_extr = match term {
            T1 => self.e_t1_now.as_ref(),
            T2 => self.e_t2_now.as_ref(),
            T3 => self.e_t3_now.as_ref(),
            T4 => self.e_t4_now.as_ref(),
            T5 => self.e_t5_now.as_ref(),
        };
        // если нет исторического экстремума, то не может быть и трендов.
        if last_extr.is_none() {
            return;
        }
        let last_extr = last_extr.unwrap();
        let now_extr = now_extr.unwrap();

        let last_trend = match term {
            T1 => self.t_t1.last(),
            T2 => self.t_t2.last(),
            T3 => self.t_t3.last(),
            T4 => self.t_t4.last(),
            T5 => self.t_t5.last(),
        };
        let now_trend = match term {
            T1 => self.t_t1_now.as_ref(),
            T2 => self.t_t2_now.as_ref(),
            T3 => self.t_t3_now.as_ref(),
            T4 => self.t_t4_now.as_ref(),
            T5 => self.t_t5_now.as_ref(),
        };
        // если нет исторического тренда, в графике еще мало баров,
        // попробовать снова посчитать тренды:
        if last_trend.is_none() {
            self.calc_trends(term, bars);
            return;
        }
        let last_trend = last_trend.unwrap();
        let now_trend = now_trend.unwrap();

        // если конец исторического тренда не равен последнему историческому
        // экстремуму то обновился исторический экстремум. Значит нужно
        // сделать новый исторический тренд и новый текущий тренд.
        if last_trend.e2 != *last_extr {
            let new_last_trend = build_trend(&last_trend.e2, last_extr, bars);
            self.t_t1.push(new_last_trend);

            let new_now_trend = build_trend(last_extr, now_extr, bars);
            self.t_t1_now = Some(new_now_trend);
            return;
        }

        // иначе конец исторического тренда равен последнему историческому
        // экстремуму, возможно обновился текущий экстремум, без формирования
        // нового исторического экстремума. Проверяем соответствие текущего
        // экстремума и обновляем текущий тренд если нужно.
        if now_trend.e2 != *now_extr {
            let new_now_trend = build_trend(last_extr, now_extr, bars);
            self.t_t1_now = Some(new_now_trend);
        }
    }
}

#[inline]
fn build_trend(e1: &Extremum, e2: &Extremum, all_bars: &[Bar]) -> Trend {
    // select bars of trend
    let f = bisect_right(all_bars, e1.ts_nanos, |b| b.ts_nanos).unwrap();
    let t = bisect_left(all_bars, e2.ts_nanos, |b| b.ts_nanos).unwrap();
    let bars_of_trend = &all_bars[f..=t];

    Trend::new(e1, e2, bars_of_trend)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use avin_utils as utils;

    #[test]
    fn extremum_t1() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2024-12-20");
        let end = utils::str_date_to_utc("2025-01-01");
        share.load_chart_period(tf, begin, end).unwrap();

        let chart = share.chart_mut(tf).unwrap();
        ExtremumIndicator::init(chart);

        // one real-time extremum
        let extr = chart.extr(T1, 0).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 279.49);

        // 4 historical extremum
        let extr = chart.extr(T1, 1).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Min);
        assert_eq!(extr.price, 268.57);
        let extr = chart.extr(T1, 2).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 274.25);
        let extr = chart.extr(T1, 3).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Min);
        assert_eq!(extr.price, 260.31);
        let extr = chart.extr(T1, 4).unwrap();
        assert_eq!(extr.kind, ExtremumKind::Max);
        assert_eq!(extr.price, 270.0);
    }
    #[test]
    fn trend_t1() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2024-12-20");
        let end = utils::str_date_to_utc("2025-01-01");
        share.load_chart_period(tf, begin, end).unwrap();

        let chart = share.chart_mut(tf).unwrap();
        ExtremumIndicator::init(chart);

        // last 3 extremums
        let e2 = chart.extr(T1, 2).unwrap();
        let e1 = chart.extr(T1, 1).unwrap();
        let e0 = chart.extr(T1, 0).unwrap();

        // trend 0 = real-time trend
        let trend = chart.trend(T1, 0).unwrap();
        assert_eq!(trend.len(), 3);
        assert_eq!(trend.begin(), e1);
        assert_eq!(trend.end(), e0);

        // trend 1 = last historical trend
        let trend = chart.trend(T1, 1).unwrap();
        assert_eq!(trend.len(), 2);
        assert_eq!(trend.begin(), e2);
        assert_eq!(trend.end(), e1);

        // trend 2
        let trend = chart.trend(T1, 2).unwrap();
        assert_eq!(trend.len(), 2);
        assert_eq!(trend.end(), e2);

        // trend 3
        let trend = chart.trend(T1, 3).unwrap();
        assert_eq!(trend.len(), 3);
    }
}
