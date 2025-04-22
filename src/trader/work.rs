use crate::core::{Asset, Event, Share};
use crate::strategy::Every;

pub struct Work {
    share: Share,
    strategys: Vec<Every>,
    in_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    in_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl Work {
    pub fn new(share: Share) -> Self {
        let (in_tx, in_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            share,
            strategys: Vec::new(),
            in_tx,
            in_rx,
        }
    }

    pub fn figi(&self) -> &String {
        self.share.figi()
    }
    pub fn add_strategy(&mut self, strategy: Every) {
        self.strategys.push(strategy);
    }
    pub fn get_sender(&self) -> tokio::sync::mpsc::UnboundedSender<Event> {
        self.in_tx.clone()
    }

    pub async fn start(&mut self) {
        while let Some(e) = self.in_rx.recv().await {
            self.share.receive(e);
            self.process_strategy().await;
        }
    }

    // private
    async fn process_strategy(&mut self) {
        for strategy in self.strategys.iter_mut() {
            strategy.process(&self.share).await;
        }
    }
}
