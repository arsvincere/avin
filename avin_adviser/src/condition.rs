/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Asset;

use crate::Notice;

pub trait Condition {
    fn name(&self) -> &'static str;
    fn apply(&self, asset: &Asset) -> Option<Notice>;
}
