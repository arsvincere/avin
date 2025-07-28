/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};

use crate::{
    BarEvent, Chart, Footprint, Iid, Manager, Share, Tic, TicEvent, TimeFrame,
};
use avin_utils::AvinError;

/// Aggregation of instrument id, charts, tics.
///
/// # ru
/// Актив - обертка над конкретными типами активов: акция, фьючерс, облигация,
/// индекс и тп. Пока реализована только акция.
///
/// Содержит идентификатор инструмента, графики разных таймфреймов и тиковые
/// данные, а так же кластеры (footprint chart). Это владеющий тип.
///
/// Предоставляет общий для всех активов интерфейс.
///
/// ## Examples
/// ```
/// use avin_core::{Asset, TimeFrame};
///
/// let mut sber = Asset::new("moex_share_sber").unwrap();
/// assert_eq!(sber.name(), "Сбер Банк");
///
/// let tf = TimeFrame::Day;
/// assert!(sber.chart(tf).is_none());
///
/// sber.load_chart(tf).unwrap();
/// assert!(sber.chart(tf).is_some());
///
/// sber.load_tics().unwrap();
/// assert!(sber.tics().is_some());
///
/// sber.build_footprint(tf).unwrap();
/// assert!(sber.footprint(tf).is_some());
/// ```
pub enum Asset {
    SHARE(Share),
}
impl Asset {
    /// Create new asset from str (case insensitive),
    /// format: "exchange_category_ticker".
    ///
    /// # ru
    /// Создает актив из строки (не чувствительно к регистру).
    /// Формат строки: "exchange_category_ticker".
    pub fn new(s: &str) -> Result<Asset, AvinError> {
        let iid = Manager::find_iid(s)?;
        let share = Share::from_iid(iid);
        let asset = Asset::SHARE(share);

        Ok(asset)
    }
    /// Create new asset from instrument id.
    ///
    /// # ru
    /// Создает актив из идентификатора инструмента.
    pub fn from_iid(iid: Iid) -> Self {
        assert!(iid.category() == "SHARE");
        let share = Share::from_iid(iid);

        Asset::SHARE(share)
    }
    /// Create new asset from csv.
    ///
    /// # ru
    /// Создает актив из csv формата.
    #[allow(clippy::get_first)]
    pub fn from_csv(line: &str) -> Result<Self, String> {
        // line example: 'MOEX;SHARE;SBER;'
        let parts: Vec<&str> = line.split(';').collect();
        let exchange = parts.get(0).expect("invalid line");
        let category = parts.get(1).expect("invalid line");
        let ticker = parts.get(2).expect("invalid line");

        let query = format!("{exchange}_{category}_{ticker}");
        let result = Manager::find_iid(&query);

        match result {
            Ok(iid) => {
                let asset = Asset::from_iid(iid);
                Ok(asset)
            }
            Err(why) => {
                let msg = format!("fail create from csv {line}, {why}");
                Err(msg)
            }
        }
    }
    /// Return vector with all shares whose have market data in user dir.
    ///
    /// # ru
    /// Возвращает вектор с акциями, для которых есть рыночные данные в
    /// папке пользователя.
    pub fn all_shares() -> Vec<Share> {
        Share::all()
    }

