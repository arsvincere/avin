/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::VecDeque;

use chrono::{DateTime, Utc};

use avin_core::{
    Bar, BarEvent, Event, Iid, Manager, MarketData, Source, TimeFrame,
};

pub struct DataStream {
    pub iid: Iid,
    bars_1m: VecDeque<Bar>,
}

impl DataStream {
    pub fn new(iid: &Iid, begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            iid: iid.clone(),
            bars_1m: load_bars(iid, begin, end),
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        // достать 1М бар
        if let Some(bar) = self.bars_1m.pop_front() {
            // собрать и вернуть эвент
            let figi = self.iid.figi().clone();
            let tf = TimeFrame::M1;

            return Some(Event::Bar(BarEvent::new(figi, tf, bar)));
        }

        // если бары кончились
        None
    }

    // private
}

fn load_bars(iid: &Iid, b: DateTime<Utc>, e: DateTime<Utc>) -> VecDeque<Bar> {
    let source = Source::MOEXALGO;
    let df = Manager::load(iid, source, MarketData::BAR_1M, b, e).unwrap();

    let ts = df
        .column("ts_nanos")
        .unwrap()
        .i64()
        .unwrap()
        .into_no_null_iter();
    let mut o = df
        .column("open")
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter();
    let mut h = df
        .column("high")
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter();
    let mut l = df.column("low").unwrap().f64().unwrap().into_no_null_iter();
    let mut c = df
        .column("close")
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter();
    let mut v = df
        .column("volume")
        .unwrap()
        .u64()
        .unwrap()
        .into_no_null_iter();

    let mut bars_1m = VecDeque::with_capacity(df.height());
    for t in ts {
        let bar = Bar::new(
            t,
            o.next().unwrap(),
            h.next().unwrap(),
            l.next().unwrap(),
            c.next().unwrap(),
            v.next().unwrap(),
        );
        bars_1m.push_back(bar);
    }

    if bars_1m.is_empty() {
        log::warn!("No data for {iid}");
    }

    bars_1m
}

#[cfg(test)]
mod tests {
    use super::*;
    use avin_core::Share;
    use chrono::{TimeZone, Utc};

    #[test]
    fn stream() {
        let share = Share::new("moex_share_sber").unwrap();
        let iid = share.iid();
        let begin = Utc.with_ymd_and_hms(2023, 8, 1, 10, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2023, 8, 1, 10, 10, 0).unwrap();
        let mut ds = DataStream::new(iid, begin, end);

        let mut bars_1m_count = 0;
        let mut bars_10m_count = 0;
        while let Some(e) = ds.next_event() {
            match e {
                Event::Bar(e) => {
                    if e.tf == TimeFrame::M1 {
                        bars_1m_count += 1;
                    } else if e.tf == TimeFrame::M10 {
                        bars_10m_count += 1;
                    }
                }
                _ => todo!(),
            }
        }
        assert_eq!(bars_1m_count, 10);

        // после изменения логики работы графиков - теперь они принимают
        // только 1М бары и дальше с них клеят свой таймфрейм, дата стрим
        // теперь тоже выдает только 1М бары
        assert_eq!(bars_10m_count, 0);
    }
}
