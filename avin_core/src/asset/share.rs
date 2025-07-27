/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

use chrono::prelude::*;

use avin_utils::{AvinError, CFG, Cmd};

use crate::{
    BarEvent, Chart, Footprint, Iid, Manager, MarketData, Tic, TicEvent,
    TimeFrame,
};

/// Aggregation of instrument id, charts, tics, footprint charts.
///
/// # ru
/// Акция - содержит идентификатор инструмента, графики разных таймфреймов
/// и тиковые данные, а так же кластеры (footprint chart). Это владеющий тип.
///
/// Предоставляет интерфейс для доступа к данным по акции, загрузке графиков
/// и тп.
///
/// Перед обращение к графику [`Share::chart`] его нужно загрузить.
/// А перед обращением к [`Share::footprint`] нужно загрузить тики и вызвать
/// [`Share::build_footprint`] для нужного таймфрейма.
///
/// Загруженные и рассчитанные графики сохраняются. Это владеющий тип.
///
/// ## Examples
/// ```
/// use avin_core::{Share, TimeFrame};
///
/// let mut sber = Share::new("moex_share_sber").unwrap();
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
pub struct Share {
    iid: Iid,
    tics: Vec<Tic>,
    charts: HashMap<TimeFrame, Chart>,
    footprints: HashMap<TimeFrame, Footprint>,
}

impl Share {
    /// Create new share from str (case insensitive).
    ///
    /// # ru
    /// Создает акцию из строки (не чувствительно к регистру).
    /// Формат строки:
    /// ```text
    /// "<exchange>_SHARE_<ticker>".
    /// ```
    ///
    /// ## Examples
    /// ```
    /// use avin_core::{Share, TimeFrame};
    ///
    /// let sber = Share::new("moex_share_sber").unwrap();
    /// assert_eq!(sber.name(), "Сбер Банк");
    /// ```
    pub fn new(s: &str) -> Result<Share, AvinError> {
        let iid = Manager::find_iid(s)?;
        let share = Share::from_iid(iid);

        Ok(share)
    }
    /// Create new share from instrument id.
    ///
    /// # ru
    /// Создает акцию из идентификатора инструмента.
    pub fn from_iid(iid: Iid) -> Share {
        assert!(iid.category() == "SHARE");

        Self {
            iid,
            tics: Vec::new(),
            charts: HashMap::new(),
            footprints: HashMap::new(),
        }
    }
    /// Create new share from HashMap.
    ///
    /// # ru
    /// Создает акцию из HashMap с информацией об инструменте.
    /// Не предназначена для прямого использования пользователем.
    pub fn from_info(info: HashMap<String, String>) -> Share {
        let iid = Iid::new(info);

        Share::from_iid(iid)
    }
    /// Return vector with all shares whose have market data in user dir.
    ///
    /// # ru
    /// Возвращает вектор с акциями, для которых есть рыночные данные в
    /// папке пользователя.
    pub fn all() -> Vec<Share> {
        let mut shares: Vec<Share> = Vec::new();

        // shares dir path
        let mut dir_path = CFG.dir.data();
        dir_path.push("MOEX");
        dir_path.push("SHARE");

        // shares dirs: dir name == ticker
        let dirs = Cmd::get_dirs(&dir_path).unwrap();
        if dirs.is_empty() {
            log::warn!("Shares not found! Dir empty: {}", dir_path.display());
            return shares;
        }

        // create shares from dir name (ticker)
        for dir in dirs.iter() {
            let ticker = Cmd::name(dir).unwrap();
            let s = format!("MOEX_SHARE_{ticker}");
            let share = Share::new(&s).unwrap();
            shares.push(share);
        }

        shares
    }

