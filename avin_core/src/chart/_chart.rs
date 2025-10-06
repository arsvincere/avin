/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use avin_utils::{AvinError, bisect_left, bisect_right};

use crate::{Bar, Iid, Indicator, Manager, TimeFrame};

/// Aggregation of instrument id, timeframe and bars.
///
/// # ru
/// График - хранит идентификатора инструмента, таймфрейм и бары.
pub struct Chart {
    iid: Iid,
    tf: TimeFrame,
    bars: Vec<Bar>,
    ind: HashMap<String, Indicator>,
}
impl Chart {
    /// Create new chart.
    ///
    /// # ru
    /// Конструктор.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::{TimeFrame, Bar, Chart, Manager};
    ///
    /// let iid = Manager::find_iid("moex_share_sber").unwrap();
    /// let tf = TimeFrame::Day;
    ///
    /// let b1 = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let b2 = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let bars = vec![b1, b2];
    ///
    /// let chart = Chart::new(&iid, tf, bars);
    ///
    /// assert_eq!(chart.ticker(), "SBER");
    /// ```
    pub fn new(iid: &Iid, tf: TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            iid: iid.clone(),
            tf,
            bars,
            ind: HashMap::new(),
        }
    }
    /// Create new chart without bars.
    ///
    /// # ru
    /// Создает пустой график, без баров. Используется в тестере.
    pub fn empty(iid: &Iid, tf: TimeFrame) -> Self {
        Self::new(iid, tf, Vec::new())
    }
    /// Loading chart with bars from half-open interval [begin, end)
    /// market data must be available in [`CFG.dir.data()`].
    ///
    /// # ru
    /// Загружает график с барами в полуоткрытом в интервале [begin, end).
    /// Данные должны быть доступны в папке указанной в конфиге пользователя.
    /// Рыночные данные можно загрузить воспользовавшись модулем avin_data
    /// написанном на python, доступен в том же репозитарии.
    ///
    /// Паникует если данных не найдено.
    pub fn load(
        iid: &Iid,
        tf: TimeFrame,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Self, AvinError> {
        match Manager::load(iid, tf.market_data(), begin, end) {
            Ok(df) => {
                let bars = Bar::from_df(&df).unwrap();
                let chart = Self::new(iid, tf, bars);

                Ok(chart)
            }
            Err(e) => {
                panic!("{e}");
            }
        }
    }

    /// Return chart instrument id.
    ///
    /// # ru
    /// Возвращает ссылку на идентификатор инструмента.
    pub fn iid(&self) -> &Iid {
        &self.iid
    }
    /// Return ticker.
    ///
    /// # ru
    /// Возвращает ссылку на строку с тикером.
    pub fn ticker(&self) -> &String {
        self.iid.ticker()
    }
    /// Return chart timeframe.
    ///
    /// # ru
    /// Возвращает таймфрейм.
    pub fn tf(&self) -> TimeFrame {
        self.tf
    }
    /// Return bars of chart.
    ///
    /// # ru
    /// Возвращает cсылку на вектор исторических баров в графике.
    pub fn bars(&self) -> &Vec<Bar> {
        &self.bars
    }
    /// Get bar by number.
    ///
    /// # ru
    /// Возвращает ссылку на бар по номеру или None, если такой отсутствует.
    ///
    /// Поведение как Pine от TradingView.
    /// Бар с индексом 0 == текущий реалтайм бар, тоже что chart.now().
    /// Бар с индексом 1 == последний исторический бар.
    /// Бар с индексом 2 == предпоследний бар в графике.
    /// И так далее...
    pub fn bar(&self, n: usize) -> Option<&Bar> {
        if n == 0 {
            return self.bars.last();
        };

        let index = self.bars.len() - n - 1;
        self.bars.get(index)
    }
    /// Return fist historical bar of chart.
    ///
    /// # ru
    /// Возвращает ссылку на первый исторический бар или None,
    /// если график не содержит баров.
    pub fn first(&self) -> Option<&Bar> {
        self.bars.first()
    }
    /// Return last historical bar of chart
    ///
    /// # ru
    /// Возвращает ссылку на последний исторический бар или None,
    /// если график не содержит баров.
    pub fn last(&self) -> Option<&Bar> {
        self.bars.get(self.bars.len() - 2)
    }
    /// Return real-time bar of chart
    ///
    /// # ru
    /// Возвращает ссылку на текущий real-time бар или None,
    /// если график не содержит баров.
    pub fn now(&self) -> Option<&Bar> {
        self.bars.last()
    }
    /// Return last price
    ///
    /// # ru
    /// Возвращает цену последней сделки реал-тайм бара, если он есть,
    /// или последнего исторического бара. Если график не содержит баров,
    /// возвращает None.
    pub fn last_price(&self) -> Option<f64> {
        let last_bar = self.bars.last()?;
        Some(last_bar.c)
    }
    /// Select bars in closed range [from, till].
    ///
    /// # ru
    /// Возвращает срез баров в закрытом интервале заданном
    /// начальным и конечным timestamp [from, till].
    pub fn select(&self, from: i64, till: i64) -> &[Bar] {
        assert!(from <= till);

        let f = bisect_right(&self.bars, from, |b| b.ts).unwrap();
        let t = bisect_left(&self.bars, till, |b| b.ts).unwrap();

        &self.bars[f..=t]
    }
    /// Add new bar
    /// Depending on datetime of 'new_bar' this function do:
    ///  - only update real-time bar
    ///  - add new historical bar and update real-time
    ///
    /// # ru
    /// Добавляет в график новый бар. В зависимости от даты и
    /// времени добавляемого бара функция:
    /// - обновит текущий реал-тайм бар новым баром;
    /// - сделает текущий реал-тайм бар историческим (last), а новый
    ///   поставит текущим (now);
    pub fn add_bar(&mut self, new_bar: Bar) {
        self.adding_bar(new_bar);
        self.update_ind();
    }
    /// Get bar with this timestamp.
    ///
    /// # ru
    /// Возвращает ссылку на бар с заданным timestamp или None,
    /// если такой отсутствует.
    ///
    /// Используется в GUI ChartWidget, и поэтому имеет спецефическое
    /// поведение. Если время до самого первого бара в графике, или
    /// в графике нет баров - возвращает None. Однако если график
    /// не пустой и время после самого последнего бара - то возвращает
    /// самый последний бар.
    ///
    /// Такое поведение имеет Тинькофф терминал, если мышь находится
    /// справа от графика, где нет баров, то отображается информация
    /// по последнему бару в графике.
    pub fn get_bar_of_ts(&self, ts: i64) -> Option<&Bar> {
        // если вообще баров нет -> None
        if self.bars.is_empty() {
            return None;
        }

        // если первый бар в графике есть
        // и если время меньше чем время первого бара -> None
        let bar = self.bars.first()?;
        if ts < bar.ts {
            return None;
        }

        // если текущий бар есть
        // и если время больше чем время текущего бара -> текущий бар
        let bar = self.bars.last()?;
        if ts > bar.ts {
            return Some(bar);
        }

        // Иначе время где-то в пределах имеющихся баров, делаем поиск
        let index = bisect_left(&self.bars, ts, |b| b.ts).unwrap();
        self.bars.get(index)
    }

    /// Add indicator.
    ///
    /// # ru
    /// Добавляет индикатор на график. Индикатор автоматически обновляется
    /// на каждом новом баре.
    pub fn add_ind(&mut self, i: Indicator) {
        let id = i.id().to_string();
        self.ind.insert(id, i);
    }
    /// Get indicator.
    ///
    /// # ru
    /// Возвращает индикатор по ID, или None если такого индикатора на
    /// график не добавлено.
    pub fn get_ind(&self, id: &str) -> Option<&Indicator> {
        self.ind.get(id)
    }
    /// Get mutable indicator.
    ///
    /// # ru
    /// Возвращает изменяемый индикатор по ID, или None если такого
    /// индикатора на график не добавлено.
    pub fn get_ind_mut(&mut self, id: &str) -> Option<&mut Indicator> {
        self.ind.get_mut(id)
    }

    // private
    fn adding_bar(&mut self, new_bar: Bar) {
        let last_bar = self.bars.last_mut();

        // если баров не было - в пустой график добавляем первый бар
        if last_bar.is_none() {
            self.bars.push(new_bar);
            return;
        }

        // далее ситуации когда в графике есть бары
        let last_bar = last_bar.unwrap();

        // если время одинаковое - только обновить текущий бар
        if last_bar.ts == new_bar.ts {
            *last_bar = new_bar;
            return;
        }

        // время смены бара
        let next_ts = self.tf.next_ts(last_bar.ts);

        // если время пришедшего нового бара больше текущего последнего
        // и при этом меньше чем время смены бара, - джоинить этот бар
        if new_bar.ts > last_bar.ts && new_bar.ts < next_ts {
            *last_bar = Bar::join(*last_bar, new_bar);
            return;
        }

        // если время пришедшего нового бара больше текущего последнего
        // и при этом больше или равно времени смены бара, - новый текущий
        // ВАЖНО: именно 'больше или равно', потому что например на
        // минутках после смены дня идет разрыв во времени... и не будет
        // бара 23:50 будет сразу бар 06:59.
        if new_bar.ts > last_bar.ts && new_bar.ts >= next_ts {
            self.bars.push(new_bar);
        }
    }
    #[inline]
    fn update_ind(&mut self) {
        for (_id, ind) in self.ind.iter_mut() {
            ind.update(&self.bars);
        }
    }
}
impl AsRef<Chart> for Chart {
    fn as_ref(&self) -> &Chart {
        self
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use avin_utils as utils;

    use super::*;
    use crate::*;

    #[test]
    fn new() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let df = Manager::load(&iid, tf.market_data(), begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();

        let chart = Chart::new(&iid, tf, bars);
        assert_eq!(chart.iid(), &iid);
        assert_eq!(chart.tf(), tf);
        assert_eq!(chart.bars().len(), 256);
    }
    #[test]
    fn empty() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;

        let chart = Chart::empty(&iid, tf);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
    }
    #[test]
    fn load() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2023-08-01");
        let end = utils::str_date_to_utc("2023-09-01");

