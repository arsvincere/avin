/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::transaction::Transaction;

#[derive(Debug)]
pub struct StopOrder {}
#[derive(Debug)]
pub struct NewStopOrder {}
#[derive(Debug)]
pub struct PostedStopOrder {}
#[derive(Debug)]
pub struct RejectedStopOrder {}
