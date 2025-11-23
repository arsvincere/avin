/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::{
    Action, Event, GetBarsAction, LimitOrder, MarketData, MarketOrder, Order,
    OrderAction, OrderEvent, StreamAction, TimeFrame,
};
use avin_utils::AvinError;

use crate::tinkoff::TinkoffClient;

pub struct Tinkoff {
    event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    action_rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
    client: TinkoffClient,
}
impl Tinkoff {
    pub fn new(
        action_rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
        event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    ) -> Self {
        Self {
            event_tx: event_tx.clone(),
            action_rx,
            client: TinkoffClient::new(event_tx),
        }
    }
    pub async fn connect(&mut self) -> Result<(), AvinError> {
        self.client.connect().await.unwrap();

        Ok(())
    }
    pub async fn create_marketdata_stream(&mut self) -> Result<(), AvinError> {
        self.client.create_marketdata_stream().await.unwrap();

        Ok(())
    }
    pub async fn create_transactions_stream(
        &mut self,
    ) -> Result<(), AvinError> {
        self.client.create_transactions_stream().await.unwrap();

        Ok(())
    }
    pub async fn start(&mut self) {
        // receive actions main loop
        while let Some(a) = self.action_rx.recv().await {
            match a {
                Action::GetAccount(_a) => todo!(),
                Action::GetBars(a) => self.get_bars_action(a).await,
                Action::Post(a) => {
                    self.post_action(a).await;
                }
                Action::Cancel(_) => todo!(),
                Action::Subscribe(a) => {
                    self.subscribe_action(a).await;
                }
                Action::Unsubscribe(_) => todo!(),
                Action::TradeClosed(_) => unreachable!(),
                Action::TradeOpened(_) => unreachable!(),
            }
        }
    }

    // private
    async fn get_bars_action(&mut self, a: GetBarsAction) {
        let bars = self
            .client
            .get_bars(&a.iid, a.tf, a.from, a.till)
            .await
            .unwrap();

        a.tx.send(bars).unwrap();
    }
    async fn post_action(&mut self, a: OrderAction) {
        let result = match a.order {
            Order::Market(market) => match market {
                MarketOrder::New(new_market) => {
                    self.client
                        .post_market(&a.account, &a.iid, new_market)
                        .await
                }
                _ => todo!(),
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(new_limit) => {
                    self.client.post_limit(&a.account, &a.iid, new_limit).await
                }
                _ => todo!(),
            },
            Order::Stop(_order) => todo!(),
        };

        let order = result.unwrap();
        let e = OrderEvent::new(a.account, a.iid, a.owner, order);
        let e = Event::Order(e);
        self.event_tx.send(e).unwrap();
    }
    async fn subscribe_action(&mut self, a: StreamAction) {
        for md in a.market_data_kinds {
            match md {
                MarketData::TIC => {
                    self.client.subscribe_tic(&a.iid).await.unwrap()
                }
                MarketData::ORDER_BOOK => {
                    self.client.subscribe_ob(&a.iid).await.unwrap()
                }
                MarketData::BAR_1M => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::M1)
                    .await
                    .unwrap(),
                MarketData::BAR_5M => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::M5)
                    .await
                    .unwrap(),
                MarketData::BAR_10M => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::M10)
                    .await
                    .unwrap(),
                MarketData::BAR_15M => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::M15)
                    .await
                    .unwrap(),
                MarketData::BAR_1H => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::H1)
                    .await
                    .unwrap(),
                MarketData::BAR_4H => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::H4)
                    .await
                    .unwrap(),
                MarketData::BAR_DAY => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::Day)
                    .await
                    .unwrap(),
                MarketData::BAR_WEEK => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::Week)
                    .await
                    .unwrap(),
                MarketData::BAR_MONTH => self
                    .client
                    .subscribe_bar(&a.iid, &TimeFrame::Month)
                    .await
                    .unwrap(),
                MarketData::SC_TRADE => unreachable!(),
                MarketData::SC_ORDER => unreachable!(),
                MarketData::SC_OB => unreachable!(),
            };
        }
    }
}
