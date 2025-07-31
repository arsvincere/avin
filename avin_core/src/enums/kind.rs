/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// TODO: move to extremum, rename TrendKind
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Bear = -1,
    Bull = 1,
}
