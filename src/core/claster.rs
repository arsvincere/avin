/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use polars::prelude::*;

use super::{Asset, Direction, Tic, TimeFrame};

#[derive(Debug, Clone)]
pub struct Quant {
    pub price: f64,
    pub vol_b: u64,
    pub vol_s: u64,
    pub val_b: f64,
    pub val_s: f64,
}
impl Quant {
    pub fn new(price: f64) -> Self {
        Self {
            price,
            val_b: 0.0,
            val_s: 0.0,
            vol_b: 0,
            vol_s: 0,
        }
    }

    pub fn add(&mut self, tic: &Tic) {
        assert!(self.price == tic.price);

        if tic.direction == Direction::Buy {
            self.vol_b += tic.lots as u64;
            self.val_b += tic.value;
        } else {
            self.vol_s += tic.lots as u64;
            self.val_s += tic.value;
        }
    }
    pub fn vol(&self) -> u64 {
        self.vol_b + self.vol_s
    }
    pub fn val(&self) -> f64 {
        self.val_b + self.val_s
    }
}

#[derive(Debug)]
pub struct Quantum {
    pub quants: Vec<Quant>,
}
impl Quantum {
    // build
    pub fn from_tics(tics: &Vec<Tic>) -> Self {
        assert!(tics.len() > 0);

        // select unique prices
        let mut prices = Vec::new();
        for tic in tics.iter() {
            prices.push(tic.price);
        }
        let unique = Series::new("prices".into(), prices).unique().unwrap();

        // create quants
        let mut quants = Vec::new();
        for price in unique.f64().unwrap().into_no_null_iter() {
            let mut quant = Quant::new(price);

            for tic in tics.iter() {
                if tic.price == price {
                    quant.add(tic);
                }
            }

            quants.push(quant);
        }

        Self { quants }
    }
    pub fn from_quants(quants: Vec<Quant>) -> Self {
        assert!(quants.len() > 0);

        Self { quants }
    }

    // public
    pub fn df(&self) -> DataFrame {
        // tmp vec
        let mut prices = Vec::new();
        let mut vol_b = Vec::new();
        let mut vol_s = Vec::new();
        let mut val_b = Vec::new();
        let mut val_s = Vec::new();

        // collect values
        for quant in self.quants.iter() {
            prices.push(quant.price);
            vol_b.push(quant.vol_b);
            vol_s.push(quant.vol_s);
            val_b.push(quant.val_b);
            val_s.push(quant.val_s);
        }

        // create & return df
        df!(
            "price" => prices,
            "vol_b" => vol_b,
            "vol_s" => vol_s,
            "val_b" => val_b,
            "val_s" => val_s,
        )
        .unwrap()
    }
    pub fn poc(&self) -> Quant {
        let mut max = self.quants.first().clone().unwrap();

        for q in self.quants.iter() {
            if q.vol() > max.vol() {
                max = q;
            }
        }

        max.clone()
    }
}

#[derive(Debug)]
pub struct Claster<'a> {
    tics: &'a Vec<Tic>,
    asset: &'a Asset,
    tf: TimeFrame,

    pub ts_nanos: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vol: u64,
    pub vol_b: u64,
    pub vol_s: u64,
    pub val: f64,
    pub val_b: f64,
    pub val_s: f64,
    pub count: u64,
    pub count_b: u64,
    pub count_s: u64,
    pub vwap: f64,
    pub vwap_b: f64,
    pub vwap_s: f64,
    pub buy_p: f64,
    pub sell_p: f64,
    pub disb_p: f64,
    pub pct: f64,
    pub quantum: Quantum,
    // pub var: f64,
    // pub std: f64,
}
impl<'a> Claster<'a> {
    // build
    pub fn new(
        asset: &'a Asset,
        tf: &'_ TimeFrame,
        tics: &'a Vec<Tic>,
    ) -> Claster<'a> {
        let open = Self::eval_open(tics);
        let high = Self::eval_high(tics);
        let low = Self::eval_low(tics);
        let close = Self::eval_close(tics);
        let pct = (close - open) / open * 100.0;

        let vol_b = Self::eval_vol_b(tics);
        let vol_s = Self::eval_vol_s(tics);
        let vol = vol_b + vol_s;

