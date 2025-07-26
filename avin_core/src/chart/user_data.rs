/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::DataFrame;

use crate::Bar;

pub trait UserData: Send {
    fn id(&self) -> &str;
    fn df(&self) -> &DataFrame;
    fn init(&mut self, bars: &[Bar], now: Option<&Bar>);
    fn update(&mut self, bars: &[Bar], now: Option<&Bar>);
}
