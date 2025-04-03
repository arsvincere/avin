/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::transaction::Transaction;

#[derive(Debug)]
pub struct LimitOrder {}
#[derive(Debug)]
pub struct NewLimitOrder {}
#[derive(Debug)]
pub struct PostedLimitOrder {}
#[derive(Debug)]
pub struct RejectedLimitOrder {}
#[derive(Debug)]
pub struct PartialLimitOrder {}
#[derive(Debug)]
pub struct FilledLimitOrder {}
