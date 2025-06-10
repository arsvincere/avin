/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::frame::DataFrame;

use crate::IID;
use crate::utils;

use super::{Cluster, Tic, TimeFrame};

#[derive(Debug)]
pub struct Footprint {
    iid: IID,
    tf: TimeFrame,
    clusters: Vec<Cluster>,
    now: Option<Cluster>,
}
impl Footprint {
    pub fn new(
        iid: &IID,
        tf: &TimeFrame,
        clusters: Vec<Cluster>,
    ) -> Footprint {
        assert!(clusters.len() > 0);

        Self {
            iid: iid.clone(),
            tf: tf.clone(),
            clusters,
            now: None,
        }
    }
    pub fn from_tics(
        iid: &IID,
        tf: &TimeFrame,
        tics: &Vec<Tic>,
    ) -> Footprint {
        assert!(tics.len() > 0);

        Self {
            iid: iid.clone(),
            tf: tf.clone(),
            clusters: Self::split(tics, tf),
            now: None,
        }
    }

    // public
    pub fn iid(&self) -> &IID {
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
        assert!(self.clusters.len() > 0);

        let mut all = self.clusters[0].df();

        for i in 1..self.clusters.len() {
            let df = self.clusters[i].df();
            all.extend(&df).unwrap();
        }

        all
    }

    // private
    fn split(tics: &Vec<Tic>, tf: &TimeFrame) -> Vec<Cluster> {
        assert!(tics.len() > 0);

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

            if selected.len() > 0 {
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
        &self
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: test it
}
