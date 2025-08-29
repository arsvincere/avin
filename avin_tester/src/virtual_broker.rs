/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use avin_core::{
    Account, Action, Bar,
    Direction::{self, Sell},
    Event, LimitOrder, MarketOrder, Order, OrderAction, OrderEvent,
    PostedLimitOrder, PostedMarketOrder, PostedStopOrder, StopOrder,
    StopOrderKind::{StopLoss, TakeProfit},
    TimeFrame, Transaction, TriggeredStopOrder,
};
use avin_data::Iid;

use super::data_stream::DataStream;
use super::test::Test;

pub struct VirtualBroker {
    tx: UnboundedSender<Action>,
    rx: UnboundedReceiver<Action>,
    data_stream: DataStream,
    account: Account,
    strategy_name: String,
    commission: f64,

    current_bar: Bar,
    queue: VecDeque<Event>,
    market_orders: Vec<MarketOrder>,
    limit_orders: Vec<LimitOrder>,
    stop_orders: Vec<StopOrder>,
    need_check_orders: bool,
}
impl VirtualBroker {
    pub fn new(test: &Test) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let data_stream = Self::create_marketdata_stream(
            &test.iid,
            test.begin(),
            test.end(),
        )
        .unwrap();

        VirtualBroker {
            tx,
            rx,
            data_stream,
            account: Account::new("VirtualAccount", "Virtual_ID"),
            strategy_name: test.strategy_name.clone(),
            commission: test.commission,

            current_bar: Bar::new(0, 0.0, 0.0, 0.0, 0.0, 0),
            queue: VecDeque::new(),
            market_orders: Vec::new(),
            limit_orders: Vec::new(),
            stop_orders: Vec::new(),
            need_check_orders: false,
        }
    }

    pub fn get_virtual_account(&self) -> Account {
        self.account.clone()
    }
    pub fn get_sender(&self) -> UnboundedSender<Action> {
        self.tx.clone()
    }
    pub fn next_event(&mut self) -> Option<Event> {
        // process actions from strategys
        while let Ok(a) = self.rx.try_recv() {
            match a {
                Action::GetAccount(_) => todo!(),
                Action::GetBars(_) => todo!(),
                Action::Post(a) => self.post_action(a),
                Action::Cancel(a) => self.cancel_action(a),
                Action::TradeOpened(_) => unreachable!(),
                Action::TradeClosed(_) => unreachable!(),
                Action::Subscribe(_) => unreachable!(),
                Action::Unsubscribe(_) => unreachable!(),
            }
        }

        // если в очереди есть event (а там будут Event::Order) - выдать его
        let e = self.queue.pop_front();
        if e.is_some() {
            return e;
        }

        // чекаем ордера в текущем баре
        if self.need_check_orders {
            self.check_all_orders();
        }

        // Иначе: достать новый эвент из дата стрима
        if let Some(e) = self.data_stream.next_event() {
            self.queue.push_back(e.clone());
            match e {
                Event::Bar(e) => {
                    // NOTE: тестер вызывает стратегии только на обновлении
                    // 1М бара, соответственно новые ордера могут прилететь
                    // только после него, а когда обновляется 5М, 10М, 1Н...
                    // не проверяем сработку ордеров. Тем более ее надо
                    // смотреть только в 1М баре последнем(!), иначе будут
                    // неправильные сработки, если в дневном например
                    // с диапазоном в 2% в середине находимся, и постится
                    // какой нибудь ордер +/- 0.3% он в дневном баре
                    // будет сразу исполнен - что будет ошибкой. Поэтому
                    // тут проверяется таймфрейм бар эвента и current_bar
                    // обновляется только на 1М
                    if e.tf == TimeFrame::M1 {
                        self.current_bar = e.bar;
                        self.need_check_orders = true;
                    } else {
                        self.need_check_orders = false;
                    };
                }
                Event::Tic(_) => todo!("Обработка тиков виртуал брокером..."),
                Event::Order(_) => unreachable!("OrderEvent in data stream?"),
            }

            // достать из очереди первый эвент и выдать его
            return self.queue.pop_front();
        }

        None
    }

    // private
    fn create_marketdata_stream(
        iid: &Iid,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataStream, &'static str> {
        let stream = DataStream::new(iid, begin, end);

        Ok(stream)
    }
    fn post_action(&mut self, action: OrderAction) {
        let posted_order = match action.order {
            Order::Market(order) => self.post_market(order),
            Order::Limit(order) => self.post_limit(order),
            Order::Stop(order) => self.post_stop(order),
        };

        let e = OrderEvent::new(
            action.account,
            action.iid,
            action.owner,
            posted_order,
        );
        let e = Event::Order(e);
        self.queue.push_back(e);
    }
    fn post_market(&mut self, order: MarketOrder) -> Order {
        // unwrap
        let new_order = order
            .as_new()
            .expect("Order must have status 'New', posting failed");

        // create broker id
        let broker_id = uuid::Uuid::new_v4().to_string();

        // change status
        let posted_order = new_order.post(&broker_id);

        // wrap
        let posted_order = MarketOrder::Posted(posted_order);

        // save copy
        self.market_orders.push(posted_order.clone());

        // wrap & return posted order
        Order::Market(posted_order)
    }
    fn post_limit(&mut self, order: LimitOrder) -> Order {
        // unwrap
        let new_order = order
            .as_new()
            .expect("Order must have status 'New', posting failed");

        // create broker id
        let broker_id = uuid::Uuid::new_v4().to_string();

        // change status
        let posted_order = new_order.post(&broker_id);

        // wrap
        let posted_order = LimitOrder::Posted(posted_order);

        // save copy
        self.limit_orders.push(posted_order.clone());

        // wrap & return posted order
        Order::Limit(posted_order)
    }
    fn post_stop(&mut self, order: StopOrder) -> Order {
        // unwrap
        let new_order = order
            .as_new()
            .expect("Order must have status 'New', posting failed");

        // create broker id
        let broker_id = uuid::Uuid::new_v4().to_string();

        // change status
        let posted_order = new_order.post(&broker_id);

        // wrap
        let posted_order = StopOrder::Posted(posted_order);

        // save copy
        self.stop_orders.push(posted_order.clone());

        // wrap & return posted order
        Order::Stop(posted_order)
    }
    fn check_all_orders(&mut self) {
        self.check_all_orders_market();
        self.check_all_orders_limit();
        self.check_all_orders_stop();
    }
    fn check_all_orders_market(&mut self) {
        let bar = self.current_bar;

        while let Some(order) = self.market_orders.pop() {
            // unwrap
            let order = order.as_posted().unwrap();

            // exec in current bar
            self.exec_market(bar.ts_nanos, bar.c, order);
        }
    }
    fn check_all_orders_limit(&mut self) {
        let bar = self.current_bar;
        let mut i = 0;

        while i < self.limit_orders.len() {
            // unwrap PostedLimitOrder
            let limit_order = &self.limit_orders[i];
            let posted = limit_order.clone().as_posted().unwrap();

            // если бар содержит цену лимитки -> по цене лимитки
            if self.current_bar.contains(posted.price) {
                self.exec_limit(bar.ts_nanos, posted.price, posted);
                self.limit_orders.remove(i);
            }
            // бар открылся под лимиткой на покупку -> по цене открытия
            else if posted.direction == Direction::Buy {
                if bar.o < posted.price {
                    self.exec_limit(bar.ts_nanos, posted.price, posted);
                    self.limit_orders.remove(i);
                }
            }
            // бар открылся над лимиткой на продажу -> по цене открытия
            else if posted.direction == Direction::Sell {
                if bar.o > posted.price {
                    self.exec_limit(bar.ts_nanos, posted.price, posted);
                    self.limit_orders.remove(i);
                }
            }
            // ордер не исполнен, переходим к следующему
            else {
                i += 1;
            }
        }
    }
    fn check_all_orders_stop(&mut self) {
        let bar = self.current_bar;
        let ts = bar.ts_nanos;
        let mut i = 0;

        while i < self.stop_orders.len() {
            // unwrap PostedStopOrder
            let stop_order = &self.stop_orders[i];
            let posted = match stop_order {
                StopOrder::Posted(order) => order,
                _ => panic!("WTF??? Тут должны быть только 'posted' ордера"),
            };

            // если бар содержит цену сработки...
            if bar.contains(posted.stop_price) {
                self.trigger_stop(ts, posted.stop_price, posted.clone());
                self.stop_orders.remove(i);
                continue;
            }

            // если StopLoss
            if posted.kind == StopLoss {
                // Трейд в лонг, order.direction == Sell
                if posted.direction == Sell {
                    // open ниже stop_price, trigger по цене открытия бара
                    if bar.o < posted.stop_price {
                        self.trigger_stop(ts, bar.o, posted.clone());
                        self.stop_orders.remove(i);
                        continue;
                    }
                }
                // Трейд в шорт, order.direction == Buy
                else {
                    // open выше stop_price, trigger по цене открытия бара
                    if bar.o > posted.stop_price {
                        self.trigger_stop(ts, bar.o, posted.clone());
                        self.stop_orders.remove(i);
                        continue;
                    }
                }
            }

            // если TakeProfit
            if posted.kind == TakeProfit {
                // Трейд в лонг, order.direction == Sell
                if posted.direction == Sell {
                    // open выше stop_price, trigger по цене открытия бара
                    if bar.o > posted.stop_price {
                        self.trigger_stop(ts, bar.o, posted.clone());
                        self.stop_orders.remove(i);
                        continue;
                    }
                }
                // Трейд в шорт, order.direction == Buy
                else {
                    // open ниже stop_price, trigger по цене открытия бара
                    if bar.o < posted.stop_price {
                        self.trigger_stop(ts, bar.o, posted.clone());
                        self.stop_orders.remove(i);
                        continue;
                    }
                }
            }

            // ордер не исполнен, переходим к следующему
            i += 1;
        }
    }
    fn exec_market(
        &mut self,
        ts_nanos: i64,
        price: f64,
        mut order: PostedMarketOrder,
    ) {
        // create transaction
        let quantity = order.lots * self.data_stream.iid.lot();
        let transaction = Transaction::new(quantity as i32, price);
        let commission = transaction.value() * self.commission;
        order.add_transaction(transaction);

        // change status
        let order = order.fill(ts_nanos, commission);

        // wrap
        let order = Order::Market(MarketOrder::Filled(order));

        // create order event and push in queue
        let e = OrderEvent::new(
            self.account.clone(),
            self.data_stream.iid.clone(),
            self.strategy_name.clone(),
            order,
        );
        let e = Event::Order(e);
        self.queue.push_back(e);
    }
    fn exec_limit(
        &mut self,
        ts_nanos: i64,
        price: f64,
        mut order: PostedLimitOrder,
    ) {
        // create transaction
        let quantity = order.lots * self.data_stream.iid.lot();
        let transaction = Transaction::new(quantity as i32, price);
        let commission = transaction.value() * self.commission;
        order.add_transaction(transaction);

        // change status
        let order = order.fill(ts_nanos, commission);

        // wrap
        let order = Order::Limit(LimitOrder::Filled(order));

        // create order event and push in queue
        let e = OrderEvent::new(
            self.account.clone(),
            self.data_stream.iid.clone(),
            self.strategy_name.clone(),
            order,
        );
        let e = Event::Order(e);
        self.queue.push_back(e);
    }
    fn trigger_stop(
        &mut self,
        _ts_nanos: i64,
        _price: f64,
        order: PostedStopOrder,
    ) {
        let bar = &self.current_bar;
        let id = order.broker_id.clone();
        let triggered = order.trigger(&id);

        match triggered {
            TriggeredStopOrder::Limit(order) => {
                let order = LimitOrder::Posted(order);
                self.limit_orders.push(order);
                self.check_all_orders_limit();
            }
            TriggeredStopOrder::Market(order) => {
                self.exec_market(bar.ts_nanos, bar.c, order);
            }
        };
    }

    fn cancel_action(&mut self, action: OrderAction) {
        let canceled_order_opt = match action.clone().order {
            Order::Market(_) => unreachable!("Cancel market order? Really?"),
            Order::Limit(order) => self.cancel_limit(order),
            Order::Stop(order) => self.cancel_stop(order),
        };

        if let Some(canceled_order) = canceled_order_opt {
            let e = OrderEvent::new(
                action.account,
                action.iid,
                action.owner,
                canceled_order,
            );
            let e = Event::Order(e);
            self.queue.push_back(e);
        } else {
            log::error!("WTF?! Action: {action:#?}");
            unreachable!();
        }
    }
    fn cancel_limit(&mut self, order: LimitOrder) -> Option<Order> {
        let mut i = 0;

        while i < self.limit_orders.len() {
            let posted = &self.limit_orders[i];

            if posted.broker_id() == order.broker_id() {
                // if exist -> remove
                self.limit_orders.remove(i);

                // unwrap
                let order = order.as_posted().unwrap();
                let canceled = order.cancel();

                // wrap and return
                return Some(Order::Limit(LimitOrder::Canceled(canceled)));
            }

            i += 1;
        }

        None
    }
    fn cancel_stop(&mut self, order: StopOrder) -> Option<Order> {
        let mut i = 0;

        while i < self.stop_orders.len() {
            let posted = &self.stop_orders[i];

            if posted.broker_id() == order.broker_id() {
                // if exist -> remove
                self.stop_orders.remove(i);

                // unwrap
                let order = order.as_posted().unwrap();
                let canceled = order.cancel();

                // wrap and return
                return Some(Order::Stop(StopOrder::Canceled(canceled)));
            }

            i += 1;
        }

        // ордер не найден, значит уже сработал или уже отменяли
        // пофигу отправим его как отмененный
        let order = order.as_posted().unwrap();
        let canceled = order.cancel();

        // wrap and return
        Some(Order::Stop(StopOrder::Canceled(canceled)))
    }
}