    /// Return instrument id.
    ///
    /// # ru
    /// Возвращает ссылку на идентификатор инструмента.
    pub fn iid(&self) -> &Iid {
        &self.iid
    }
    /// Return exchange.
    ///
    /// # ru
    /// Возвращает название биржи на которой торгуется инструмент.
    pub fn exchange(&self) -> &String {
        self.iid.exchange()
    }
    /// Return category.
    ///
    /// # ru
    /// Возвращает название категории инструмента: акция (SHARE)
    pub fn category(&self) -> &String {
        self.iid.category()
    }
    /// Return ticker.
    ///
    /// # ru
    /// Возвращает тикер инструмента.
    pub fn ticker(&self) -> &String {
        self.iid.ticker()
    }
    /// Return FIGI - Financial Instrument Global Identifier.
    ///
    /// # ru
    /// Возвращает FIGI - глобальный финансовый идентификатор
    /// инструмента. Используется брокером при выставлении ордера,
    /// так как тикер не является уникальным идентификатором, однозначно
    /// определяющим актив.
    pub fn figi(&self) -> &String {
        self.iid.figi()
    }
    /// Return share name.
    ///
    /// # ru
    /// Возвращает название акции.
    pub fn name(&self) -> &String {
        self.iid.name()
    }
    /// Return reference to HashMap with share info.
    ///
    /// # ru
    /// Возвращает ссылку на HashMap со всей имеющейся информацией
    /// об акции.
    pub fn info(&self) -> &HashMap<String, String> {
        self.iid.info()
    }
    /// Return the dir path with market data of share.
    ///
    /// # ru
    /// Возвращает путь к каталогу с рыночными данными акции.
    pub fn path(&self) -> PathBuf {
        self.iid.path()
    }

    /// Return chart.
    ///
    /// # ru
    /// Возвращает ссылку на график, или None если график заданного
    /// таймфрейма не загружен.
    pub fn chart(&self, tf: TimeFrame) -> Option<&Chart> {
        self.charts.get(&tf)
    }
    /// Return mutable chart.
    ///
    /// # ru
    /// Возвращает мутабельную ссылку на график (например, для добавление
    /// индикаторов на график), или None, если график заданного таймфрейма
    /// не загружен.
    pub fn chart_mut(&mut self, tf: TimeFrame) -> Option<&mut Chart> {
        self.charts.get_mut(&tf)
    }
    /// Load chart with default bars count. Return reference of loaded chart.
    ///
    /// # ru
    /// Загружает график с количеством баров по умолчанию, задается в
    /// конфиге пользователя. Возвращает ссылку на загруженный график.
    /// График сохраняется внутри актива.
    pub fn load_chart(&mut self, tf: TimeFrame) -> Result<&Chart, AvinError> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * CFG.core.default_bars_count as i32;

        self.load_chart_period(tf, &begin, &end)
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
        let end = Utc::now();
        let begin = end - tf.timedelta() * CFG.core.default_bars_count as i32;

        self.load_chart_period(tf, &begin, &end).unwrap();

        Ok(self.charts.get_mut(&tf).unwrap())
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
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, AvinError> {
        let chart = Chart::load(&self.iid, tf, begin, end)?;
        self.charts.insert(tf, chart);

        Ok(self.charts[&tf].as_ref())
    }
    /// Create empty chart with given timeframe, and store in self.
    ///
    /// # ru
    /// Создает пустой график для актива. Используется бэктестером.
    pub fn load_chart_empty(&mut self, tf: TimeFrame) -> &Chart {
        let chart = Chart::empty(&self.iid, tf);
        self.charts.insert(tf, chart);

        self.charts[&tf].as_ref()
    }

