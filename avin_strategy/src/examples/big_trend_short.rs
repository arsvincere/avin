/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use crate::Strategy;
use avin_analyse::{Size, TrendAnalytic};
use avin_core::{
    Account, Action, Asset, Chart, Direction, ExtremumIndicator, Iid,
    MarketData, MarketOrder, Order, OrderAction, OrderEvent, StopOrder,
    StopOrderKind, Term, TimeFrame, Trade, TradeKind,
};
use avin_search::Condition;
use avin_utils as utils;

const NAME: &str = "BigTrend-S-1.1";
const LOTS: u32 = 10;
const STOP: f64 = 1.01;
const TAKE: f64 = 0.98;

type Trader = tokio::sync::mpsc::UnboundedSender<Action>;

#[derive(Debug, Default)]
enum Status {
    #[default]
    /// Стратегия наблюдает, ждет условий для входа
    Observe,
    /// Условия наступили, постит ордер открытия позиции
    PostingSell,
    /// Ожидает исполнения ордера
    Opening,
    /// Когда трейд открыт постит стоп лосс
    PostingStop,
    /// Потом постит тейк профит
    PostingTake,
    /// Стратегия имеет активный трейд, ничего не делает, ждет его закрытия
    Active,
    /// Стратегия закрыла трейд и ждет отмены остальных ордеров
    Canceling,
}

#[derive(Debug, Default)]
pub struct BigTrendShort {
    trader: Option<Trader>,
    account: Option<Account>,
    iid: Option<Iid>,

    status: Status,
    last_ts: i64,
    trade: Option<Trade>,
    sell_order: Option<Order>,
    stop_loss: Option<Order>,
    take_profit: Option<Order>,
}

