/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::*;

use super::{Quant, Tic};

#[derive(Debug)]
pub struct Quantum {
    pub quants: Vec<Quant>,
}
impl Quantum {
    // build
    pub fn from_tics(tics: &[Tic]) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