    /// Return vector of tics, if loaded, else None.
    ///
    /// # ru
    /// Возвращает вектор тиков, если они загружены, иначе None.
    pub fn tics(&self) -> Option<&Vec<Tic>> {
        if !self.tics.is_empty() {
            return Some(&self.tics);
        }

        None
    }
    /// Return footprint chart
    ///
    /// # ru
    /// Возвращает ссылку на кластерный график заданного таймфрейма.
    ///
    /// Сначала нужно загрузить тиковые данные [`Share::load_tics`], затем
    /// рассчитать кластеры для таймфрейма [`Share::build_footprint`]. Если
    /// это не сделано, вернет None.
    pub fn footprint(&self, tf: TimeFrame) -> Option<&Footprint> {
        self.footprints.get(&tf)
    }
    /// Return footprint chart
    ///
    /// # ru
    /// Возвращает мутабельную ссылку на кластерный график заданного
    /// таймфрейма.
    ///
    /// Сначала нужно загрузить тиковые данные [`Share::load_tics`], затем
    /// рассчитать кластеры для таймфрейма [`Share::build_footprint`]. Если
    /// это не сделано, вернет None.
    pub fn footprint_mut(
        &mut self,
        tf: &TimeFrame,
    ) -> Option<&mut Footprint> {
        self.footprints.get_mut(tf)
    }
    /// Load tics data.
    ///
    /// # ru
    /// Загружает тиковые данные по активу.
    pub fn load_tics(&mut self) -> Result<(), AvinError> {
        let begin = Utc::now().with_time(NaiveTime::MIN).unwrap();
        let end = Utc::now();

        match Manager::load(&self.iid, &MarketData::TIC, &begin, &end) {
            Ok(df) => {
                self.tics = Tic::from_df(&df).unwrap();
                Ok(())
            }
            Err(AvinError::NotFound(_)) => {
                self.tics = Vec::new();
                Ok(())
            }
            Err(err) => {
                log::error!("{err}");
                panic!();
            }
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
        let footprint = Footprint::from_tics(self.iid(), tf, &self.tics);
        self.footprints.insert(tf, footprint);

        Ok(())
    }

    /// Change per month
    ///
    /// # ru
    /// Изменение за месяц (процент тела бара, знаковое).
    /// Если график не загружен None.
    pub fn delta_month(&self) -> Option<f64> {
        let chart = self.charts.get(&TimeFrame::Month)?;
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
        let chart = self.charts.get(&TimeFrame::Week)?;
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
        let chart = self.charts.get(&TimeFrame::Day)?;
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
        let chart = self.charts.get(&TimeFrame::H1)?;
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
        let chart = self.charts.get(&TimeFrame::M10)?;
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
        let chart = self.charts.get(&TimeFrame::M1)?;
        let bar = chart.now()?;
        let delta = bar.body().delta_p();

        Some(delta)
    }

    /// Receive bar event
    ///
    /// # ru
    /// Принимает [`BarEvent`], чекает что эвент таймфрейм 1М, достает
    /// из него бар и добавляет во все имеющиеся графки. Графики сами
    /// разбираются в зависимости от своего таймфрейма как с этим баром
    /// 1М быть, автоматически склеивают все.
    ///
    /// Используется тестером и трейдером при получении нового бара из
    /// стрима данных.
    ///
    /// Не предназначена для прямого использования пользователем.
    pub fn bar_event(&mut self, e: BarEvent) {
        assert!(e.tf == TimeFrame::M1);
        for (_tf, chart) in self.charts.iter_mut() {
            chart.add_bar(e.bar);
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
impl std::fmt::Display for Share {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Share={} {}", self.exchange(), self.ticker())
    }
}
impl Hash for Share {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.figi().hash(state);
    }
}
impl PartialEq for Share {
    fn eq(&self, other: &Self) -> bool {
        self.figi() == other.figi()
    }
}
impl Eq for Share {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_new() {
        let share = Share::new("moex_share_sber").unwrap();
        assert_eq!(share.exchange(), "MOEX");
        assert_eq!(share.category(), "SHARE");
        assert_eq!(share.ticker(), "SBER");
        assert_eq!(share.figi(), "BBG004730N88");
    }
    #[test]
    fn load_chart() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::H1;
        let begin = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let chart = share.load_chart_period(tf, &begin, &end).unwrap();

        assert_eq!(chart.tf(), tf);
        assert_eq!(
            chart.first().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 3, 6, 0, 0).unwrap()
        );
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 31, 19, 0, 0).unwrap()
        );
        assert_eq!(
            chart.now().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 31, 20, 0, 0).unwrap()
        );
    }
    #[test]
    fn load_chart_no_args() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;

        let chart = share.load_chart(tf).unwrap();
        assert_eq!(chart.tf(), tf);

        assert!(!chart.bars().is_empty());
        assert!(chart.bars().len() <= CFG.core.default_bars_count);
    }
}
