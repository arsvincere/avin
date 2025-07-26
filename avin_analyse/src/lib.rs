/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod analyse;
mod bar;
mod cluster;
mod size;
mod trend;

pub use analyse::Analyse;
pub use bar::BarAnalytic;
pub use size::{Size, Sz};
pub use trend::TrendAnalytic;
