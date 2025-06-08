/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::{Event, Share};
use crate::strategy::Strategy;

pub struct Work {
    share: Share,
    strategys: Vec<Strategy>,
    in_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    in_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}
impl Work {
    pub fn new(share: Share) -> Work {
        let (in_tx, in_rx) = tokio::sync::mpsc::unbounded_channel();

        Work {
            share,
            strategys: Vec::new(),
            in_tx,
            in_rx,
        }
    }

    pub fn figi(&self) -> &String {
        self.share.figi()
    }
    pub fn add_strategy(&mut self, strategy: Strategy) {
        self.strategys.push(strategy);
    }
    pub fn get_sender(&self) -> tokio::sync::mpsc::UnboundedSender<Event> {
        self.in_tx.clone()
    }

    pub async fn start(&mut self) {
        while let Some(e) = self.in_rx.recv().await {
            match e {
                Event::Bar(e) => {
                    self.share.bar_event(e);
                    self.process_all_strategy();
                }
                Event::Tic(e) => {
                    self.share.tic_event(e);
                    // self.process_strategy();
                }
                Event::Order(e) => {
                    for strategy in self.strategys.iter_mut() {
                        if *strategy.name() == e.strategy_name {
                            strategy.order_event(e);
                            break;
                        }
                    }
                }
            }
        }
    }

    // private
    fn process_all_strategy(&mut self) {
        for strategy in self.strategys.iter_mut() {
            strategy.process(&self.share);
        }
    }
}
