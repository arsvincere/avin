/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::frame::DataFrame;

use avin_utils as utils;

use crate::{Cluster, Iid, Tic, TimeFrame};

/// Aggregation of instrument id, timeframe and clusters.
///
/// # ru
/// Кластерный график. Содержит идентификатор инструмента, таймфрейм,
/// кластеры.
///
/// Кластеры рассчитываются из тиков. Тики хранятся в родительском
/// активе, график заимствует тики, рассчитывает по ним кластеры. То есть
/// тики хранятся в одном экземпляре и используются всеми кластерными
/// графиками разных таймфреймов.
///
/// Структура новая, возможны изменения. Документации по методам пока нет.
#[derive(Debug)]
pub struct Footprint {
    iid: Iid,
    tf: TimeFrame,
    clusters: Vec<Cluster>,
    now: Option<Cluster>,
}
impl Footprint {
    pub fn new(
        iid: &Iid,
        tf: &TimeFrame,
        clusters: Vec<Cluster>,
    ) -> Footprint {
        assert!(!clusters.is_empty());

        Self {
            iid: iid.clone(),
            tf: *tf,
            clusters,
            now: None,
        }
    }
    pub fn from_tics(iid: &Iid, tf: &TimeFrame, tics: &[Tic]) -> Footprint {
        assert!(!tics.is_empty());

        Self {
            iid: iid.clone(),
            tf: *tf,
            clusters: Self::split(tics, tf),
            now: None,
        }
    }

    // public
    pub fn iid(&self) -> &Iid {
        &self.iid
    }
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
    }
    pub fn clusters(&self) -> &Vec<Cluster> {
        &self.clusters
    }
    pub fn now(&self) -> Option<&Cluster> {
        self.now.as_ref()
    }
    pub fn df(&self) -> DataFrame {
        assert!(!self.clusters.is_empty());

        let mut all = self.clusters[0].df();

        for i in 1..self.clusters.len() {
            let df = self.clusters[i].df();
            all.extend(&df).unwrap();
        }

        all
    }

    // private
    fn split(tics: &[Tic], tf: &TimeFrame) -> Vec<Cluster> {
        assert!(!tics.is_empty());

        // output clusters
        let mut clusters = Vec::new();

        // key func for search
        let key = |x: &Tic| x.ts_nanos;

        // split tics by part == timeframe
        let mut ts1 = tics.first().unwrap().ts_nanos;
        let mut ts2 = tf.next_ts(ts1);
        let mut b = 0;
        let mut e;
        while b < tics.len() {
            b = utils::bisect_right(tics, ts1, key).unwrap();
            e = utils::bisect_right(tics, ts2, key).unwrap_or(tics.len());
            let selected = &tics[b..e];

            if !selected.is_empty() {
                let cluster = Cluster::new(selected, tf);
                clusters.push(cluster);
            }

            ts1 = tf.next_ts(ts1);
            ts2 = tf.next_ts(ts2);
            b = e;
        }

        clusters
    }
}
impl AsRef<Footprint> for Footprint {
    fn as_ref(&self) -> &Footprint {
        self
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: test it
}