    /// Return instrument id.
    ///
    /// # ru
    /// Возвращает ссылку на идентификатор инструмента.
    pub fn iid(&self) -> &Iid {
        match self {
            Self::SHARE(share) => share.iid(),
        }
    }
    /// Return exchange.
    ///
    /// # ru
    /// Возвращает название биржи на которой торгуется инструмент.
    pub fn exchange(&self) -> &String {
        match self {
            Self::SHARE(share) => share.exchange(),
        }
    }
    /// Return category.
    ///
    /// # ru
    /// Возвращает название категории инструмента: акция, облигация,
    /// индекс, фьючерс и тп.
    pub fn category(&self) -> &String {
        match self {
            Self::SHARE(share) => share.category(),
        }
    }
    /// Return ticker.
    ///
    /// # ru
    /// Возвращает тикер инструмента.
    pub fn ticker(&self) -> &String {
        match self {
            Self::SHARE(share) => share.ticker(),
        }
    }
    /// Return FIGI - Financial Instrument Global Identifier.
    ///
    /// # ru
    /// Возвращает FIGI - глобальный финансовый идентификатор
    /// инструмента. Используется брокером при выставлении ордера,
    /// так как тикер не является уникальным идентификатором, однозначно
    /// определяющим актив.
    pub fn figi(&self) -> &String {
        match self {
            Self::SHARE(share) => share.figi(),
        }
    }
    /// Return instrument name.
    ///
    /// # ru
    /// Возвращает название инструмента.
    pub fn name(&self) -> &String {
        match self {
            Self::SHARE(share) => share.name(),
        }
    }
    /// Return reference to HashMap with instrument info.
    ///
    /// # ru
    /// Возвращает ссылку на HashMap со всей имеющейся информацией
    /// об инструменте.
    pub fn info(&self) -> &HashMap<String, String> {
        match self {
            Self::SHARE(share) => share.info(),
        }
    }
    /// Return the dir path with market data of instrument.
    ///
    /// # ru
    /// Возвращает путь к каталогу с рыночными данными инструмента.
    pub fn path(&self) -> PathBuf {
        match self {
            Self::SHARE(share) => share.path(),
        }
    }

