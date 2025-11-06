/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod example;
mod filter;

pub use example::MyCondition;
pub use filter::{
    Condition, Marker, MarkerColor, MarkerShape, MarkerSize, Scanner,
    ScannerResult, ScannerResultList,
};
