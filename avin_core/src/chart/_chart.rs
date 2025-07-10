/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use avin_utils::{AvinError, bisect_left, bisect_right};

use crate::{Bar, Iid, Indicator, Manager, TimeFrame, UserData};

/// Aggregation of instrument id, timeframe and bars.
///
/// # ru
/// График - хранит идентификатора инструмента, таймфрейм и бары.
pub struct Chart {
    iid: Iid,
    tf: TimeFrame,
    bars: Vec<Bar>,
    now: Option<Bar>,
    ind: HashMap<String, Indicator>,
    user_data: HashMap<String, Box<dyn UserData>>,
}
impl Chart {
    /// Create new chart.
    ///
    /// # ru
    /// Конструктор.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::{Manager, TimeFrame, Bar, Chart};
    ///
    /// let iid = Manager::find_iid("moex_share_sber").unwrap();
    /// let tf = TimeFrame::Day;
    ///
    /// let b1 = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10, None);
    /// let b2 = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10, None);
    /// let bars = vec![b1, b2];
    ///
    /// let chart = Chart::new(&iid, &tf, bars);
    ///
    /// assert_eq!(chart.ticker(), "SBER");
    /// ```
    pub fn new(iid: &Iid, tf: &TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            iid: iid.clone(),
            tf: *tf,
            bars,
            now: None,
            ind: HashMap::new(),
            user_data: HashMap::new(),
        }
    }
    /// Create new chart without bars.
    ///
    /// # ru
    /// Создает пустой график, без баров. Используется в тестере.
    pub fn empty(iid: &Iid, tf: &TimeFrame) -> Self {
        Self::new(iid, tf, Vec::new())
    }
    /// Loading chart with bars from half-open interval [begin, end)
    /// market data must be available in [`CFG.dir.data()`].
    ///
    /// # ru
    /// Загружает график с барами в полуоткрытом в интервале [begin, end).
    /// Данные должны быть доступны в папке указанной в конфиге пользователя.
    /// Рыночные данные можно загрузить воспользовавшись модулем avin_data
    /// написанном на python, доступен в том же репозитарии, и на PyPl:
    /// "pip install avin_data". Так же есть консольная утилита avin-data
    /// см. репозитарий [проекта](https://github.com/arsvincere/avin).
    pub fn load(
        iid: &Iid,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Self, AvinError> {
        match Manager::load(iid, &tf.market_data(), begin, end) {
            Ok(df) => {
                let bars = Bar::from_df(&df).unwrap();
                let chart = Self::new(iid, tf, bars);

                Ok(chart)
            }
            Err(e) => {
                log::warn!("{}, using empty chart", e);
                Ok(Self::empty(iid, tf))
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
    /// Возвращает ссылку на таймфрейм.
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
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
    /// Бар с индексом 0 == текущий реалтайм бар, тоже что chart.now()
    /// Бар с индексом 1 == последний исторический бар
    /// Бар с индексом 2 == предпоследний бар в графике
    /// ...
    pub fn bar(&self, n: usize) -> Option<&Bar> {
        if n == 0 {
            return self.now.as_ref();
        };

        let bars_count = self.bars.len();
        if bars_count < n {
            None
        } else {
            let index = self.bars.len() - n;
            self.bars.get(index)
        }
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
        self.bars.last()
    }
    /// Return real-time bar of chart
    ///
    /// # ru
    /// Возвращает ссылку на текущий real-time бар или None,
    /// если график не содержит баров.
    pub fn now(&self) -> Option<&Bar> {
        self.now.as_ref()
    }
    /// Return last price
    ///
    /// # ru
    /// Возвращает цену последней сделки реал-тайм бара, если он есть,
    /// или последнего исторического бара. Если график не содержит баров,
    /// возвращает None.
    pub fn last_price(&self) -> Option<f64> {
        if let Some(bar) = self.now() {
            Some(bar.c)
        } else {
            self.last().map(|bar| bar.c)
        }
    }
    /// Select bars in closed range [from, till].
    ///
    /// # ru
    /// Возвращает срез баров в закрытом интервале заданном
    /// начальным и конечным timestamp [from, till].
    pub fn select(&self, from: i64, till: i64) -> &[Bar] {
        assert!(from <= till);

        let f = bisect_right(&self.bars, from, |b| b.ts_nanos).unwrap();
        let t = bisect_left(&self.bars, till, |b| b.ts_nanos).unwrap();

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
    /// - сделает текущий реал-тайм бар историческим, а новый
    ///   поставит текущим;
    pub fn add_bar(&mut self, new_bar: Bar) {
        match self.now.take() {
            None => {
                // receive first real time bar
                self.now = Some(new_bar);
            }
            Some(old_bar) => {
                // only update now bar
                if old_bar.ts_nanos == new_bar.ts_nanos {
                    self.now = Some(new_bar);
                // new historical bar and update now bar
                } else if old_bar.ts_nanos < new_bar.ts_nanos {
                    self.bars.push(old_bar);
                    self.now = Some(new_bar);
                }
                // old_bar.ts_nanos > new_bar.ts_nanos
                // Тинькофф бывает прокидывает в дата стриме
                // исторические бары законченные уже после новых
                // реал-тайм баров. По факту же - последний
                // реал-тайм бар который был в потоке как незаконченный
                // он равен этому законченному историческому бару
                // так что в моем алгоритме приема баров он не нужен, игнор.
                else {
                    return;
                }
            }
        };

        self.update_ind();
        self.update_user_data();
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
        // если первый бар в графике есть
        if let Some(bar) = self.first() {
            // если время меньше чем время первого бара -> None
            if ts < bar.ts_nanos {
                return None;
            }
        }
        // если первого бара нет (то есть вообще баров нет) -> None
        else {
            return None;
        }

        // если текущий бар есть
        if let Some(bar) = self.now {
            // если время больше чем время текущего бара -> текущий бар
            if ts > bar.ts_nanos {
                return self.now();
            }
        }
        // если текущего бара нет
        else {
            // если время больше чем время последнего бара -> последний бар
            let bar = self.last().unwrap();
            if ts > bar.ts_nanos {
                return self.last();
            }
        }

        // Иначе время где-то в пределах имеющихся баров, делаем поиск
        let index = bisect_left(&self.bars, ts, |b| b.ts_nanos).unwrap();
        self.bars.get(index)
    }

    // XXX: Unstable experimental features
    pub fn add_ind(&mut self, mut i: Indicator) {
        i.init(&self.bars, self.now.as_ref());

        let id = i.id().to_string();
        self.ind.insert(id, i);
    }
    pub fn get_ind(&self, id: &str) -> Option<&Indicator> {
        self.ind.get(id)
    }
    pub fn get_ind_mut(&mut self, id: &str) -> Option<&mut Indicator> {
        self.ind.get_mut(id)
    }

    // private
    #[inline]
    fn update_ind(&mut self) {
        for i in self.ind.iter_mut() {
            i.1.update(&self.bars, self.now.as_ref());
        }
    }
    #[inline]
    fn update_user_data(&mut self) {
        for i in self.user_data.iter_mut() {
            i.1.update(&self.bars, self.now.as_ref());
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
        let df =
            Manager::load(&iid, &tf.market_data(), &begin, &end).unwrap();
        let bars = Bar::from_df(&df).unwrap();

        let chart = Chart::new(&iid, &tf, bars);
        assert_eq!(chart.iid, iid);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 256);
        assert!(chart.now.is_none());
    }
    #[test]
    fn empty() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;

        let chart = Chart::empty(&iid, &tf);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
        assert!(chart.now.is_none());
    }
    #[test]
    fn load() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2023-08-01");
        let end = utils::str_date_to_utc("2023-09-01");

        let chart = Chart::load(&iid, &tf, &begin, &end).unwrap();
        assert_eq!(chart.tf(), &tf);
        assert_eq!(chart.bars().len(), 23);
        assert!(chart.now().is_none());

        assert_eq!(chart.first().unwrap().dt(), begin);
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap(),
        )
    }
    #[test]
    fn select_on_d() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;
        let begin = utils::str_date_to_utc("2024-12-20");
        let end = utils::str_date_to_utc("2025-01-01");
        let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

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
        let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

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
}