    /// Return chart.
    ///
    /// # ru
    /// Возвращает ссылку на график, или None если график заданного
    /// таймфрейма не загружен.
    pub fn chart(&self, tf: TimeFrame) -> Option<&Chart> {
        match self {
            Self::SHARE(share) => share.chart(tf),
        }
    }
    /// Return mutable chart.
    ///
    /// # ru
    /// Возвращает мутабельную ссылку на график (например, для добавление
    /// индикаторов на график), или None, если график заданного таймфрейма
    /// не загружен.
    pub fn chart_mut(&mut self, tf: TimeFrame) -> Option<&mut Chart> {
        match self {
            Self::SHARE(share) => share.chart_mut(tf),
        }
    }
    /// Load chart with default bars count. Return reference of loaded chart.
    ///
    /// # ru
    /// Загружает график с количеством баров по умолчанию (задается в
    /// конфиге пользователя). Возвращает ссылку на загруженный график.
    /// График сохраняется внутри актива.
    pub fn load_chart(&mut self, tf: TimeFrame) -> Result<&Chart, AvinError> {
        match self {
            Self::SHARE(share) => share.load_chart(tf),
        }
    }
    /// Load chart with default bars count. Return mutable reference of
    /// loaded chart.
    ///
    /// # ru
    /// Загружает график с количеством баров по умолчанию (задается в
    /// конфиге пользователя). График сохраняется внутри актива. Возвращает
    /// мутабельную ссылку на загруженный график.
    pub fn load_chart_mut(
        &mut self,
        tf: TimeFrame,
    ) -> Result<&mut Chart, AvinError> {
        match self {
            Self::SHARE(share) => share.load_chart_mut(tf),
        }
    }
    /// Load chart with bars of half-open interval [begin, end).
    ///
    /// # ru
    /// Загружает график с барами в полуоткрытом интервале [begin, end).
    /// График сохраняется внутри актива. Возвращает ссылку на загруженный
    /// график.
    pub fn load_chart_period(
        &mut self,
        tf: TimeFrame,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<&Chart, AvinError> {
        match self {
            Self::SHARE(share) => share.load_chart_period(tf, begin, end),
        }
    }
    /// Create empty chart with given timeframe, and store in self.
    ///
    /// # ru
    /// Создает пустой график для актива. Используется бэктестером.
    pub fn load_chart_empty(&mut self, tf: TimeFrame) -> &Chart {
        match self {
            Self::SHARE(share) => share.load_chart_empty(tf),
        }
    }

    /// Return vector of tics, if loaded, else None.
    ///
    /// # ru
    /// Возвращает вектор тиков, если они загружены, иначе None.
    pub fn tics(&self) -> Option<&Vec<Tic>> {
        match self {
            Self::SHARE(share) => share.tics(),
        }
    }
    /// Return footprint chart
    ///
    /// # ru
    /// Возвращает ссылку на кластерный график заданного таймфрейма.
    ///
    /// Сначала нужно загрузить тиковые данные [`Asset::load_tics`], затем
    /// рассчитать кластеры для таймфрейма [`Asset::build_footprint`]. Если
    /// это не сделано, вернет None.
    pub fn footprint(&self, tf: TimeFrame) -> Option<&Footprint> {
        match self {
            Self::SHARE(share) => share.footprint(tf),
        }
    }
    /// Return footprint chart
    ///
    /// # ru
    /// Возвращает мутабельную ссылку на кластерный график заданного
    /// таймфрейма.
    ///
    /// Сначала нужно загрузить тиковые данные [`Asset::load_tics`], затем
    /// рассчитать кластеры для таймфрейма [`Asset::build_footprint`]. Если
    /// это не сделано, вернет None.
    pub fn footprint_mut(
        &mut self,
        tf: &TimeFrame,
    ) -> Option<&mut Footprint> {
        match self {
            Asset::SHARE(share) => share.footprint_mut(tf),
        }
    }
    /// Load tics data.
    ///
    /// # ru
    /// Загружает тиковые данные по активу.
    pub fn load_tics(&mut self) -> Result<(), AvinError> {
        match self {
            Self::SHARE(share) => share.load_tics(),
        }
    }
    /// Calculate footprint chart
    ///
    /// # ru
    /// Рассчитывает кластерный график заданного таймфрейма из загруженных
    /// тиков. Сохраняет результат.
    pub fn build_footprint(
        &mut self,
        tf: TimeFrame,
    ) -> Result<(), AvinError> {
        match self {
            Self::SHARE(share) => share.build_footprint(tf),
        }
    }

    /// Change per month
    ///
    /// # ru
    /// Изменение за месяц (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_month(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::Month)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }
    /// Change per week
    ///
    /// # ru
    /// Изменение за неделю (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_week(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::Week)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }
    /// Change per day
    ///
    /// # ru
    /// Изменение за день (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_day(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::Day)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }
    /// Change per hour
    ///
    /// # ru
    /// Изменение за час (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_1h(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::H1)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }
    /// Change per 10 minutes
    ///
    /// # ru
    /// Изменение за 10 минут (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_10m(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::M10)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }
    /// Change per 1 minute
    ///
    /// # ru
    /// Изменение за 1 минуту (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_1m(&self) -> Option<f64> {
        let chart = self.chart(TimeFrame::M1)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }

    /// Receive bar event
    ///
    /// # ru
    /// Принимает [`BarEvent`], находит график соответствующего таймфрейма
    /// и добавляет туда новый бар, содержащийся в эвенте. Используется
    /// тестером и трейдером при получении нового бара из стрима данных.
    /// Не предназначена для прямого использования пользователем.
    pub fn bar_event(&mut self, e: BarEvent) {
        match self {
            Self::SHARE(share) => share.bar_event(e),
        }
    }
    /// Receive tic event
    ///
    /// # ru
    /// Принимает [`TicEvent`], сохраняет новый тик в активе. Используется
    /// тестером и трейдером при получении нового бара из стрима данных.
    /// Не предназначена для прямого использования пользователем.
    pub fn tic_event(&mut self, _e: TicEvent) {
        todo!();
    }
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SHARE(s) => write!(
                f,
                "Asset={}_{}_{}",
                s.exchange(),
                s.category(),
                s.ticker()
            ),
        }
    }
}
impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.figi() == other.figi()
    }
}
