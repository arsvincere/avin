/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::{Account, Action, Asset, OrderEvent};

type Trader = tokio::sync::mpsc::UnboundedSender<Action>;

pub trait Strategy: Send + 'static {
    fn name(&self) -> &'static str;
    fn init(&mut self, trader: Trader, account: Account, asset: &mut Asset);
    fn process(&mut self, asset: &Asset);
    fn order_event(&mut self, event: OrderEvent);
}