        let val_b = Self::eval_val_b(tics);
        let val_s = Self::eval_val_s(tics);
        let val = val_b + val_s;

        let count_b = Self::eval_count_b(tics);
        let count_s = Self::eval_count_s(tics);
        let count = count_b + count_s;

        let buy_p = val_b / val * 100.0;
        let sell_p = val_s / val * 100.0;
        let disb_p = buy_p - sell_p;

        Self {
            asset,
            tf: tf.clone(),
            tics,

            ts_nanos: Self::eval_ts(tics),
            open,
            high,
            low,
            close,
            vol,
            vol_b,
            vol_s,
            val,
            val_b,
            val_s,
            count,
            count_b,
            count_s,
            vwap: Self::eval_vwap(tics),
            vwap_b: Self::eval_vwap_b(tics),
            vwap_s: Self::eval_vwap_s(tics),
            buy_p,
            sell_p,
            disb_p,
            pct,
            quantum: Quantum::from_tics(tics),
        }
    }

    // public
    pub fn asset(&self) -> &Asset {
        self.asset
    }
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
    }
    pub fn tics(&self) -> &Vec<Tic> {
        self.tics
    }
    pub fn df(&self) -> DataFrame {
        df!(
            "ts" => [self.ts_nanos],
            "open" => [self.open],
            "high" => [self.high],
            "low" => [self.low],
            "close" => [self.close],
            "vol" => [self.vol],
            "vol_b" => [self.vol_b],
            "vol_s" => [self.vol_s],
            "val" => [self.val],
            "val_b" => [self.val_b],
            "val_s" => [self.val_s],
            "count" => [self.count],
            "count_b" => [self.count_b],
            "count_s" => [self.count_s],
            "vwap" => [self.vwap],
            "vwap_b" => [self.vwap_b],
            "vwap_s" => [self.vwap_s],
            "buy_p" => [self.buy_p],
            "sell_p" => [self.sell_p],
            "disb_p" => [self.disb_p],
            "pct" => [self.pct],
            // "var" => [Option::None::<f64>],
            // "std" => [Option::None::<f64>],
        )
        .unwrap()
    }
    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts_nanos);
        let local: DateTime<Local> = DateTime::from(utc);

        local.naive_local()
    }

    // private
    fn eval_ts(tics: &Vec<Tic>) -> i64 {
        tics.first().unwrap().ts_nanos
    }
    fn eval_open(tics: &Vec<Tic>) -> f64 {
        tics.first().unwrap().price
    }
    fn eval_high(tics: &Vec<Tic>) -> f64 {
        let mut max = 0.0;

        for tic in tics.iter() {
            if tic.price > max {
                max = tic.price
            }
        }

        max
    }
    fn eval_low(tics: &Vec<Tic>) -> f64 {
        let mut min = tics.first().unwrap().price;

        for tic in tics.iter() {
            if tic.price < min {
                min = tic.price
            }
        }

        min
    }
    fn eval_close(tics: &Vec<Tic>) -> f64 {
        tics.last().unwrap().price
    }
    fn eval_vol_b(tics: &Vec<Tic>) -> u64 {
        let mut vol = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                vol += tic.lots as u64
            }
        }

        vol
    }
    fn eval_vol_s(tics: &Vec<Tic>) -> u64 {
        let mut vol = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                vol += tic.lots as u64
            }
        }

        vol
    }
    fn eval_val_b(tics: &Vec<Tic>) -> f64 {
        let mut val = 0.0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                val += tic.value
            }
        }

        val
    }
    fn eval_val_s(tics: &Vec<Tic>) -> f64 {
        let mut val = 0.0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                val += tic.value
            }
        }

        val
    }
    fn eval_count_b(tics: &Vec<Tic>) -> u64 {
        let mut count = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                count += 1;
            }
        }

        count
    }
    fn eval_count_s(tics: &Vec<Tic>) -> u64 {
        let mut count = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                count += 1;
            }
        }

        count
    }
    fn eval_vwap(tics: &Vec<Tic>) -> f64 {
        // Средневзвешенная цена — средняя цена сделок с учетом объема
        let mut sum = 0.0;
        let mut vol = 0;
        for tic in tics.iter() {
            sum += tic.price * tic.lots as f64;
            vol += tic.lots;
        }

        sum / vol as f64
    }
    fn eval_vwap_b(tics: &Vec<Tic>) -> f64 {
        // Средневзвешенная цена покупки — это средняя цена покупки,
        // весом которых является объем соответствующих сделок
        let mut sum = 0.0;
        let mut vol = 0;
        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                sum += tic.price * tic.lots as f64;
                vol += tic.lots;
            }
        }
        sum / vol as f64
    }
    fn eval_vwap_s(tics: &Vec<Tic>) -> f64 {
        // Средневзвешенная цена покупки — это средняя цена покупки,
        // весом которых является объем соответствующих сделок
        let mut sum = 0.0;
        let mut vol = 0;
        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                sum += tic.price * tic.lots as f64;
                vol += tic.lots;
            }
        }
        sum / vol as f64
    }
    // fn eval_var(tics: &Vec<Tic>) -> f64 {
    //     // Дисперсия цены — мера разброса значений цены относительно
    //     // её средневзвешенной цены
    //     // Дисперсией называют среднее квадрата отклонения величины
    //     // от её средней.
    //
    //     // за среднюю в данном случае принимаем vwap
    //     let _e = vwap();
    //     for _tic in tics.iter() {
    //         // let delta = tic.price - e;
    //         todo!()
    //     }
    //     todo!()
    // }
    // fn eval_std(tics: &Vec<Tic>) -> f64 {
    //     // Стандартное отклонение цены
    //     // — это мера волатильности, показывающая, насколько сильно цена
    //     // акции отклоняется от средневзвешенной.
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quant() {
        let price = 320.5;

        let mut quant = Quant::new(price);
        assert_eq!(quant.price, price);
        assert_eq!(quant.val_b, 0.0);
        assert_eq!(quant.val_s, 0.0);

        let b = Tic::new(100500, Direction::Buy, 1, 320.5, 320.5);
        let s = Tic::new(100500, Direction::Sell, 1, 320.5, 320.5);
        quant.add(&b);
        quant.add(&s);
        quant.add(&s);
        assert_eq!(quant.price, price);
        assert_eq!(quant.vol_b, 1);
        assert_eq!(quant.vol_s, 2);
        assert_eq!(quant.val_b, 320.5);
        assert_eq!(quant.val_s, 641.0);
        assert_eq!(quant.vol(), 3);
        assert_eq!(quant.val(), 320.5 + 641.0);

        quant.add(&b);
        assert_eq!(quant.price, price);
        assert_eq!(quant.vol_b, 2);
        assert_eq!(quant.vol_s, 2);
        assert_eq!(quant.val_b, 641.0);
        assert_eq!(quant.val_s, 641.0);
        assert_eq!(quant.vol(), 4);
        assert_eq!(quant.val(), 641.0 + 641.0);
    }
    #[test]
    fn quantum() {
        let path = std::path::Path::new(
            "/home/alex/avin/usr/data/MOEX/SHARE/GAZP/TIC/2025/2025-06-06.pqt",
        );
        let df = crate::Cmd::read_pqt(path).unwrap();
        let df = df.tail(Some(10));
        // ┌─────────────────────┬───────────┬────────┬──────┬──────────┐
        // │ ts_nanos            ┆ direction ┆ price  ┆ lots ┆ value    │
        // │ ---                 ┆ ---       ┆ ---    ┆ ---  ┆ ---      │
        // │ i64                 ┆ str       ┆ f64    ┆ i64  ┆ f64      │
        // ╞═════════════════════╪═══════════╪════════╪══════╪══════════╡
        // │ 1749241800000000000 ┆ S         ┆ 125.83 ┆ 32   ┆ 40265.6  │
        // │ 1749241800000000000 ┆ S         ┆ 125.82 ┆ 5    ┆ 6291.0   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 5    ┆ 6290.5   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 309  ┆ 388752.9 │
        // │ 1749241802000000000 ┆ S         ┆ 125.81 ┆ 585  ┆ 735988.5 │
        // │ 1749241804000000000 ┆ B         ┆ 125.84 ┆ 60   ┆ 75504.0  │
        // │ 1749241804000000000 ┆ B         ┆ 125.84 ┆ 40   ┆ 50336.0  │
        // └─────────────────────┴───────────┴────────┴──────┴──────────┘

        let tics = Tic::from_df(df).unwrap();
        let quantum = Quantum::from_tics(&tics);
        let _df = quantum.df();
        // ┌────────┬───────┬───────┬──────────┬───────────┐
        // │ price  ┆ vol_b ┆ vol_s ┆ val_b    ┆ val_s     │
        // │ ---    ┆ ---   ┆ ---   ┆ ---      ┆ ---       │
        // │ f64    ┆ u64   ┆ u64   ┆ f64      ┆ f64       │
        // ╞════════╪═══════╪═══════╪══════════╪═══════════╡
        // │ 125.81 ┆ 0     ┆ 905   ┆ 0.0      ┆ 1138580.5 │
        // │ 125.82 ┆ 0     ┆ 5     ┆ 0.0      ┆ 6291.0    │
        // │ 125.83 ┆ 0     ┆ 32    ┆ 0.0      ┆ 40265.6   │
        // │ 125.84 ┆ 100   ┆ 0     ┆ 125840.0 ┆ 0.0       │
        // └────────┴───────┴───────┴──────────┴───────────┘
        let poc = quantum.poc();
        assert_eq!(poc.price, 125.81);
        assert_eq!(poc.vol(), 905);
        assert_eq!(poc.val(), 1138580.5);
    }
    #[test]
    fn claster() {
        let asset = crate::Asset::new("moex_share_gazp").unwrap();
        let tf = crate::TimeFrame::Day; // not affect anything in this case
        let path = std::path::Path::new(
            "/home/alex/avin/usr/data/MOEX/SHARE/GAZP/TIC/2025/2025-06-06.pqt",
        );
        let df = crate::Cmd::read_pqt(path).unwrap();
        let df = df.tail(Some(10));
        // ┌─────────────────────┬───────────┬────────┬──────┬──────────┐
        // │ ts_nanos            ┆ direction ┆ price  ┆ lots ┆ value    │
        // │ ---                 ┆ ---       ┆ ---    ┆ ---  ┆ ---      │
        // │ i64                 ┆ str       ┆ f64    ┆ i64  ┆ f64      │
        // ╞═════════════════════╪═══════════╪════════╪══════╪══════════╡
        // │ 1749241800000000000 ┆ S         ┆ 125.83 ┆ 32   ┆ 40265.6  │
        // │ 1749241800000000000 ┆ S         ┆ 125.82 ┆ 5    ┆ 6291.0   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 5    ┆ 6290.5   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 2    ┆ 2516.2   │
        // │ 1749241800000000000 ┆ S         ┆ 125.81 ┆ 309  ┆ 388752.9 │
        // │ 1749241802000000000 ┆ S         ┆ 125.81 ┆ 585  ┆ 735988.5 │
        // │ 1749241804000000000 ┆ B         ┆ 125.84 ┆ 60   ┆ 75504.0  │
        // │ 1749241804000000000 ┆ B         ┆ 125.84 ┆ 40   ┆ 50336.0  │
        // └─────────────────────┴───────────┴────────┴──────┴──────────┘

        let tics = Tic::from_df(df).unwrap();
        let claster = Claster::new(&asset, &tf, &tics);
        assert_eq!(claster.ts_nanos, 1749241800000000000);
        assert_eq!(claster.open, 125.83);
        assert_eq!(claster.high, 125.84);
        assert_eq!(claster.low, 125.81);
        assert_eq!(claster.close, 125.84);
        assert_eq!(claster.vol, 1042);
        assert_eq!(claster.vol_b, 100);
        assert_eq!(claster.vol_s, 942);
        assert_eq!(claster.val, 1.3109771e6);
        assert_eq!(claster.val_b, 125840.0);
        assert_eq!(claster.val_s, 1.1851371e6);
        assert_eq!(claster.count, 10);
        assert_eq!(claster.count_b, 2);
        assert_eq!(claster.count_s, 8);
        assert_eq!(claster.vwap, 125.81354126679462);
        assert_eq!(claster.vwap_b, 125.84);
        assert_eq!(claster.vwap_s, 125.81073248407644);
        assert_eq!(claster.buy_p, 9.598947227987429);
        assert_eq!(claster.sell_p, 90.40105277201256);
        assert_eq!(claster.disb_p, -80.80210554402514);
        assert_eq!(claster.pct, 0.007947230390213078);
    }
}
