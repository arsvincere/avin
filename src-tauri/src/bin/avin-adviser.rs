/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

use avin_adviser::*;
use avin_analyse::*;
use avin_core::*;
use avin_utils::{Notice, NoticePriority};

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let mut adviser = Adviser::new();

    // conditions
    let million = MillionTic::default();
    adviser.add_condition(million);

    let anomal_bar = AnomalBar1M::default();
    adviser.add_condition(anomal_bar);
    let anomal_bar = AnomalBar5M::default();
    adviser.add_condition(anomal_bar);
    let anomal_bar = AnomalBar15M::default();
    adviser.add_condition(anomal_bar);
    let anomal_bar = AnomalBar1H::default();
    adviser.add_condition(anomal_bar);
    let anomal_bar = AnomalBar4H::default();
    adviser.add_condition(anomal_bar);

    adviser.start().await;
}

#[derive(Default)]
struct MillionTic {
    storage_ts: HashMap<String, i64>,
}
impl Condition for MillionTic {
    fn name(&self) -> &'static str {
        "1kk tic"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let tic;
        if let Some(tics) = asset.tics() {
            tic = tics.last().unwrap();
        } else {
            return None;
        }

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };
        if tic.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), tic.ts);
        }

        if tic.value > 10_000_000.0 {
            let title = format!(
                "{} {} = {:.2}m",
                asset.ticker(),
                tic.direction,
                tic.value / 1_000_000.0
            );
            let notice =
                Notice::new(title, String::new(), NoticePriority::Critical);
            return Some(notice);
        }

        if tic.value > 5_000_000.0 {
            let title = format!(
                "{} {} = {:.2}m",
                asset.ticker(),
                tic.direction,
                tic.value / 1_000_000.0
            );
            let notice =
                Notice::new(title, String::new(), NoticePriority::Normal);
            return Some(notice);
        }

        if tic.value > 1_000_000.0 {
            let title = format!(
                "{} {} = {:.2}m",
                asset.ticker(),
                tic.direction,
                tic.value / 1_000_000.0
            );
            let notice = Notice::new(title, String::new(), NoticePriority::Low);
            return Some(notice);
        }

        None
    }
}

#[derive(Default)]
struct AnomalBar1M {
    storage_ts: HashMap<String, i64>,
}
impl Condition for AnomalBar1M {
    fn name(&self) -> &'static str {
        "anomal bar 1M"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let chart = asset.chart(TimeFrame::M1)?;
        let bar = chart.last()?;

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };
        if bar.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), bar.ts);
        }

        let full = chart.bar_full_size(bar)?;

        if full == Size::GreatestBig {
            let ticker = asset.ticker();

            let title = format!("{ticker} Bar 1M");
            let body = format!("full = {}%", bar.full().abs_p());

            let notice = Notice::new(title, body, NoticePriority::Low);
            return Some(notice);
        }

        None
    }
}

#[derive(Default)]
struct AnomalBar5M {
    storage_ts: HashMap<String, i64>,
}
impl Condition for AnomalBar5M {
    fn name(&self) -> &'static str {
        "anomal bar 5M"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let chart = asset.chart(TimeFrame::M5)?;
        let bar = chart.last()?;

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };

        if bar.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), bar.ts);
        }

        let full = chart.bar_full_size(bar)?;

        if full as u8 >= Size::VeryBig as u8 {
            let ticker = asset.ticker();

            let title = format!("{ticker} Bar 5M");
            let body = format!("full = {}%", bar.full().abs_p());

            let notice = Notice::new(title, body, NoticePriority::Normal);
            return Some(notice);
        }

        None
    }
}

#[derive(Default)]
struct AnomalBar15M {
    storage_ts: HashMap<String, i64>,
}
impl Condition for AnomalBar15M {
    fn name(&self) -> &'static str {
        "anomal bar 15M"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let chart = asset.chart(TimeFrame::M15)?;
        let bar = chart.last()?;

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };

        if bar.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), bar.ts);
        }

        let full = chart.bar_full_size(bar)?;

        if full as u8 >= Size::VeryBig as u8 {
            let ticker = asset.ticker();

            let title = format!("{ticker} Bar 15M");
            let body = format!("full = {}%", bar.full().abs_p());

            let notice = Notice::new(title, body, NoticePriority::Normal);
            return Some(notice);
        }

        None
    }
}

#[derive(Default)]
struct AnomalBar1H {
    storage_ts: HashMap<String, i64>,
}
impl Condition for AnomalBar1H {
    fn name(&self) -> &'static str {
        "anomal bar 1H"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let chart = asset.chart(TimeFrame::H1)?;
        let bar = chart.last()?;

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };

        if bar.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), bar.ts);
        }

        let full = chart.bar_full_size(bar)?;

        if full as u8 >= Size::VeryBig as u8 {
            let ticker = asset.ticker();

            let title = format!("{ticker} Bar 1H");
            let body = format!("full = {}%", bar.full().abs_p());

            let notice = Notice::new(title, body, NoticePriority::Critical);
            return Some(notice);
        }

        None
    }
}

#[derive(Default)]
struct AnomalBar4H {
    storage_ts: HashMap<String, i64>,
}
impl Condition for AnomalBar4H {
    fn name(&self) -> &'static str {
        "anomal bar 4H"
    }
    fn apply(&mut self, asset: &avin_core::Asset) -> Option<Notice> {
        let chart = asset.chart(TimeFrame::H4)?;
        let bar = chart.last()?;

        // инициализация 0 если первый раз здесь, иначе значение из стореджа
        let last_ts = if let Some(value) = self.storage_ts.get(asset.figi()) {
            *value
        } else {
            0
        };

        if bar.ts <= last_ts {
            return None;
        } else {
            self.storage_ts.insert(asset.figi().clone(), bar.ts);
        }

        let full = chart.bar_full_size(bar)?;

        if full as u8 >= Size::VeryBig as u8 {
            let ticker = asset.ticker();

            let title = format!("{ticker} Bar 4H");
            let body = format!("full = {}%", bar.full().abs_p());

            let notice = Notice::new(title, body, NoticePriority::Critical);
            return Some(notice);
        }

        None
    }
}
