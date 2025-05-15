/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod extremum;
mod size;
mod term;
mod trend;

pub use extremum::{Extremum, ExtremumKind};
pub use size::Size;
pub use term::{Term, Term::T1, Term::T2, Term::T3, Term::T4, Term::T5};
pub use trend::Trend;
