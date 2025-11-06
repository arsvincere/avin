/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::prelude::*;
use polars::{
    df,
    prelude::{DataFrame, DataType, Field, Schema},
};

use crate::Range;
use avin_utils::{self as utils, AvinError};

/// Bar type.
///
/// # ru
/// Тип бара: бычий, доджи, медвежий.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarKind {
    /// Бычий
    Bull = 1,
    /// Доджи - открытие == закрытие
    Dodji = 0,
    /// Медвежий
    Bear = -1,
}

/// Like cundle, but more shortly name.
///
/// # ru
/// Бар - суть таже что и свеча, но слово короче.
///
/// Дата и время бара хранится в timestamp nanos как i64. Для
/// преобразования в человеко-читаемую дату время используется
/// крейт chrono, методы [`Bar::dt`] [`Bar::dt_local`].
///
/// В остальном все очевидно, хранит значения OHLCV.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bar {
    /// Timestamp nanos
    pub ts: i64,
    /// Open price
    pub o: f64,
    /// High price
    pub h: f64,
    /// Low price
    pub l: f64,
    /// Close price
    pub c: f64,
    /// Volume
    pub v: u64,
}
impl Bar {
    /// Create new bar.
    ///
    /// # ru
    /// Конструктор.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let b = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// assert_eq!(b.ts, 123456789);
    /// assert_eq!(b.o, 320.5);
    /// assert_eq!(b.h, 321.2);
    /// assert_eq!(b.l, 320.1);
    /// assert_eq!(b.c, 320.8);
    /// assert_eq!(b.v, 10);
    /// ```
    pub fn new(ts: i64, o: f64, h: f64, l: f64, c: f64, v: u64) -> Bar {
        Bar { ts, o, h, l, c, v }
    }
    /// Polars dataframe schema for bars.
    ///
    /// # ru
    /// Возвращает polars схему датафрейма для баров.
    pub fn schema() -> Schema {
        Schema::from_iter(vec![
            Field::new("ts_nanos".into(), DataType::Int64),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("volume".into(), DataType::UInt64),
        ])
    }
    /// Create bars from DataFrame.
    ///
    /// # ru
    /// Создает вектор баров из датафрейма.
    /// Датафрейм с рыночными данными создается модулем avin_data.
    ///
    /// ## Пример датафрейма:
    /// ```text
    /// ┌─────────────────────┬────────┬────────┬────────┬────────┬──────────┐
    /// │ ts_nanos            ┆ open   ┆ high   ┆ low    ┆ close  ┆ volume   │
    /// │ ---                 ┆ ---    ┆ ---    ┆ ---    ┆ ---    ┆ ---      │
    /// │ i64                 ┆ f64    ┆ f64    ┆ f64    ┆ f64    ┆ u64      │
    /// ╞═════════════════════╪════════╪════════╪════════╪════════╪══════════╡
    /// │ 1735851600000000000 ┆ 280.0  ┆ 280.41 ┆ 271.8  ┆ 272.25 ┆ 43086870 │
    /// │ 1736110800000000000 ┆ 270.88 ┆ 274.41 ┆ 270.07 ┆ 274.37 ┆ 28454750 │
    /// │ 1736283600000000000 ┆ 273.07 ┆ 277.87 ┆ 273.07 ┆ 277.0  ┆ 26634660 │
    /// │ 1736370000000000000 ┆ 276.71 ┆ 278.77 ┆ 270.73 ┆ 271.8  ┆ 52952880 │
    /// │ 1736456400000000000 ┆ 272.31 ┆ 279.53 ┆ 270.27 ┆ 278.77 ┆ 71154220 │
    /// │ …                   ┆ …      ┆ …      ┆ …      ┆ …      ┆ …        │
    /// ```
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    /// use polars::prelude::*;
    ///
    /// let df: DataFrame = df!(
    ///     "ts_nanos" => [123456789000000_i64, 123456790000000_i64],
    ///     "open" => [100.0, 101.0],
    ///     "high" => [110.0, 111.0],
    ///     "low" => [90.0, 91.0],
    ///     "close" => [105.0, 106.0],
    ///     "volume" => [10_u64, 20_u64],
    /// )
    /// .unwrap();
    ///
    /// let bars = Bar::from_df(&df).unwrap();
    /// assert_eq!(bars.len(), 2);
    /// ```
    pub fn from_df(df: &DataFrame) -> Result<Vec<Bar>, String> {
        let ts = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut o = df
            .column("open")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut h = df
            .column("high")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut l =
            df.column("low").unwrap().f64().unwrap().into_no_null_iter();
        let mut c = df
            .column("close")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut v = df
            .column("volume")
            .unwrap()
            .u64()
            .unwrap()
            .into_no_null_iter();

        let mut bars: Vec<Bar> = Vec::with_capacity(df.height());
        for t in ts {
            let bar = Bar::new(
                t,
                o.next().unwrap(),
                h.next().unwrap(),
                l.next().unwrap(),
                c.next().unwrap(),
                v.next().unwrap(),
            );
            bars.push(bar);
        }

        Ok(bars)
    }
    /// Create DataFrame from &[Bar].
    ///
    /// # ru
    /// Преобразовывает вектор баров в датафрейм.
    pub fn to_df(bars: &[Bar]) -> Result<DataFrame, AvinError> {
        let mut ts = Vec::new();
        let mut open = Vec::new();
        let mut high = Vec::new();
        let mut low = Vec::new();
        let mut close = Vec::new();
        let mut volume = Vec::new();

        for bar in bars.iter() {
            ts.push(bar.ts);
            open.push(bar.o);
            high.push(bar.h);
            low.push(bar.l);
            close.push(bar.c);
            volume.push(bar.v);
        }

        let df = df!(
            "ts_nanos" => ts,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => volume,
        )
        .unwrap();

        Ok(df)
    }
    /// Join self and other bar, used when converting timeframes.
    ///
    /// # ru
    /// Объединяет бар с другим. Используется для преобразования таймфреймов.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let b1 = Bar::new(123000000, 320.5, 321.2, 320.1, 320.8, 10);
    /// let b2 = Bar::new(124000000, 320.8, 322.2, 321.1, 321.8, 11);
    /// let joined = Bar::join(b1, b2);
    /// assert_eq!(joined.ts, b1.ts);
    /// assert_eq!(joined.o, b1.o);
    /// assert_eq!(joined.h, b2.h);
    /// assert_eq!(joined.l, b1.l);
    /// assert_eq!(joined.c, b2.c);
    /// assert_eq!(joined.v, b1.v + b2.v);
    /// ```
    #[inline]
    pub fn join(bar_1: Bar, bar_2: Bar) -> Bar {
        Bar {
            ts: bar_1.ts,
            o: bar_1.o,
            h: utils::max(bar_1.h, bar_2.h),
            l: utils::min(bar_1.l, bar_2.l),
            c: bar_2.c,
            v: utils::sum(bar_1.v, bar_2.v),
        }
    }

