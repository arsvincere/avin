/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod _strategy;
mod examples;
mod scanner;

pub use _strategy::Strategy;
pub use examples::{BuySell, PinBarLong};
pub use scanner::{
    Filter, Marker, MarkerColor, MarkerShape, MarkerSize, Scanner,
    ScannerResult,
};
