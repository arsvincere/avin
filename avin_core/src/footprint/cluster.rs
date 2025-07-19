/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use polars::prelude::*;

use crate::{Direction, Quantum, Tic, TimeFrame};

/// Set of metrics calculated on tics.
///
/// # ru
/// Группа метрик рассчитанных на тиках за определенный период (таймфрейм).
///
/// Отдельной частью является [`Quantum`], который состоит из
/// [`crate::Quant`]. Все это разные метрики над одними и теми же тиками
/// разбитыми по таймфреймам.
///
/// Quant - это ценовой уровень. Внутри него посчитано раздельно количество
/// продаж и покупок именно по этой цене. Группа квантов в пределах одного
/// бара объединяется в Quantum.
///
/// Остальные метрики рассчитаны по всем тикам в целом, без привязки к
/// конкретной цене.
#[derive(Debug)]
pub struct Cluster {
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
impl Cluster {
    // build
    pub fn new(tics: &[Tic], tf: TimeFrame) -> Cluster {
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
            ts_nanos: Self::eval_ts(tics, tf),
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
    pub fn schema() -> Schema {
        Schema::from_iter(vec![
            Field::new("ts_nanos".into(), DataType::Int64),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("vol".into(), DataType::UInt64),
            Field::new("vol_b".into(), DataType::UInt64),
            Field::new("vol_s".into(), DataType::UInt64),
            Field::new("val".into(), DataType::Float64),
            Field::new("val_b".into(), DataType::Float64),
            Field::new("val_s".into(), DataType::Float64),
            Field::new("count".into(), DataType::UInt64),
            Field::new("count_b".into(), DataType::UInt64),
            Field::new("count_s".into(), DataType::UInt64),
            Field::new("vwap".into(), DataType::Float64),
            Field::new("vwap_b".into(), DataType::Float64),
            Field::new("vwap_s".into(), DataType::Float64),
            Field::new("buy_p".into(), DataType::Float64),
            Field::new("sell_p".into(), DataType::Float64),
            Field::new("disb_p".into(), DataType::Float64),
            Field::new("pct".into(), DataType::Float64),
        ])
    }

    // public
    pub fn df(&self) -> DataFrame {
        df!(
            "ts_nanos" => [self.ts_nanos],
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
    fn eval_ts(tics: &[Tic], tf: TimeFrame) -> i64 {
        let first_tic_ts = tics.first().unwrap().ts_nanos;

        tf.prev_ts(first_tic_ts)
    }
    fn eval_open(tics: &[Tic]) -> f64 {
        tics.first().unwrap().price
    }
    fn eval_high(tics: &[Tic]) -> f64 {
        let mut max = 0.0;

        for tic in tics.iter() {
            if tic.price > max {
                max = tic.price
            }
        }

        max
    }
    fn eval_low(tics: &[Tic]) -> f64 {
        let mut min = tics.first().unwrap().price;

        for tic in tics.iter() {
            if tic.price < min {
                min = tic.price
            }
        }

        min
    }
    fn eval_close(tics: &[Tic]) -> f64 {
        tics.last().unwrap().price
    }
    fn eval_vol_b(tics: &[Tic]) -> u64 {
        let mut vol = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                vol += tic.lots as u64
            }
        }

        vol
    }
    fn eval_vol_s(tics: &[Tic]) -> u64 {
        let mut vol = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                vol += tic.lots as u64
            }
        }

        vol
    }
    fn eval_val_b(tics: &[Tic]) -> f64 {
        let mut val = 0.0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                val += tic.value
            }
        }

        val
    }
    fn eval_val_s(tics: &[Tic]) -> f64 {
        let mut val = 0.0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                val += tic.value
            }
        }

        val
    }
    fn eval_count_b(tics: &[Tic]) -> u64 {
        let mut count = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Buy {
                count += 1;
            }
        }

        count
    }
    fn eval_count_s(tics: &[Tic]) -> u64 {
        let mut count = 0;

        for tic in tics.iter() {
            if tic.direction == Direction::Sell {
                count += 1;
            }
        }

        count
    }
    fn eval_vwap(tics: &[Tic]) -> f64 {
        // Средневзвешенная цена — средняя цена сделок с учетом объема
        let mut sum = 0.0;
        let mut vol = 0;
        for tic in tics.iter() {
            sum += tic.price * tic.lots as f64;
            vol += tic.lots;
        }

        sum / vol as f64
    }
    fn eval_vwap_b(tics: &[Tic]) -> f64 {
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
    fn eval_vwap_s(tics: &[Tic]) -> f64 {
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
    // fn eval_var(tics: &[Tic]) -> f64 {
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
    // fn eval_std(tics: &[Tic]) -> f64 {
    //     // Стандартное отклонение цены
    //     // — это мера волатильности, показывающая, насколько сильно цена
    //     // акции отклоняется от средневзвешенной.
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avin_utils as utils;

    #[test]
    fn cluster() {
        let path = std::path::Path::new(
            "/home/alex/trading/data/MOEX/SHARE/GAZP/TIC/2025/2025-06-06.parquet",
        );
        let df = utils::Cmd::read_pqt(path).unwrap();
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

        let tf = TimeFrame::M1;
        let tics = Tic::from_df(&df).unwrap();
        let c = Cluster::new(&tics, tf);
        assert_eq!(c.ts_nanos, TimeFrame::M1.prev_ts(1749241800000000000));
        assert_eq!(c.open, 125.83);
        assert_eq!(c.high, 125.84);
        assert_eq!(c.low, 125.81);
        assert_eq!(c.close, 125.84);
        assert_eq!(c.vol, 1042);
        assert_eq!(c.vol_b, 100);
        assert_eq!(c.vol_s, 942);
        assert_eq!(c.val, 1.3109771e6);
        assert_eq!(c.val_b, 125840.0);
        assert_eq!(c.val_s, 1.1851371e6);
        assert_eq!(c.count, 10);
        assert_eq!(c.count_b, 2);
        assert_eq!(c.count_s, 8);
        assert_eq!(c.vwap, 125.81354126679462);
        assert_eq!(c.vwap_b, 125.84);
        assert_eq!(c.vwap_s, 125.81073248407644);
        assert_eq!(c.buy_p, 9.598947227987429);
        assert_eq!(c.sell_p, 90.40105277201256);
        assert_eq!(c.disb_p, -80.80210554402514);
        assert_eq!(c.pct, 0.007947230390213078);
    }
}
