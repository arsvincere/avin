/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Asset;

use crate::Notice;

pub trait Condition: 'static {
    fn name(&self) -> &'static str;
    fn apply(&mut self, asset: &Asset) -> Option<Notice>;
}
