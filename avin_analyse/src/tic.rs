/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::Utc;
use polars::prelude::DataFrame;
use strum::{EnumIter, IntoEnumIterator};

use avin_core::{Asset, Iid, Manager, MarketData, Share, Source, Tic};
use avin_utils::AvinError;

use crate::{Analyse, Size};

const NAME: &str = "tic";

#[derive(Debug, Clone, Copy, EnumIter)]
enum Metric {
    Cdf,
    Size,
    Sz,
}
impl Metric {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Cdf => "cdf",
            Self::Size => "size",
            Self::Sz => "sz",
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Feat {
    Lots,
    Value,
}
impl Feat {
    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Lots => "lots",
            Self::Value => "value",
        }
    }
}

impl Analyse for Tic {
    fn analyse() -> Result<(), AvinError> {
        let shares = Share::all();

        for share in shares.iter() {
            let result = analyse(share.iid());
            match result {
                Ok(_) => (),
                Err(e) => log::error!("{e}"),
            }
        }

        Ok(())
    }
}

// public interface for Asset
pub trait TicAnalytic {
    fn tic_lots_cdf(&self, tic: &Tic) -> Option<f64>;
    fn tic_value_cdf(&self, tic: &Tic) -> Option<f64>;

    fn tic_lots_size(&self, tic: &Tic) -> Option<Size>;
    fn tic_value_size(&self, tic: &Tic) -> Option<Size>;
}
impl TicAnalytic for Asset {
    fn tic_lots_cdf(&self, tic: &Tic) -> Option<f64> {
        match get_cdf_df(self, Feat::Lots) {
            Ok(cdf_df) => Some(Tic::cdf(tic.lots, cdf_df)),
            Err(_) => None,
        }
    }
    fn tic_value_cdf(&self, tic: &Tic) -> Option<f64> {
        match get_cdf_df(self, Feat::Value) {
            Ok(cdf_df) => Some(Tic::cdf(tic.value, cdf_df)),
            Err(_) => None,
        }
    }
    fn tic_lots_size(&self, tic: &Tic) -> Option<Size> {
        match get_sizes_df(self, Feat::Lots) {
            Ok(sizes) => Some(Tic::size(tic.lots, &sizes)),
            Err(_) => None,
        }
    }
    fn tic_value_size(&self, tic: &Tic) -> Option<Size> {
        match get_sizes_df(self, Feat::Value) {
            Ok(sizes) => Some(Tic::size(tic.value, &sizes)),
            Err(_) => None,
        }
    }
}

// analyse
fn analyse(iid: &Iid) -> Result<(), AvinError> {
    log::info!(":: Analyse {} {}", NAME, iid.ticker());

    // load tics
    let df = load_tics(iid)?;

    // analyse features
    for feat in Feat::iter() {
        analyse_feat(iid, &df, feat);
    }

    Ok(())
}
fn analyse_name(feat: Feat, metric: Metric) -> String {
    format!("{} {} {}", NAME, feat.name(), metric.name(),)
}
fn load_tics(iid: &Iid) -> Result<DataFrame, AvinError> {
    let md = MarketData::TIC;
    let source = Source::MOEXALGO;
    let end = Utc::now();
    let begin = end - avin_utils::ONE_YEAR;

    Manager::load(iid, source, md, begin, end)
}
fn analyse_feat(iid: &Iid, df: &DataFrame, feat: Feat) {
    log::info!("Analyse feat {}", feat.name());

    let metric = Metric::Cdf;
    let name = analyse_name(feat, metric);
    let mut cdf =
        Tic::eval_cdf(df.column(feat.name()).unwrap().as_materialized_series());
    Tic::save(iid, &name, &mut cdf);

    let metric = Metric::Size;
    let mut sizes = Tic::eval_size(&cdf);
    let name = analyse_name(feat, metric);
    Tic::save(iid, &name, &mut sizes);

    let metric = Metric::Sz;
    let mut szs = Tic::eval_sz(&cdf);
    let name = analyse_name(feat, metric);
    Tic::save(iid, &name, &mut szs);
}
fn get_cdf_df(asset: &Asset, feat: Feat) -> Result<DataFrame, AvinError> {
    let iid = asset.iid();
    let metric = Metric::Cdf;
    let name = analyse_name(feat, metric);

    Tic::load(iid, &name)
}
fn get_sizes_df(asset: &Asset, feat: Feat) -> Result<DataFrame, AvinError> {
    let iid = asset.iid();
    let metric = Metric::Size;
    let name = analyse_name(feat, metric);

    Tic::load(iid, &name)
}
