/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::Action;
use crate::Asset;
use crate::Event;
use crate::Share;
use crate::Strategy;
use crate::TestStatus;
use crate::TimeFrame;

use super::Test;
use super::VirtualBroker;

pub struct Tester {
    tx: UnboundedSender<Action>,
    rx: UnboundedReceiver<Action>,
}
impl Tester {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Tester { tx, rx }
    }

    pub async fn run(&mut self, test: &mut Test) {
        log::info!(":: Tester clear test");
        test.clear();

        log::info!(":: Tester load broker");
        let mut broker = VirtualBroker::new(&test);
        let broker_tx = broker.get_sender();

        log::info!(":: Tester load account");
        let account = broker.get_virtual_account();

        log::info!(":: Tester load share");
        let mut share = Share::from_iid(test.iid.clone());
        self.load_charts(&mut share);

        log::info!(":: Tester load strategys");
        let name = &test.strategy_name;
        let sender = self.tx.clone();
        let mut strategy = Strategy::new(name, sender, account, share.iid());

        log::info!(":: Tester start main loop");
        test.status = TestStatus::Process;
        while let Some(e) = broker.next() {
            match e {
                Event::Bar(e) => {
                    // PERF: чтобы 5 раз не дергать стратегию на обновление
                    // после 1М, 5М, 10М, 1Н... дергаю ее только на обновлении
                    // 1М бара, чаще все равно смысла нет, а вызовов в 5 раз
                    // меньше.
                    if e.tf.name() == "1M" {
                        share.bar_event(e);
                        strategy.process(&share);
                    } else {
                        share.bar_event(e);
                    }
                }
                Event::Tic(e) => {
                    share.tic_event(e);
                    // strategy.process(&share).await;
                }
                Event::Order(e) => strategy.order_event(e),
            }

            // process actions from strategys
            while let Ok(a) = self.rx.try_recv() {
                match a {
                    Action::TradeClosed(trade) => {
                        test.trade_list.add(trade);
                    }
                    other => broker_tx.send(other).unwrap(),
                }
            }
        }

        test.status = TestStatus::Complete;
        Test::save(&test).unwrap();
    }

    // private
    fn load_charts(&mut self, share: &mut Share) {
        log::info!(":: Tester load charts {share}");

        for tf in TimeFrame::all() {
            log::info!("   {}", tf.name());
            share.load_chart_empty(&tf);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::*;

    #[tokio::test]
    async fn run_test() {
        let share = Share::new("moex_share_sber").unwrap();
        let mut test = Test::new("Every", share.iid());
        test.set_begin(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap());
        test.set_end(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 10, 0).unwrap());
        assert_eq!(test.status, TestStatus::New);

        let mut tester = Tester::new();
        tester.run(&mut test).await;
        assert_eq!(test.status, TestStatus::Complete);
        assert_eq!(test.trade_list.len(), 5);

        Test::delete(&test).unwrap();
    }
}