impl Strategy for BigTrendShort {
    fn name(&self) -> &'static str {
        NAME
    }
    fn init(&mut self, trader: Trader, account: Account, asset: &mut Asset) {
        let tf = TimeFrame::Day;
        let chart = asset.chart_mut(tf).unwrap();
        ExtremumIndicator::init(chart);
        TrendAnalytic::init(chart);

        let tf = TimeFrame::H1;
        let chart = asset.chart_mut(tf).unwrap();
        ExtremumIndicator::init(chart);
        TrendAnalytic::init(chart);

        let tf = TimeFrame::M10;
        let chart = asset.chart_mut(tf).unwrap();
        ExtremumIndicator::init(chart);
        TrendAnalytic::init(chart);

        self.trader = Some(trader);
        self.account = Some(account);
        self.iid = Some(asset.iid().clone());

        self.last_ts = 0;
    }
    fn process(&mut self, asset: &Asset) {
        if let Status::Observe = self.status {
            self.observe(asset)
        }
    }
    fn order_event(&mut self, e: OrderEvent) {
        match self.status {
            Status::Observe => unreachable!(),
            Status::PostingSell => self.on_sell_event(e),
            Status::Opening => self.on_opening_event(e),
            Status::PostingStop => self.on_posting_stop_event(e),
            Status::PostingTake => self.on_posting_take_event(e),
            Status::Active => self.on_active_event(e),
            Status::Canceling => self.on_canceling_event(e),
        }
    }
}
impl BigTrendShort {
    fn observe(&mut self, asset: &Asset) {
        // if !self.observe_10m(asset) {
        //     return;
        // }

        if !self.observe_1h(asset) {
            return;
        }

        // if !self.observe_d(asset) {
        //     return;
        // }

        // все выполнено, открываем сделку
        self.sell();
    }
    fn observe_10m(&mut self, asset: &Asset) -> bool {
        let tf = TimeFrame::M10;
        let chart = asset.chart(tf).unwrap();

        let trend = match chart.trend(Term::T3, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bear() {
            return false;
        }
        let size = chart.trend_abs_size(trend).unwrap();
        if size == Size::Biggest {
            return true;
        }

        false
    }
    fn observe_1h(&mut self, asset: &Asset) -> bool {
        let tf = TimeFrame::H1;
        let chart = asset.chart(tf).unwrap();

        let trend = match chart.trend(Term::T1, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bear() {
            return false;
        }

        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if !(0.70..=0.90).contains(&cdf) {
            return false;
        }

        if trend.len() < 5 {
            return false;
        }

        true
    }
    fn observe_d(&mut self, asset: &Asset) -> bool {
        // берем график
        let tf = TimeFrame::Day;
        let chart = asset.chart(tf).unwrap();

        // если долгосрочный Д тренд бычий выходим
        let trend = match chart.trend(Term::T3, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bull() {
            return false;
        }

        true
    }

    fn sell(&mut self) {
        // создаем трейд
        self.create_trade();

        // отправляем ордер трейдеру
        self.send_sell_order();
    }
    fn create_trade(&mut self) {
        let trade = Trade::new(
            self.last_ts,
            self.name(),
            TradeKind::Short,
            self.iid.clone().unwrap(),
        );
        self.trade = Some(Trade::New(trade));
    }
    fn send_sell_order(&mut self) {
        // Создаем ордер
        let order = MarketOrder::new(Direction::Sell, LOTS);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        self.sell_order = Some(order.clone());

        // Заворачиваем в экшен
        let a = OrderAction::new(
            self.account.clone().unwrap(),
            self.iid.clone().unwrap(),
            self.name(),
            order,
        );
        let a = Action::Post(a);

        // Отправляем трейдеру
        self.trader.as_ref().unwrap().send(a).unwrap();

        // Ставим статус - отправка ордера на покупку
        self.status = Status::PostingSell;
    }
    fn on_sell_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе PostingSell
        // и прилетает ордер эвент, по отправленному ордеру на продажу.

        // Достаем из эвента ордер
        let order = e.order;

        // Если ордер выставлен - ставим статус "открываемся", и выходим
        // На практике этот статус едва ли будет получен, маркет ордер
        // исполнится сразу. Но если бы открывались по лимитке что более
        // практично, то такая логика была бы уместна.
        if order.is_posted() {
            self.sell_order = Some(order);
            self.status = Status::Opening;
            return;
        }

        // Если ордер полностью исполнен (что скорее всего, маркет
        // ордер исполняется сразу)
        if order.is_filled() {
            // достаем трейд
            let trade = self.trade.take().unwrap();
            // разворачиваем из него NewTrade
            let trade = trade.as_new().unwrap();
            // вызываем у него метод open передав ордер которым открылись
            // получается OpenedTrade
            let trade = trade.open(order);
            // заворачиваем OpenedTrade в Trade и сохраняем
            self.trade = Some(Trade::Opened(trade));
            // начинаем постить стоп лосс
            self.post_stop();
        }
    }
    fn on_opening_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе Opening
        // и прилетает ордер эвент, по отправленному ордеру на покупку.

        // Достаем из эвента ордер
        let order = e.order;

        if order.is_filled() {
            // достаем трейд
            let trade = self.trade.take().unwrap();
            // разворачиваем из него NewTrade
            let trade = trade.as_new().unwrap();
            // вызываем у него метод open передав ордер которым открылись
            // получается OpenedTrade
            let trade = trade.open(order);
            // заворачиваем OpenedTrade в Trade и сохраняем
            let trade = Trade::Opened(trade);
            self.trade = Some(trade);
            // начинаем постить стоп лосс
            self.post_stop();
        }
    }
    fn post_stop(&mut self) {
        // достаем трейд
        let trade = self.trade.take().unwrap();

        // разворачиваем из него OpenedTrade
        let trade = trade.as_opened().unwrap();

        // получаем среднюю цену позиции
        let price = trade.avg();

        // рассчитываем цену стопа
        let stop = price * STOP;

        // округляем цену до минимального шага цены, иначе не выставится
        let min_step = self.iid.as_ref().unwrap().step();
        let stop = utils::round_price(stop, min_step);

        // создаем стоп лосс
        let stop_order = StopOrder::new(
            StopOrderKind::StopLoss,
            Direction::Buy,
            LOTS,
            stop,
            None, // цена сработки ордера, None - будет рыночное исполнение
        );

        // заворачиваем ордер
        let order = StopOrder::New(stop_order);
        let order = Order::Stop(order);

        // сохраняем для себя на всякий, не обязательно.
        // Если например захочется переставлять стоп по мере изменения
        // цены, то удобно иметь его под рукой, а не доставать каждый раз
        // из трейда.
        self.stop_loss = Some(order.clone());

        // Заворачиваем ордер в экшен
        let a = OrderAction::new(
            self.account.clone().unwrap(),
            self.iid.clone().unwrap(),
            self.name(),
            order,
        );
        let a = Action::Post(a);

        // отправляем трейдеру
        self.trader.as_ref().unwrap().send(a).unwrap();

        // сохраняем обратно трейд
        self.trade = Some(Trade::Opened(trade));

        // статус - выставляем стоп
        self.status = Status::PostingStop;
    }
    fn on_posting_stop_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе PostingStop
        // и прилетает ордер эвент, по отправленному stop loss.

        // Достаем из эвента ордер
        let order = e.order;

        if order.is_posted() {
            // достаем PostedStopOrder
            let order = order.as_stop().unwrap().as_posted().unwrap();

            // достаем трейд
            let trade = self.trade.take().unwrap();

            // разворачиваем из него OpenedTrade
            let mut trade = trade.as_opened().unwrap();

            // крепим стоп ордер к трейду
            trade.set_stop(order.clone());

            // заворачиваем OpenedTrade в Trade и сохраняем
            let trade = Trade::Opened(trade);
            self.trade = Some(trade);

            // сохраняем у себя запостенный стоп
            self.stop_loss = Some(Order::Stop(StopOrder::Posted(order)));

            // начинаем постить тейк профит
            self.post_take();
        } else {
            // тут могла бы быть логика что делать если ордер отклонен
            // например. Но для примера это избыточно, если денег хватает
            // ордер обычно выставляется.
            panic!("ордер таки не выставился");
        }
    }
    fn post_take(&mut self) {
        // достаем трейд
        let trade = self.trade.take().unwrap();

        // разворачиваем из него OpenedTrade
        let trade = trade.as_opened().unwrap();

        // получаем среднюю цену позиции
        let price = trade.avg();

        // рассчитываем цену тейк профита
        let stop = price * TAKE;

        // округляем цену до минимального шага цены, иначе не выставится
        let min_step = self.iid.as_ref().unwrap().step();
        let stop = utils::round_price(stop, min_step);

        // создаем тейк профит
        let stop_order = StopOrder::new(
            StopOrderKind::TakeProfit,
            Direction::Buy,
            LOTS,
            stop,
            Some(stop), // цена исполнения ордера == цена сработки
        );

        // заворачиваем ордер
        let order = StopOrder::New(stop_order);
        let order = Order::Stop(order);

        // сохраняем для себя на всякий, не обязательно.
        // Если например захочется переставлять стоп по мере изменения
        // цены, то удобно иметь его под рукой, а не доставать каждый раз
        // из трейда.
        self.take_profit = Some(order.clone());

        // Заворачиваем ордер в экшен
        let a = OrderAction::new(
            self.account.clone().unwrap(),
            self.iid.clone().unwrap(),
            self.name(),
            order,
        );
        let a = Action::Post(a);

        // отправляем трейдеру
        self.trader.as_ref().unwrap().send(a).unwrap();

        // сохраняем обратно трейд
        self.trade = Some(Trade::Opened(trade));

        // статус - выставляем тейк
        self.status = Status::PostingTake;
    }
    fn on_posting_take_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе PostingTake
        // и прилетает ордер эвент, по отправленному take profit

