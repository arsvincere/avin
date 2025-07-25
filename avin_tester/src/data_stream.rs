/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::VecDeque;

use chrono::{DateTime, Utc};

use avin_core::{Bar, BarEvent, Event, Iid, Manager, MarketData, TimeFrame};

pub struct DataStream {
    pub iid: Iid,
    bars_1m: VecDeque<Bar>,
    queue: VecDeque<Event>,
    // bar_5m: Option<Bar>,
    bar_10m: Option<Bar>,
    bar_1h: Option<Bar>,
    bar_d: Option<Bar>,
}

impl DataStream {
    pub fn new(
        iid: &Iid,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Self {
        let bars_1m = DataStream::load_bars(iid, begin, end);

        Self {
            iid: iid.clone(),
            bars_1m,
            queue: VecDeque::new(),
            // bar_5m: None,
            bar_10m: None,
            bar_1h: None,
            bar_d: None,
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        // если в очереди есть event - выдать его
        let e = self.queue.pop_front();
        if e.is_some() {
            return e;
        }

        // Иначе: достать 1М бар
        if let Some(bar) = self.bars_1m.pop_front() {
            self.create_event_1m(bar);
            self.create_event_10m(bar);
            self.create_event_1h(bar);
            self.create_event_d(bar);

            // достать из очереди первый эвент и выдать его
            return self.queue.pop_front();
        }

        None
    }

    // private
    fn load_bars(
        iid: &Iid,
        b: &DateTime<Utc>,
        e: &DateTime<Utc>,
    ) -> VecDeque<Bar> {
        let df = Manager::load(iid, &MarketData::BAR_1M, b, e).unwrap();

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
        let mut l =
            df.column("low").unwrap().f64().unwrap().into_no_null_iter();
        let mut c = df
            .column("close")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut v = df
            .column("volume")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut val = df
            .column("value")
            .unwrap()
            .f64()
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
                v.next().unwrap() as u64,
                Some(val.next().unwrap()),
            );
            bars_1m.push_back(bar);
        }

        if bars_1m.is_empty() {
            log::warn!("No data for {iid}");
        }

        bars_1m
    }
    fn create_event_1m(&mut self, bar_1m: Bar) {
        let figi = self.iid.figi().clone();
        let tf = TimeFrame::M1;
        let event = BarEvent::new(figi, tf, bar_1m);

        self.queue.push_back(Event::Bar(event));
    }
    fn create_event_10m(&mut self, bar_1m: Bar) {
        // first bar
        if self.bar_10m.is_none() {
            self.bar_10m = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::M10;
            let event = BarEvent::new(figi, tf, self.bar_10m.unwrap());
            self.queue.push_back(Event::Bar(event));
            return;
        }

        // else
        let bar_10m = self.bar_10m.take().unwrap();
        let next_ts = TimeFrame::M10.next_ts(bar_10m.ts_nanos);

        // only update
        if bar_1m.ts_nanos < next_ts {
            self.bar_10m = Some(bar_10m.join(bar_1m));

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::M10;
            let event = BarEvent::new(figi, tf, self.bar_10m.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
        // create new
        else {
            self.bar_10m = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::M10;
            let event = BarEvent::new(figi, tf, self.bar_10m.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
    }
    fn create_event_1h(&mut self, bar_1m: Bar) {
        // first bar
        if self.bar_1h.is_none() {
            self.bar_1h = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::H1;
            let event = BarEvent::new(figi, tf, self.bar_1h.unwrap());
            self.queue.push_back(Event::Bar(event));
            return;
        }

        // else
        let bar_1h = self.bar_1h.take().unwrap();
        let next_ts = TimeFrame::H1.next_ts(bar_1h.ts_nanos);

        // only update
        if bar_1m.ts_nanos < next_ts {
            self.bar_1h = Some(bar_1h.join(bar_1m));

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::H1;
            let event = BarEvent::new(figi, tf, self.bar_1h.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
        // create new
        else {
            self.bar_1h = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::H1;
            let event = BarEvent::new(figi, tf, self.bar_1h.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
    }
    fn create_event_d(&mut self, bar_1m: Bar) {
        // first bar
        if self.bar_d.is_none() {
            self.bar_d = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::Day;
            let event = BarEvent::new(figi, tf, self.bar_d.unwrap());
            self.queue.push_back(Event::Bar(event));
            return;
        }

        // else
        let bar_d = self.bar_d.take().unwrap();
        let next_ts = TimeFrame::Day.next_ts(bar_d.ts_nanos);

        // only update
        if bar_1m.ts_nanos < next_ts {
            self.bar_d = Some(bar_d.join(bar_1m));

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::Day;
            let event = BarEvent::new(figi, tf, self.bar_d.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
        // create new
        else {
            self.bar_d = Some(bar_1m);

            let figi = self.iid.figi().clone();
            let tf = TimeFrame::Day;
            let event = BarEvent::new(figi, tf, self.bar_d.unwrap());
            self.queue.push_back(Event::Bar(event));
        }
    }
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
        let mut ds = DataStream::new(iid, &begin, &end);

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
        assert_eq!(bars_10m_count, 10);
    }
}