        let chart = Chart::load(&iid, tf, begin, end).unwrap();
        assert_eq!(chart.tf(), tf);
        assert_eq!(chart.bars().len(), 23);
        assert!(chart.now().is_some());

        assert_eq!(chart.first().unwrap().dt(), begin);
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2023, 8, 29, 21, 0, 0).unwrap(),
        );
        assert_eq!(
            chart.now().unwrap().dt(),
            Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap(),
        );
    }
    #[test]
    fn select_on_d() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2024-12-20");
        let end = utils::str_date_to_utc("2025-01-01");
        let chart = share.load_chart_period(tf, begin, end).unwrap();

        let from = utils::str_date_to_utc("2024-12-23")
            .timestamp_nanos_opt()
            .unwrap();
        let till = utils::str_date_to_utc("2024-12-25")
            .timestamp_nanos_opt()
            .unwrap();

        let selected = chart.select(from, till);
        assert_eq!(selected.len(), 3);
    }
    #[test]
    fn select_on_h() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::H1;

        let begin = utils::str_date_to_utc("2023-08-01");
        let end = utils::str_date_to_utc("2023-08-02");
        let chart = share.load_chart_period(tf, begin, end).unwrap();

        // выборка с 12:30 до 15:30
        // должно войти 3 бара 13:00 14:00 15:00
        let from = utils::str_dt_to_utc("2023-08-01 12:30:00")
            .timestamp_nanos_opt()
            .unwrap();
        let till = utils::str_dt_to_utc("2023-08-01 15:30:00")
            .timestamp_nanos_opt()
            .unwrap();

        let selected = chart.select(from, till);
        assert_eq!(selected.len(), 3);
    }
    #[test]
    fn add_bar_10m() {
        // 1M
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280 l=280 c=280 v=158150
        // Bar: dt=2025-01-03 10:00:00 o=279.99 h=280 l=279.55 c=279.7 v=476620
        // Bar: dt=2025-01-03 10:01:00 o=279.64 h=279.9 l=279.2 c=279.85 v=643880
        // Bar: dt=2025-01-03 10:02:00 o=279.85 h=280.41 l=279.74 c=280.1 v=584470
        // Bar: dt=2025-01-03 10:03:00 o=280.17 h=280.2 l=279.87 c=279.9 v=369760
        // Bar: dt=2025-01-03 10:04:00 o=279.9 h=279.95 l=279.54 c=279.54 v=338140
        // Bar: dt=2025-01-03 10:05:00 o=279.54 h=279.56 l=279 c=279.44 v=767470
        // Bar: dt=2025-01-03 10:06:00 o=279.43 h=279.44 l=278.58 c=278.91 v=520310
        // Bar: dt=2025-01-03 10:07:00 o=278.99 h=279.38 l=278.71 c=279.07 v=281490
        // Bar: dt=2025-01-03 10:08:00 o=279.07 h=279.07 l=278.11 c=278.31 v=304020
        // Bar: dt=2025-01-03 10:09:00 o=278.34 h=278.91 l=278.15 c=278.4 v=416040
        // Bar: dt=2025-01-03 10:10:00 o=278.53 h=278.89 l=278.14 c=278.62 v=233030
        // Bar: dt=2025-01-03 10:11:00 o=278.62 h=278.93 l=278.58 c=278.93 v=164140
        // Bar: dt=2025-01-03 10:12:00 o=278.91 h=279.21 l=278.85 c=278.86 v=208890
        // Bar: dt=2025-01-03 10:13:00 o=278.86 h=278.88 l=278.51 c=278.73 v=153850
        // ---> 10M
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280 l=280 c=280 v=158150
        // Bar: dt=2025-01-03 10:00:00 o=279.99 h=280.41 l=278.11 c=278.4 v=4702200
        // Bar: dt=2025-01-03 10:10:00 o=278.53 h=279.21 l=278.14 c=278.73 v=759910

        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::M10;
        let mut chart = Chart::empty(&iid, tf);

        let bars_data = bars();
        for b in bars_data.iter() {
            let bar = Bar::new(b.0, b.1, b.2, b.3, b.4, b.5 as u64);
            chart.add_bar(bar);
        }

        let bars = chart.bars();
        assert_eq!(bars.len(), 3);
        assert_eq!(
            bars[0],
            Bar::new(
                1735887540000000000_i64,
                280.0,
                280.0,
                280.0,
                280.0,
                158150
            )
        );
        assert_eq!(
            bars[1],
            Bar::new(
                1735887600000000000_i64,
                279.99,
                280.41,
                278.11,
                278.4,
                4702200
            )
        );
        // Bar: dt=2025-01-03 10:10:00 o=278.53 h=279.21 l=278.14 c=278.73 v=759910
        assert_eq!(
            bars[2],
            Bar::new(
                1735888200000000000_i64,
                278.53,
                279.21,
                278.14,
                278.73,
                759910
            )
        );
    }
    #[test]
    fn add_bar_1h() {
        // 1M
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280 l=280 c=280 v=158150
        // Bar: dt=2025-01-03 10:00:00 o=279.99 h=280 l=279.55 c=279.7 v=476620
        // Bar: dt=2025-01-03 10:01:00 o=279.64 h=279.9 l=279.2 c=279.85 v=643880
        // Bar: dt=2025-01-03 10:02:00 o=279.85 h=280.41 l=279.74 c=280.1 v=584470
        // Bar: dt=2025-01-03 10:03:00 o=280.17 h=280.2 l=279.87 c=279.9 v=369760
        // Bar: dt=2025-01-03 10:04:00 o=279.9 h=279.95 l=279.54 c=279.54 v=338140
        // Bar: dt=2025-01-03 10:05:00 o=279.54 h=279.56 l=279 c=279.44 v=767470
        // Bar: dt=2025-01-03 10:06:00 o=279.43 h=279.44 l=278.58 c=278.91 v=520310
        // Bar: dt=2025-01-03 10:07:00 o=278.99 h=279.38 l=278.71 c=279.07 v=281490
        // Bar: dt=2025-01-03 10:08:00 o=279.07 h=279.07 l=278.11 c=278.31 v=304020
        // Bar: dt=2025-01-03 10:09:00 o=278.34 h=278.91 l=278.15 c=278.4 v=416040
        // Bar: dt=2025-01-03 10:10:00 o=278.53 h=278.89 l=278.14 c=278.62 v=233030
        // Bar: dt=2025-01-03 10:11:00 o=278.62 h=278.93 l=278.58 c=278.93 v=164140
        // Bar: dt=2025-01-03 10:12:00 o=278.91 h=279.21 l=278.85 c=278.86 v=208890
        // Bar: dt=2025-01-03 10:13:00 o=278.86 h=278.88 l=278.51 c=278.73 v=153850
        // ---> 1H
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280 l=280 c=280 v=158150
        // Bar: dt=2025-01-03 10:00:00 o=279.99 h=280.41 l=278.11 c=278.73 v=5462110

        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::H1;
        let mut chart = Chart::empty(&iid, tf);

        let bars_data = bars();
        for b in bars_data.iter() {
            let bar = Bar::new(b.0, b.1, b.2, b.3, b.4, b.5 as u64);
            chart.add_bar(bar);
        }

        let bars = chart.bars();
        assert_eq!(bars.len(), 2);
        assert_eq!(
            bars[0],
            Bar::new(
                1735887540000000000_i64,
                280.0,
                280.0,
                280.0,
                280.0,
                158150
            )
        );
        assert_eq!(
            bars[1],
            Bar::new(
                1735887600000000000_i64,
                279.99,
                280.41,
                278.11,
                278.73,
                5462110
            )
        );
    }
    #[test]
    fn add_bar_day() {
        // 1M
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280 l=280 c=280 v=158150
        // Bar: dt=2025-01-03 10:00:00 o=279.99 h=280 l=279.55 c=279.7 v=476620
        // Bar: dt=2025-01-03 10:01:00 o=279.64 h=279.9 l=279.2 c=279.85 v=643880
        // Bar: dt=2025-01-03 10:02:00 o=279.85 h=280.41 l=279.74 c=280.1 v=584470
        // Bar: dt=2025-01-03 10:03:00 o=280.17 h=280.2 l=279.87 c=279.9 v=369760
        // Bar: dt=2025-01-03 10:04:00 o=279.9 h=279.95 l=279.54 c=279.54 v=338140
        // Bar: dt=2025-01-03 10:05:00 o=279.54 h=279.56 l=279 c=279.44 v=767470
        // Bar: dt=2025-01-03 10:06:00 o=279.43 h=279.44 l=278.58 c=278.91 v=520310
        // Bar: dt=2025-01-03 10:07:00 o=278.99 h=279.38 l=278.71 c=279.07 v=281490
        // Bar: dt=2025-01-03 10:08:00 o=279.07 h=279.07 l=278.11 c=278.31 v=304020
        // Bar: dt=2025-01-03 10:09:00 o=278.34 h=278.91 l=278.15 c=278.4 v=416040
        // Bar: dt=2025-01-03 10:10:00 o=278.53 h=278.89 l=278.14 c=278.62 v=233030
        // Bar: dt=2025-01-03 10:11:00 o=278.62 h=278.93 l=278.58 c=278.93 v=164140
        // Bar: dt=2025-01-03 10:12:00 o=278.91 h=279.21 l=278.85 c=278.86 v=208890
        // Bar: dt=2025-01-03 10:13:00 o=278.86 h=278.88 l=278.51 c=278.73 v=153850
        // ---> Day
        // Bar: dt=2025-01-03 09:59:00 o=280 h=280.41 l=278.11 c=278.73 v=5620260

        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let mut chart = Chart::empty(&iid, tf);

        let bars_data = bars();
        for b in bars_data.iter() {
            let bar = Bar::new(b.0, b.1, b.2, b.3, b.4, b.5 as u64);
            chart.add_bar(bar);
        }

        let bars = chart.bars();
        assert_eq!(bars.len(), 1);
        assert_eq!(
            bars[0],
            Bar::new(
                1735887540000000000_i64,
                280.0,
                280.41,
                278.11,
                278.73,
                5620260
            )
        );
    }

    // data for testing chart.add_bar(...)
    fn bars() -> std::vec::Vec<(i64, f64, f64, f64, f64, i32)> {
        vec![
            (1735887540000000000_i64, 280.0, 280.0, 280.0, 280.0, 158150),
            (
                1735887600000000000_i64,
                279.99,
                280.0,
                279.55,
                279.7,
                476620,
            ),
            (
                1735887660000000000_i64,
                279.64,
                279.9,
                279.2,
                279.85,
                643880,
            ),
            (
                1735887720000000000_i64,
                279.85,
                280.41,
                279.74,
                280.1,
                584470,
            ),
            (
                1735887780000000000_i64,
                280.17,
                280.2,
                279.87,
                279.9,
                369760,
            ),
            (
                1735887840000000000_i64,
                279.9,
                279.95,
                279.54,
                279.54,
                338140,
            ),
            (
                1735887900000000000_i64,
                279.54,
                279.56,
                279.0,
                279.44,
                767470,
            ),
            (
                1735887960000000000_i64,
                279.43,
                279.44,
                278.58,
                278.91,
                520310,
            ),
            (
                1735888020000000000_i64,
                278.99,
                279.38,
                278.71,
                279.07,
                281490,
            ),
            (
                1735888080000000000_i64,
                279.07,
                279.07,
                278.11,
                278.31,
                304020,
            ),
            (
                1735888140000000000_i64,
                278.34,
                278.91,
                278.15,
                278.4,
                416040,
            ),
            (
                1735888200000000000_i64,
                278.53,
                278.89,
                278.14,
                278.62,
                233030,
            ),
            (
                1735888260000000000_i64,
                278.62,
                278.93,
                278.58,
                278.93,
                164140,
            ),
            (
                1735888320000000000_i64,
                278.91,
                279.21,
                278.85,
                278.86,
                208890,
            ),
            (
                1735888380000000000_i64,
                278.86,
                278.88,
                278.51,
                278.73,
                153850,
            ),
        ]
    }
}
