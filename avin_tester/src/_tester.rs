/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use avin_core::{Action, Asset, Event, TimeFrame};
use avin_strategy::Strategy;

use super::{Test, TestStatus, VirtualBroker};

pub struct Tester {
    tx: UnboundedSender<Action>,
    rx: UnboundedReceiver<Action>,
}
impl Tester {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Tester { tx, rx }
    }

    pub async fn run(
        &mut self,
        mut strategy: impl Strategy,
        test: &mut Test,
    ) {
        log::info!(":: Tester clear test");
        test.clear();

        log::info!(":: Tester load broker");
        let mut broker = VirtualBroker::new(test);
        let broker_tx = broker.get_sender();

        log::info!(":: Tester load account");
        let account = broker.get_virtual_account();

        log::info!(":: Tester load asset");
        let mut asset = Asset::from_iid(test.iid.clone());
        self.load_charts(&mut asset);

        log::info!(":: Tester load strategys");
        let sender = self.tx.clone();
        strategy.init(sender, account, asset.iid().clone());

        log::info!(":: Tester start main loop");
        test.status = TestStatus::Process;
        while let Some(e) = broker.next_event() {
            match e {
                Event::Bar(e) => {
                    // PERF: чтобы 5 раз не дергать стратегию на обновление
                    // после 1М, 5М, 10М, 1Н, Day... дергаю ее только на
                    // обновлении Day бара, чаще все равно смысла нет,
                    // а вызовов в 5 раз меньше. Бар стрим выдает бары от
                    // 1М до Day, то есть в момент прихода Day как раз
                    // все графики обновлены на текущую минуту.
                    if e.tf == TimeFrame::Day {
                        asset.bar_event(e);
                        strategy.process(&asset);
                    } else {
                        asset.bar_event(e);
                    }
                }
                Event::Tic(e) => {
                    asset.tic_event(e);
                    // strategy.process(&asset).await;
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
        Test::save(test).unwrap();
    }

    // private
    fn load_charts(&mut self, asset: &mut Asset) {
        log::info!(":: Tester load charts {asset}");

        for tf in TimeFrame::all() {
            log::info!("   {}", tf);
            asset.load_chart_empty(&tf);
        }
    }
}
impl Default for Tester {
    fn default() -> Self {
        Tester::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;
    use avin_strategy::BuySell;

    #[tokio::test]
    async fn run_test() {
        let strategy = BuySell::default();
        let asset = Asset::new("moex_share_sber").unwrap();

        let mut test = Test::new(&strategy, asset.iid());
        test.set_begin(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap());
        test.set_end(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 9, 0).unwrap());
        assert_eq!(test.status, TestStatus::New);

        let mut tester = Tester::new();
        tester.run(strategy, &mut test).await;
        assert_eq!(test.status, TestStatus::Complete);
        assert_eq!(test.trade_list.len(), 4);

        Test::delete(&test).unwrap();
    }
}