        // Достаем из эвента ордер
        let order = e.order;

        if order.is_posted() {
            // достаем PostedStopOrder
            let order = order.as_stop().unwrap().as_posted().unwrap();

            // достаем трейд
            let trade = self.trade.take().unwrap();
            // разворачиваем из него OpenedTrade
            let mut trade = trade.as_opened().unwrap();

            // крепим стоп ордер к трейду
            trade.set_take(order.clone());

            // заворачиваем OpenedTrade в Trade и сохраняем
            let trade = Trade::Opened(trade);
            self.trade = Some(trade);

            // сохраняем у себя запостенный тейк
            self.take_profit = Some(Order::Stop(StopOrder::Posted(order)));

            // статус - Active, ждем закрытия трейда
            self.status = Status::Active;
        } else {
            // тут могла бы быть логика что делать если ордер отклонен
            // например. Но для примера это избыточно, если денег хватает
            // ордер обычно выставляется.
            panic!("ордер таки не выставился");
        }
    }
    fn on_active_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе Active
        // и прилетает ордер эвент - сработал стоп или тейк

        // Достаем из эвента ордер
        let order = e.order;

        // Далее для простоты смотрим только на эвенты где ордер
        // полностью исполнен, остальные игнорим. Триггеред стоп будет
        // просто проигнорирован. Сработавший тейк, в выставленной но
        // еще не исполненной лимиткой тоже будет проигнорирован. Для
        // простоты примера опустим все эти детали.

        // Вариант - сработал стоп лосс
        if order.is_market() && order.is_filled() {
            // отменяем тейк профит
            let a = Action::Cancel(OrderAction::new(
                self.account.clone().unwrap(),
                self.iid.clone().unwrap(),
                self.name(),
                self.take_profit.take().unwrap(),
            ));
            self.trader.as_ref().unwrap().send(a).unwrap();

            // достаем трейд
            let mut trade = self.trade.take().unwrap().as_opened().unwrap();

            // сохраняем исполненный ордер в трейде
            trade.add_order(order);

            // закрываем трейд
            let trade = trade.close();

            // заворачиваем трейд в экшен
            let trade = Trade::Closed(trade);
            let a = Action::TradeClosed(trade);

            // отправляем трейдеру
            self.trader.as_ref().unwrap().send(a).unwrap();

            // Ставим статус Canceling, ждем отмены второго стоп ордера
            self.status = Status::Canceling;

            // тут еще могла бы быть логика проверки не вышла ли стратегия
            // за пределы допустимых для нее убытков, возможно остановка
            // стратегии и тп..
            return;
        }

        // Вариант - сработал тейк профит
        if order.is_limit() && order.is_filled() {
            // отменяем стоп лосс
            let a = Action::Cancel(OrderAction::new(
                self.account.clone().unwrap(),
                self.iid.clone().unwrap(),
                self.name(),
                self.stop_loss.take().unwrap(),
            ));
            self.trader.as_ref().unwrap().send(a).unwrap();

            // достаем трейд
            let mut trade = self.trade.take().unwrap().as_opened().unwrap();

            // сохраняем исполненный ордер в трейде
            trade.add_order(order);

            // закрываем трейд
            let trade = trade.close();

            // заворачиваем трейд в экшен
            let trade = Trade::Closed(trade);
            let a = Action::TradeClosed(trade);

            // отправляем трейдеру
            self.trader.as_ref().unwrap().send(a).unwrap();

            // Ставим статус Canceling, ждем отмены второго стоп ордера
            self.status = Status::Canceling;
        }
    }
    fn on_canceling_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе Canceling
        // и прилетает ордер эвент - отменен 2й стоп ордер который не сработал

        // Достаем из эвента ордер
        let order = e.order;

        // если это стоп ордер и он отменен
        if order.is_stop() && order.is_canceled() {
            // все один рабочий цикл закончен, снова ставим статус Observe
            self.status = Status::Observe;
        }
    }
}

#[derive(Default)]
struct Filter10M {}
impl Condition for Filter10M {
    fn name(&self) -> &'static str {
        "my_filter"
    }
    fn apply(&self, chart: &Chart) -> bool {
        let trend = match chart.trend(Term::T1, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bear() {
            return false;
        }
        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if cdf < 0.80 {
            return false;
        }

        let trend = match chart.trend(Term::T2, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bear() {
            return false;
        }
        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if cdf < 0.60 {
            return false;
        }

        true
    }
}
