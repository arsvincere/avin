/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::prelude::*;

use crate::data::Category;
use crate::data::IID;

use crate::Chart;
use crate::TimeFrame;

// TODO: make it enum, with Share, Bond, Future...
// and call functions through match
pub trait Asset {
    fn iid(&self) -> &IID;
    fn exchange(&self) -> &String;
    fn category(&self) -> Category;
    fn ticker(&self) -> &String;
    fn figi(&self) -> &String;
    fn info(&self) -> &HashMap<String, String>;
    fn path(&self) -> PathBuf;

    fn chart(&self, tf: &TimeFrame) -> Option<&Chart>;
    fn mut_chart(&mut self, tf: &TimeFrame) -> Option<&mut Chart>;
    fn load_chart(&mut self, tf: &TimeFrame) -> Result<&Chart, &'static str>;
    fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str>;
    fn load_chart_empty(&mut self, tf: &TimeFrame) -> &Chart;
}
