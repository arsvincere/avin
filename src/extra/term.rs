/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, PartialOrd, EnumIter)]
pub enum Term {
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
}
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::T1 => write!(f, "T1"),
            Self::T2 => write!(f, "T2"),
            Self::T3 => write!(f, "T3"),
            Self::T4 => write!(f, "T4"),
            Self::T5 => write!(f, "T5"),
        }
    }
}
