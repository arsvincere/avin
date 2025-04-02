/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::asset::Asset;
use crate::core::bar::Bar;
use crate::core::timeframe::TimeFrame;
use crate::data::Manager;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Chart {
    tf: TimeFrame,
    bars: Vec<Bar>,
    now: Option<Bar>,
}

impl Chart {
    /// Return chart timeframe
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
    }
    /// Return bars of chart
    pub fn bars(&self) -> &Vec<Bar> {
        &self.bars
    }
    /// Return fist historical bar of chart
    pub fn first(&self) -> Option<&Bar> {
        self.bars.first()
    }
    /// Return last historical bar of chart
    pub fn last(&self) -> Option<&Bar> {
        self.bars.last()
    }
    /// Return real-time bar of chart
    pub fn now(&self) -> Option<&Bar> {
        self.now.as_ref()
    }
    /// Return real-time bar close
    pub fn last_price(&self) -> Option<f64> {
        match &self.now {
            Some(bar) => Some(bar.c),
            None => None,
        }
    }

    pub fn new(_asset: &Asset, tf: &TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            tf: tf.clone(),
            bars,
            now: None,
        }
    }
    pub fn empty(_asset: &Asset, tf: TimeFrame) -> Self {
        Self {
            tf,
            bars: Vec::new(),
            now: None,
        }
    }
    pub fn load(
        asset: &Asset,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        let bars =
            Manager::request(&asset, &tf.to_market_data(), begin, end)?;

        let chart = Self {
            tf: tf.clone(),
            bars,
            now: None,
        };
        Ok(chart)
    }
}
impl AsRef<Chart> for Chart {
    fn as_ref(&self) -> &Chart {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let bars =
            Manager::request(&asset, &tf.to_market_data(), &begin, &end)
                .unwrap();

        let chart = Chart::new(&asset, &tf, bars);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 256);
        assert!(chart.now.is_none());
    }
    #[test]
    fn empty() {
        let asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = Chart::empty(&asset, tf.clone());
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
        assert!(chart.now.is_none());
    }
    #[test]
    fn load() {
        let asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

        let chart = Chart::load(&asset, &tf, &begin, &end).unwrap();
        assert_eq!(chart.tf(), &tf);
        assert_eq!(chart.bars().len(), 256);
        assert!(chart.now().is_none());

        assert_eq!(
            chart.first().unwrap().dt,
            Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(),
        );
        assert_eq!(
            chart.last().unwrap().dt,
            Utc.with_ymd_and_hms(2024, 12, 30, 0, 0, 0).unwrap(),
        )
    }
}
// class Chart:
//     DEFAULT_BARS_COUNT = 5000
//     MAX_BARS_COUNT = None  # used in tester
//
//     def __init__(  # {{{
//         self,
//         asset: Asset,
//         timeframe: TimeFrame,
//         data: pl.DataFrame,
//     ):
//
//         self.__indicators = list()
//
//         # signals
//         self.new_bar = Signal(Chart, Bar)
//         self.upd_bar = Signal(Chart, Bar)
//
//     # }}}
//     def __getitem__(self, index: int):  # {{{
//         """Доступ к барам графика по индексу
//         ----------------------------------------------------------------------
//         [0, 1, 2, 3] (now_bar)  - так данные лежат физически
//          4  3  2  1      0      - так через getitem [i]
//
//         chart[0]                - возвращаем реал тайм бар self.__now
//         chart[1] == data[-1]    - указывает на вчерашний бар
//         chart[2] == data[-2]    - указывает на позавчера
//         chart[3] == data[-3]    - ...
//         chart[4] == data[-4]    - самый старый исторический
//         сhart[5] (>len(data)    - возвращаем None
//         ----------------------------------------------------------------------
//         Если head установить == 0, тогда:
//         chart[0]     перехватываем и возвращаем реал тайм бар,
//         chart[1] == 0 - 1 < 0 перехватываем и возвращаем None
//         """
//
//         if index == 0:
//             return self.__now  # возвращаем реал тайм бар
//
//         if index <= len(self.__data):
//             dct = self.__data[-index]
//             bar = Bar(dct, self)
//             return bar
//
//         if index > len(self.__data):
//             return None
//
//         if index < 0:
//             return None
//
//     # }}}
//     def __iter__(self) -> Iterable:  # {{{
//         for dct in self.__data.iter_rows(named=True):
//             bar = Bar(data=dct, chart=self)
//             yield bar
//
//     # }}}
//     def __len__(self) -> int:  # {{{
//         """Return count of historical bars"""
//
//         return len(self.__data)
//
//     # }}}
//     def __str__(self) -> str:  # {{{
//         s = f"Chart={self.__asset}-{self.__timeframe}"
//         return s
//
//     # }}}
//
//     @property  # asset  # {{{
//     def asset(self) -> Asset:
//         return self.__asset
//
//     # }}}
//     @property  # data_frame  # {{{
//     def data_frame(self) -> pl.DataFrame:
//         return self.__data
//
//     # }}}
//     @property  # data_frame_with_now  # {{{
//     def data_frame(self) -> pl.DataFrame:
//         return self.__data
//
//     # }}}
//
//     def addInd(self, ind: Indicator) -> None:  # {{{
//         for i in self.__indicators:
//             if i.name == ind.name:
//                 logger.warning(f"{ind} already on {self}")
//                 return
//
//         ind.setChart(self)
//         self.__indicators.append(ind)
//
//     # }}}
//     def getInd(self, name: str) -> Indicator | None:  # {{{
//         for i in self.__indicators:
//             if i.name == name:
//                 return i
//
//         return None
//
//     # }}}
//     def getAllInd(self) -> list[Indicator]:  # {{{
//         return self.__indicators
//
//     # }}}
//     def removeInd(self, ind: Indicator) -> None:  # {{{
//         try:
//             self.__indicators.remove(ind)
//         except ValueError:
//             logger.exception(
//                 f"Chart.removeInd(ind) failed: '{ind}' not in {self}"
//             )
//
//     # }}}
//     def removeAllInd(self) -> None:  # {{{
//         self.__indicators.clear()
//
//     # }}}
//     def highestHigh(self):  # {{{
//         return self.__data["high"].max()
//
//     # }}}
//     def lowestLow(self):  # {{{
//         return self.__data["low"].min()
//
//     # }}}
//
//     def select(self, b: DateTime, e: DateTime) -> pl.DataFrame:  # {{{
//         selected = self.__data.filter(
//             (b <= pl.col("dt")) & (pl.col("dt") <= e)
//         )
//
//         return selected
//
//     # }}}
//     def selectTodayBars(self) -> pl.DataFrame:  # {{{
//         """Return df bars with dt.date() == chart.now.dt.date()"""
//
//         if self.__now is None:
//             return pl.DataFrame()
//
//         b = self.__now.dt.replace(hour=0, minute=0, second=0, microsecond=0)
//         e = b + ONE_DAY
//
//         return self.__selectHalfClosed(b, e)
//
//     # }}}
//     def selectBarsOfYear(self, dt: DateTime) -> pl.DataFrame:  # {{{
//         """Return df with year the same, as year of argument 'dt'"""
//
//         b = DateTime(dt.year, 1, 1, tzinfo=UTC)
//         e = DateTime(dt.year + 1, 1, 1, tzinfo=UTC)
//
//         return self.__selectHalfClosed(b, e)
//
//     # }}}
//     def selectBarsOfMonth(self, dt: DateTime) -> pl.DataFrame:  # {{{
//         """Return df with month the same, as month of argument 'dt'"""
//
//         if self.__timeframe >= TimeFrame("M"):
//             return pl.DataFrame()
//
//         b = DateTime(dt.year, dt.month, 1, tzinfo=UTC)
//         e = next_month(b)
//
//         return self.__selectHalfClosed(b, e)
//
//     # }}}
//     def selectBarsOfWeek(self, dt: DateTime) -> pl.DataFrame:  # {{{
//         """Return df with week the same, as week of argument 'dt'"""
//
//         if self.__timeframe >= TimeFrame("W"):
//             return pl.DataFrame()
//
//         # TODO: тут можно сделать функции типа
//         # get_week_begin(dt)
//         # get_week_end(dt)
//         # в утилитах, и от этого плясать
//         assert False, "TODO_ME"
//
//     # }}}
//     def selectBarsOfDay(self, dt: DateTime) -> pl.DataFrame:  # {{{
//         """Return df with day the same, as day of argument 'dt'"""
//
//         if self.__timeframe >= TimeFrame("D"):
//             return pl.DataFrame()
//
//         b = dt.replace(hour=0, minute=0, second=0, microsecond=0)
//         e = b + ONE_DAY
//
//         return self.__selectHalfClosed(b, e)
//
//     # }}}
//     def selectBarsOfHour(self, dt: DateTime) -> pl.DataFrame:  # {{{
//         """Return df with hour the same, as hour of argument 'dt'"""
//
//         if self.__timeframe >= TimeFrame("1H"):
//             return pl.DataFrame()
//
//         b = dt.replace(minute=0, second=0, microsecond=0)
//         e = b + ONE_HOUR
//
//         return self.__selectHalfClosed(b, e)
//
//     # }}}
//
//     def todayOpen(self):  # {{{
//         bars = self.selectTodayBars()
//         if bars.is_empty():
//             return None
//
//         return bars.item(0, "open")
//
//     # }}}
//     def todayHigh(self):  # {{{
//         bars = self.selectTodayBars()
//         if bars.is_empty():
//             return None
//
//         return bars["high"].max()
//
//     # }}}
//     def todayLow(self):  # {{{
//         bars = self.selectTodayBars()
//         if bars.is_empty():
//             return None
//
//         return bars["low"].min()
//
//     # }}}
//     def yesterdayOpen(self):  # {{{
//         yesterday = self.__now.dt - ONE_DAY
//
//         bars = self.selectBarsOfDay(yesterday)
//         if bars.is_empty():
//             return None
//
//         return bars.item(0, "open")
//
//     # }}}
//     def yesterdayHigh(self):  # {{{
//         yesterday = self.__now.dt - ONE_DAY
//
//         bars = self.selectBarsOfDay(yesterday)
//         if bars.is_empty():
//             return None
//
//         return bars["high"].max()
//
//     # }}}
//     def yesterdayLow(self):  # {{{
//         yesterday = self.__now.dt - ONE_DAY
//
//         bars = self.selectBarsOfDay(yesterday)
//         if bars.is_empty():
//             return None
//
//         return bars["low"].min()
//
//     # }}}
//     def yesterdayClose(self):  # {{{
//         yesterday = self.__now.dt - ONE_DAY
//
//         bars = self.selectBarsOfDay(yesterday)
//         if bars.is_empty():
//             return None
//
//         return bars.item(-1, "close")
//
//     # }}}
//
//     def receive(self, e: BarEvent) -> None:  # {{{
//         assert e.type == Event.Type.BAR
//         assert e.figi == self.__asset.figi
//         assert e.timeframe == self.__timeframe
//
//         new_bar = e.bar
//         new_bar.setChart(self)
//
//         # 1. first real-time bar, in chart only historical bars
//         if self.__now is None:
//             self.__now = new_bar
//
//             self.upd_bar.emit(self, new_bar)
//             return
//
//         # 2. only update now bar
//         if self.__now.dt == new_bar.dt:
//             self.__now = new_bar
//
//             self.upd_bar.emit(self, new_bar)
//             return
//
//         # 3. new historical bar and update now bar
//         if self.__now.dt < new_bar.dt:
//             df = pl.DataFrame(self.__now.data)
//             self.__data.extend(df)
//             self.__now = new_bar
//
//             # emit signals
//             self.new_bar.emit(self, self.last)
//             self.upd_bar.emit(self, self.now)
//             return
//
//         # 4. Тинькоф иногда в поток докидывает старые бары исторические
//         # но исправленные, пересчитанные. В пизду их пока даже внимание
//         # не буду обращать, там не большое отличие
//         if self.__now.dt > new_bar.dt:
//             logger.warning(f"Receiving event={e}")
//             logger.warning(f"self.now={self.now}")
//             logger.warning(f"self.last={self.last}")
//             return
//
//         assert False, "WTF???"
//
//     # }}}
//
//     def __selectHalfClosed(  # {{{
//         self, b: DateTime, e: DateTime
//     ) -> pl.DataFrame:
//         selected = self.__data.filter(
//             (b <= pl.col("dt")) & (pl.col("dt") < e)
//         )
//
//         return selected
//
//     # }}}
