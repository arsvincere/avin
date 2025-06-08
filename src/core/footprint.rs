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
    clasters: Vec<Cluster>,
    now: Option<Cluster>,
}
impl Footprint {
    pub fn new(
        iid: &IID,
        tf: &TimeFrame,
        clasters: Vec<Cluster>,
    ) -> Footprint {
        assert!(clasters.len() > 0);

        Self {
            iid: iid.clone(),
            tf: tf.clone(),
            clasters,
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
            clasters: Self::split(tics, tf),
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
    pub fn clasters(&self) -> &Vec<Cluster> {
        &self.clasters
    }
    pub fn now(&self) -> Option<&Cluster> {
        self.now.as_ref()
    }
    pub fn df(&self) -> DataFrame {
        assert!(self.clasters.len() > 0);

        let mut all = self.clasters[0].df();

        for i in 1..self.clasters.len() {
            let df = self.clasters[i].df();
            all.extend(&df).unwrap();
        }

        all
    }

    // private
    fn split(tics: &Vec<Tic>, tf: &TimeFrame) -> Vec<Cluster> {
        assert!(tics.len() > 0);

        // output clasters
        let mut clasters = Vec::new();

        // key func for search
        let key = |x: &Tic| x.ts_nanos;

        // split tics by part == timeframe
        let mut b = tics.first().unwrap().ts_nanos;
        let mut e = tf.next_ts(b);
        let last = tics.last().unwrap().ts_nanos;
        while b < last {
            let i = utils::bisect_left(tics, b, key).unwrap();
            let j = utils::bisect_left(tics, e, key).unwrap();
            let selected = &tics[i..j];

            let cluster = Cluster::new(selected, tf);
            clasters.push(cluster);

            b = e;
            e = tf.next_ts(e);
        }

        clasters
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