    /// Return DateTime UTC of bar.
    ///
    /// # ru
    /// Возвращает DateTime UTC бара.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    /// use chrono::prelude::*;
    ///
    /// let dt = Utc.with_ymd_and_hms(2025, 6, 22, 13, 1, 46).unwrap();
    /// let ts = dt.timestamp_nanos_opt().unwrap();
    /// let b = Bar::new(ts, 320.5, 321.2, 320.1, 320.8, 10);
    /// assert_eq!(b.dt(), dt);
    /// ```
    #[inline]
    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts)
    }
    /// Return local DateTime of bar without timezone (naive).
    ///
    /// # ru
    /// Возвращает DateTime бара в локальном времени, без таймзоны.
    #[inline]
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts);
        let local: DateTime<Local> = DateTime::from(utc);

        local.naive_local()
    }
    /// Bar type.
    ///
    /// # ru
    /// Возвращает тип бара, перечисление [`BarKind`]
    #[inline]
    pub fn kind(&self) -> BarKind {
        if self.is_bull() {
            BarKind::Bull
        } else if self.is_bear() {
            BarKind::Bear
        } else {
            BarKind::Dodji
        }
    }

    /// Check for bar is bear.
    ///
    /// # ru
    /// Если бар медвежий -> true, иначе -> false.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let b = Bar::new(123456789, 320.5, 321.2, 320.1, 320.2, 10);
    /// assert_eq!(b.is_bear(), true);
    /// ```
    #[inline]
    pub fn is_bear(&self) -> bool {
        self.o > self.c
    }
    /// Check for bar is bull.
    ///
    /// # ru
    /// Если бар бычий -> true, иначе -> false.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let b = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// assert_eq!(b.is_bull(), true);
    /// ```
    #[inline]
    pub fn is_bull(&self) -> bool {
        self.o < self.c
    }
    /// Check for bar is dodji.
    ///
    /// # ru
    /// Если открытие равно закрытию -> true, иначе -> false.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let b = Bar::new(123456789, 320.5, 321.2, 320.1, 320.5, 10);
    /// assert_eq!(b.is_dodji(), true);
    /// ```
    pub fn is_dodji(&self) -> bool {
        self.o == self.c
    }

    /// Full range of bar [bar.l, bar.h].
    ///
    /// # ru
    /// Возвращает полный диапазон бара [bar.l, bar.h].
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let bar = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let r = bar.full();
    /// assert_eq!(r.from, 320.1);
    /// assert_eq!(r.till, 321.2);
    /// ```
    #[inline]
    pub fn full(&self) -> Range {
        Range::new(self.l, self.h)
    }
    /// Body range of bar [bar.o, bar.c].
    ///
    /// # ru
    /// Возвращает диапазон тела бара [bar.o, bar.c].
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let bar = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let r = bar.body();
    /// assert_eq!(r.from, 320.5);
    /// assert_eq!(r.till, 320.8);
    /// ```
    #[inline]
    pub fn body(&self) -> Range {
        Range::new(self.o, self.c)
    }
    /// Lower shadow range of bar.
    ///
    /// # ru
    /// Возвращает диапазон нижней тени бара.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let bar = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let r = bar.lower();
    /// assert_eq!(r.from, 320.1);
    /// assert_eq!(r.till, 320.5);
    /// ```
    #[inline]
    pub fn lower(&self) -> Range {
        if self.is_bull() {
            Range::new(self.l, self.o)
        } else {
            Range::new(self.l, self.c)
        }
    }
    /// Upper shadow range of bar.
    ///
    /// # ru
    /// Возвращает диапазон верхней тени бара.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let bar = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// let r = bar.upper();
    /// assert_eq!(r.from, 320.8);
    /// assert_eq!(r.till, 321.2);
    /// ```
    #[inline]
    pub fn upper(&self) -> Range {
        if self.is_bull() {
            Range::new(self.c, self.h)
        } else {
            Range::new(self.o, self.h)
        }
    }
    /// Check for price in bar.
    ///
    /// # ru
    /// Проверка на вхождение цены в диапазон бара.
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Bar;
    ///
    /// let bar = Bar::new(123456789, 320.5, 321.2, 320.1, 320.8, 10);
    /// assert_eq!(bar.contains(320.6), true);
    /// assert_eq!(bar.contains(333.0), false);
    /// ```
    #[inline]
    pub fn contains(&self, price: f64) -> bool {
        self.l <= price && price <= self.h
    }
}
impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bar: dt={} o={} h={} l={} c={} v={}",
            self.dt_local(),
            self.o,
            self.h,
            self.l,
            self.c,
            self.v
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohlcv() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000);
        assert_eq!(b.dt(), dt);
        assert_eq!(b.o, 10.0);
        assert_eq!(b.h, 11.1);
        assert_eq!(b.l, 9.9);
        assert_eq!(b.c, 10.5);
        assert_eq!(b.v, 5000);
    }
    #[test]
    fn bear_bull() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000);
        assert!(b.is_bull());
        assert!(!b.is_bear());

        let b = Bar::new(ts, 10.0, 11.1, 9.0, 9.5, 5000);
        assert!(!b.is_bull());
        assert!(b.is_bear());
    }
    #[test]
    fn join() {
        let b1 = Bar::new(1, 100.0, 101.0, 99.0, 100.5, 5000);
        let b2 = Bar::new(2, 100.5, 101.2, 99.7, 100.8, 4000);

        let bar = Bar::join(b1, b2);
        assert_eq!(bar.ts, 1);
        assert_eq!(bar.o, 100.0);
        assert_eq!(bar.h, 101.2);
        assert_eq!(bar.l, 99.0);
        assert_eq!(bar.c, 100.8);
        assert_eq!(bar.v, 9000);
    }
}
