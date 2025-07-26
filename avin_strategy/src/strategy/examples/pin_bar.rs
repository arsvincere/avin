/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

//! Пример реализации простой стратегии pin-bar в лонг.
//! Если видим медвежий день, после него доджи молот - входит в сделку.
//! Ставит стоп 1% от входа и тейк 2% от входа.
//!
//! Не надо пытаться заработать этой стратегией! Это просто пример как
//! сделать стратегию исползуя библиотеку. Стратегия убыточна.

use crate::Strategy;
use avin_core::{
    Account, Action, Asset, Direction, Iid, MarketOrder, Order, OrderAction,
    OrderEvent, StopOrder, StopOrderKind, TimeFrame, Trade, TradeKind,
};
use avin_utils as utils;

/// Имя стратегии для себя, имя должно быть уникальным, используется как
/// ключ в HashMap. К одному инструменту может быть подключено несколько
/// стратегий.
const NAME: &str = "PinBarLong";

/// количество лотов которыми оперирует стратегия
const LOTS: u32 = 10;

/// Стоп лосс на 1%
const STOP: f64 = 0.99;

/// Тейк профит на 2%
const TAKE: f64 = 1.02;

type Trader = tokio::sync::mpsc::UnboundedSender<Action>;

/// Для удобства реализации логики стратегии можно опредлить статусы
/// в которых стратегия может находится. Но это не обязательно, поведение
/// стратегии полностью определяет пользователь.
#[derive(Debug, Default)]
enum Status {
    #[default]
    /// Стратегия наблюдает, ждет условий для входа
    Observe,
    /// Условия наступили, постит ордер на покупку
    PostingBuy,
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

/// Определяем структуру для своей стратегии
/// С любыми полями внутри, но там обязательно должно быть:
/// - trader - сендер к трейдеру
/// - account - аккаунт на котором работает стратегия
/// - iid - идентификатор инструмента по которому работает стратегия
#[derive(Debug, Default)]
pub struct PinBarLong {
    trader: Option<Trader>,
    account: Option<Account>,
    iid: Option<Iid>,

    status: Status,
    last_ts: i64,
    trade: Option<Trade>,
    buy_order: Option<Order>,
    stop_loss: Option<Order>,
    take_profit: Option<Order>,
}

/// Чтобы тестер или трейдер могли работать с этой стратегией нужно
/// имплементировать интерфейс Strategy.
impl Strategy for PinBarLong {
    /// Возвращает имя стратегии, будет фигурировать в отчетах например,
    /// как название папки в которой будут лежать результаты теста и тп.
    fn name(&self) -> &'static str {
        NAME
    }
    /// Инициализация стратегии - ей присваивается трейдер
    /// (сендер к трейдеру), аккаунт и идентификатор инструмента.
    /// Этот метод вызывается один раз перед запуском стратегии. В нем
    /// же следует разместить логику подготовки стратегии к работе, может
    /// ей нужно загрузить какие то данные и тп.
    fn init(&mut self, trader: Trader, account: Account, iid: Iid) {
        self.trader = Some(trader);
        self.account = Some(account);
        self.iid = Some(iid);

        self.last_ts = 0;
    }
    /// Функция вызывается каждый при обновлении бара, на каждом тике
    /// Передается ссылка на обновленный актив, через который можно
    /// получить доступ к графикам, тикам...
    fn process(&mut self, asset: &Asset) {
        if let Status::Observe = self.status {
            self.observe(asset)
        }
    }
    /// Функция вызывается каждый раз когда происходит что либо по
    /// ордеру выставленному этой стратегией: выставлен, отклонен,
    /// частично исполнен, исполнен, отменен пользователем...
    fn order_event(&mut self, e: OrderEvent) {
        match self.status {
            Status::Observe => unreachable!(),
            Status::PostingBuy => self.on_buy_event(e),
            Status::Opening => self.on_opening_event(e),
            Status::PostingStop => self.on_posting_stop_event(e),
            Status::PostingTake => self.on_posting_take_event(e),
            Status::Active => self.on_active_event(e),
            Status::Canceling => self.on_canceling_event(e),
        }
    }
}
/// Собственно пользовательская логика работы стратегии
impl PinBarLong {
    // private
    fn observe(&mut self, asset: &Asset) {
        // берем дневной график
        let tf = TimeFrame::Day;
        let chart = asset.chart(tf).unwrap();

        // позавчерашний бар есть в графике
        let b2 = chart.bar(2);
        if b2.is_none() {
            return;
        }
        let b2 = b2.unwrap();

        // Если уже проверяли этот бар -> return
        if b2.ts_nanos == self.last_ts {
            return;
        }

        // Обновляем время последней обработки
        self.last_ts = b2.ts_nanos;

        // позавчерашний бар медвежий не медвежий -> return
        if !b2.is_bear() {
            return;
        }

        // тело позавчерашнего бара больше 0.7%
        if b2.body().abs_p() < 0.7 {
            return;
        }

        // вчерашний бар
        let b1 = chart.bar(1).unwrap();

        // тело вчерашнего бара маленькое - меньше 0.4%
        if b1.body().abs_p() > 0.4 {
            return;
        }

        // верхняя тень маленькая, меньше 0.4%
        if b1.upper().abs_p() > 0.4 {
            return;
        }

        // нижняя тень большая - больше 0.8%
        if b1.lower().abs_p() < 0.8 {
            return;
        }

        // все условия выполнены, покупаем по рынку
        self.buy();
    }
    fn buy(&mut self) {
        // создаем трейд
        self.create_trade();

        // отправляем ордер трейдеру
        self.send_buy_order();
    }
    fn create_trade(&mut self) {
        let trade = Trade::new(
            self.last_ts,
            self.name(),
            TradeKind::Long,
            self.iid.clone().unwrap(),
        );
        self.trade = Some(Trade::New(trade));
    }
    fn send_buy_order(&mut self) {
        // Создаем ордер
        let order = MarketOrder::new(Direction::Buy, LOTS);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        self.buy_order = Some(order.clone());

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
        self.status = Status::PostingBuy;
    }
    fn on_buy_event(&mut self, e: OrderEvent) {
        // Функция вызывается когда стратегия находится в статусе PostingBuy
        // и прилетает ордер эвент, по отправленному ордеру на покупку.

        // Достаем из эвента ордер
        let order = e.order;

        // Если ордер выставлен - ставим статус "открываемся", и выходим
        // На практике этот статус едва ли будет получен, маркет ордер
        // исполнится сразу. Но если бы открывались по лимитке что более
        // практично, то такая логика была бы уместна.
        if order.is_posted() {
            self.buy_order = Some(order);
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
            let trade = Trade::Opened(trade);
            self.trade = Some(trade);
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
            Direction::Sell,
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
            Direction::Sell,
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
