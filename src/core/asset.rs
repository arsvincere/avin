/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DATA_DIR;
use crate::conf::DEFAULT_BARS_COUNT;
use crate::core::chart::Chart;
use crate::core::timeframe::TimeFrame;
use chrono::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Asset {
    pub exchange: String,
    pub itype: String,
    pub ticker: String,

    charts: HashMap<TimeFrame, Chart>,
}
impl Asset {
    pub fn from(s: &str) -> Result<Asset, &'static str> {
        let parts: Vec<&str> = s.split("_").collect();
        if parts.len() != 3 {
            eprintln!("Fail to create asset from str: {s}");
            return Err("Invalid asset");
        };

        // TODO: пока работает только биржа MOEX
        let exchange = parts[0].to_uppercase();
        assert_eq!(exchange, "MOEX");

        // TODO: пока работает только тип инструмента SHARE
        let itype = parts[1].to_uppercase();
        assert_eq!(itype, "SHARE");

        // TODO: пока не сделал кэширование информации о доступных
        // инструментах работает только ниже указанные тикеры
        let ticker = parts[2].to_uppercase();
        assert!("GAZP LKOH MOEX ROSN SBER VTBR YDEX".contains(&ticker));

        let charts = HashMap::new();

        let asset = Asset {
            exchange,
            itype,
            ticker,
            charts,
        };
        Ok(asset)
    }
    pub fn copy_id(&self) -> Asset {
        Asset {
            exchange: self.exchange.clone(),
            itype: self.itype.clone(),
            ticker: self.ticker.clone(),
            charts: HashMap::new(),
        }
    }

    pub fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        self.charts.get(tf)
    }
    pub fn load_chart(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&Chart, &'static str> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;

        return self.load_chart_period(tf, &begin, &end);
    }
    pub fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str> {
        let chart = Chart::load(&self, tf, begin, end)?;
        self.charts.insert(tf.clone(), chart);

        Ok(self.charts[tf].as_ref())
    }
    pub fn path(&self) -> PathBuf {
        let mut p = std::path::PathBuf::new();
        p.push(&DATA_DIR);
        p.push(&self.exchange);
        p.push(&self.itype);
        p.push(&self.ticker);

        return p;
    }
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Asset={} {} {}", self.exchange, self.itype, self.ticker)
    }
}

// class Asset(Instrument, ABC):  # {{{
//     @abstractmethod  # __init__# {{{
//     def __init__(self, info: dict):
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//         Instrument.__init__(self, info)
//
//         # private fields
//         self.__charts: dict[TimeFrame, Chart] = dict()
//         self.__tics: Tics = Tics(self)
//
//         # signals
//         # self.newBar1M = AsyncSignal(Asset, Chart)
//         # self.newBar5M = AsyncSignal(Asset, Chart)
//         # self.newBar10M = AsyncSignal(Asset, Chart)
//         # self.newBar1H = AsyncSignal(Asset, Chart)
//         # self.newBarD = AsyncSignal(Asset, Chart)
//         # self.newBarW = AsyncSignal(Asset, Chart)
//         # self.newBarM = AsyncSignal(Asset, Chart)
//         self.chart_updated = Signal(Asset, Chart)
//         self.tics_updated = Signal(Asset)
//
//         # TODO: говнокод
//         self.__tryLoadTics()
//
//     # }}}
//
//     @property  # tics  # {{{
//     def tics(self) -> Tics:
//         return self.__tics
//
//     @tics.setter
//     def tics(self, tics: Tics) -> None:
//         assert isinstance(tics, Tics)
//         self.__tics = tics
//
//     # }}}
//     @property  # path  # {{{
//     def path(self) -> str:
//         dir_path = Cmd.path(
//             Usr.DATA, self.exchange.name, self.type.name, self.ticker
//         )
//         return dir_path
//
//     # }}}
//
//     def chart(self, timeframe: Union[TimeFrame, str]) -> Chart:  # {{{
//         logger.debug(f"{self.__class__.__name__}.chart()")
//
//         # convert type if needed
//         if isinstance(timeframe, str):
//             timeframe = TimeFrame(timeframe)
//
//         chart = self.__charts.get(timeframe, None)
//
//         if chart is None:
//             raise AssetError(f"Chart {self.ticker}-{timeframe} not cached")
//
//         return chart
//
//     # }}}
//     def setChart(self, chart: Chart) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.setChart()")
//
//         self.__charts[chart.timeframe] = chart
//
//     # }}}
//     def clearCache(self) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.clearCache()")
//         self.__charts.clear()
//
//     # }}}
//
//     async def cacheChart(  # {{{
//         self,
//         timeframe: Union[TimeFrame, str],
//         begin: Optional[DateTime] = None,
//         end: Optional[DateTime] = None,
//     ) -> None:
//         logger.debug(f"{self.__class__.__name__}.cacheChart()")
//
//         # format args
//         timeframe, begin, end = self.__formatArgs(timeframe, begin, end)
//
//         # load chart and keep it
//         chart = await Chart.load(self, timeframe, begin, end)
//         self.__charts[timeframe] = chart
//
//     # }}}
//     async def loadChart(  # {{{
//         self,
//         timeframe: Union[TimeFrame, str],
//         begin: Optional[DateTime] = None,
//         end: Optional[DateTime] = None,
//     ) -> Chart:
//         logger.debug(f"{self.__class__.__name__}.loadChart()")
//
//         # format args
//         timeframe, begin, end = self.__formatArgs(timeframe, begin, end)
//
//         # load chart and return it
//         chart = await Chart.load(self, timeframe, begin, end)
//         return chart
//
//     # }}}
//     async def loadData(  # {{{
//         self,
//         timeframe: Union[TimeFrame, str],
//         begin: DateTime,
//         end: DateTime,
//     ) -> pl.DataFrame:
//         logger.debug(f"{self.__class__.__name__}.loadData()")
//
//         # check and convert args
//         assert begin <= end
//         if isinstance(timeframe, str):
//             data_type = DataType.fromStr(timeframe)
//         elif isinstance(timeframe, TimeFrame):
//             data_type = timeframe.toDataType()
//         else:
//             raise TypeError(f"Invalid timeframe='{timeframe}'")
//
//         # request bars records
//         records = await Data.request(self, data_type, begin, end)
//
//         # create & return DataFrame
//         df = pl.DataFrame([dict(r) for r in records])
//         return df
//
//     # }}}
//
//     def receive(self, event: Event) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.receive({event})")
//
//         match event.type:
//             case Event.Type.BAR:
//                 self.__receiveBar(event)
//             case Event.Type.TIC:
//                 self.__receiveTic(event)
//
//     # }}}
//
//     @classmethod  # fromStr# {{{
//     async def fromStr(cls, string: str) -> Asset:
//         logger.debug(f"{cls.__name__}.fromStr()")
//
//         # string is like "MOEX SHARE SBER"
//         exchange, itype, ticker = string.upper().split()
//
//         # convert types
//         exchange = Exchange.fromStr(exchange)
//         itype = Asset.Type.fromStr(itype)
//
//         # request and return asset
//         asset = await cls.fromTicker(exchange, itype, ticker)
//         return asset
//
//     # }}}
//     @classmethod  # fromRecord# {{{
//     def fromRecord(cls, record: asyncpg.Record) -> Asset:
//         logger.debug(f"{cls.__name__}.fromRecord()")
//
//         # NOTE: asyncpg.Record работает идентично словарю,
//         # а название столбцов в БД совпадает с тем что в словаре класс
//         # будет искать.
//         asset = cls.__getCertainAssetClass(record)
//
//         return asset
//
//     # }}}
//     @classmethod  # fromInfo# {{{
//     def fromInfo(cls, info: dict) -> Asset:
//         logger.debug(f"{cls.__name__}.fromInfo()")
//
//         asset = cls.__getCertainAssetClass(info)
//         return asset
//
//     # }}}
//     @classmethod  # fromInstrument# {{{
//     def fromInstrument(cls, instrument: Instrument) -> Asset:
//         logger.debug(f"{cls.__name__}.fromInstrument()")
//
//         asset = cls.__getCertainAssetClass(instrument.info)
//         return asset
//
//     # }}}
//     @classmethod  # fromTicker# {{{
//     async def fromTicker(
//         cls, exchange: ClassVar, asset_type: Asset.Type, ticker: str
//     ) -> Asset:
//         logger.debug(f"{cls.__name__}.fromTicker()")
//         assert isinstance(asset_type, cls.Type)
//         assert hasattr(exchange, "name")
//         assert isinstance(ticker, str)
//
//         asset = await Keeper.get(
//             Asset, asset_type=asset_type, exchange=exchange, ticker=ticker
//         )
//         return asset
//
//     # }}}
//     @classmethod  # fromFigi# {{{
//     async def fromFigi(cls, figi: str) -> Asset:
//         logger.debug(f"{cls.__name__}.fromFigi()")
//
//         asset = await Keeper.get(Asset, figi=figi)
//         return asset
//
//     # }}}
//     @classmethod  # fromUid# {{{
//     async def fromUid(cls, uid: str) -> Asset:
//         logger.debug(f"{cls.__name__}.fromUid()")
//
//         instrument = await Instrument.fromUid(uid)
//         asset = cls.__getCertainAssetClass(instrument.info)
//         return asset
//
//     # }}}
//     @classmethod  # toInstrument# {{{
//     def toInstrument(cls, asset: Asset) -> Instrument:
//         logger.debug(f"{cls.__name__}.toInstrument()")
//
//         instrument = Instrument(asset.info)
//         return instrument
//
//     # }}}
//     @classmethod  # requestAll# {{{
//     async def requestAll(cls) -> list[Asset]:
//         logger.debug(f"{cls.__name__}.requestAll()")
//
//         assets = await Keeper.get(cls)
//         return assets
//
//     # }}}
//
//     def __receiveBar(self, event: BarEvent) -> None:  # {{{
//         # select chart
//         timeframe = event.timeframe
//         chart = self.chart(timeframe)
//
//         # send bar to chart
//         chart.receive(event.bar)
//
//         # emit signal
//         self.chart_updated.emit(self, chart)
//
//     # }}}
//     def __receiveTic(self, event: TicEvent) -> None:  # {{{
//         assert event.type == Event.Type.TIC
//         assert event.figi == self.figi
//
//         self.__tics.add(event.tic)
//         self.tics_updated.emit(self)
//
//     # }}}
//     def __tryLoadTics(self) -> None:
//         dir_path = Cmd.path(self.path, DataType.TIC.name)
//
//         date = Date.today()
//         file_name = f"{date} tic.parquet"
//
//         file_path = Cmd.path(dir_path, file_name)
//
//         if Cmd.isExist(file_path):
//             df = pl.read_parquet(file_path)
//             self.__tics = Tics(self, df)
//
//     @classmethod  # __formatArgs# {{{
//     def __formatArgs(
//         cls, timeframe, begin, end
//     ) -> tuple[TimeFrame, DateTime, DateTime]:
//         logger.debug(f"{cls.__name__}.__formatArgs()")
//
//         # check timeframe
//         if isinstance(timeframe, TimeFrame):
//             pass
//         elif isinstance(timeframe, str):
//             timeframe = TimeFrame(timeframe)
//         else:
//             logger.critical(
//                 f"Wrong timeframe='{timeframe}', use valid 'str' "
//                 "or class TimeFrame"
//             )
//             raise TypeError(timeframe)
//
//         # when begin & end == None, load DEFAULT_BARS_COUNT
//         if begin is None and end is None:
//             period = timeframe * Chart.DEFAULT_BARS_COUNT
//             begin = now().replace(microsecond=0) - period
//             end = now()
//
//         # 'begin', 'end' must be datetime
//         if not isinstance(begin, DateTime):
//             raise TypeError(f"Invalid begin='{begin}'")
//         if not isinstance(end, DateTime):
//             raise TypeError(f"Invalid end='{end}'")
//
//         return timeframe, begin, end
//
//     # }}}
//     @classmethod  # __getCertainAssetClass# {{{
//     def __getCertainAssetClass(cls, info: dict):
//         logger.debug(f"{cls.__name__}.__getCertainAssetClass()")
//
//         t = info["type"]
//         if t == cls.Type.INDEX.name:
//             return Index(info)
//         elif t == cls.Type.SHARE.name:
//             return Share(info)
//         elif t == cls.Type.FUTURE.name:
//             return Future(info)
//
//         logger.critical(f"Unknown asset type={t}")
//         assert False
//
//     # }}}
//
// # }}}
// class Index(Asset):  # {{{
//     def __init__(self, info: dict):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//         assert info["type"] == Asset.Type.INDEX.name
//
//         super().__init__(info)
//
//     # }}}
//
// # }}}
// class Share(Asset):  # {{{
//     def __init__(self, info: dict):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//         assert info["type"] == Asset.Type.SHARE.name
//
//         super().__init__(info)
//
//     # }}}
//     @property  # uid{{{
//     def uid(self):
//         return self.info["uid"]
//
//     # }}}
//
// # }}}
// class Future(Asset):  # {{{
//     def __init__(self, info: dict):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//         assert info["type"] == Asset.Type.FUTURE.name
//
//         super().__init__(info)
//
//     # }}}
//     @property  # uid{{{
//     def uid(self):
//         return self.info["uid"]
//
//     # }}}
//
// # }}}
// class AssetList:  # {{{
//     def __init__(self, name: str, assets: Optional[list] = None):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__init__({name})")
//
//         self.__name = name
//         self.__assets = assets if assets else list()
//
//     # }}}
//     def __str__(self) -> str:  # {{{
//         return f"AssetList={self.__name}"
//
//     # }}}
//     def __getitem__(self, index: int) -> Asset:  # {{{
//         assert index < len(self.__assets)
//         return self.__assets[index]
//
//     # }}}
//     def __iter__(self) -> Iterator:  # {{{
//         return iter(self.__assets)
//
//     # }}}
//     def __contains__(self, asset: Asset) -> bool:  # {{{
//         return any(i == asset for i in self.__assets)
//
//     # }}}
//     def __len__(self):  # {{{
//         return len(self.__assets)
//
//     # }}}
//
//     @property  # name  # {{{
//     def name(self) -> str:
//         return self.__name
//
//     @name.setter
//     def name(self, name: str):
//         assert isinstance(name, str)
//         self.__name = name
//
//     # }}}
//     @property  # assets  # {{{
//     def assets(self) -> list[Asset]:
//         return self.__assets
//
//     @assets.setter
//     def assets(self, assets: list[Asset]):
//         assert isinstance(assets, list)
//         for i in assets:
//             assert isinstance(i, Asset)
//
//         self.__assets = assets
//
//     # }}}
//
//     def add(self, asset: Asset) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.add({asset.ticker})")
//         assert isinstance(asset, Asset)
//         if asset not in self:
//             self.__assets.append(asset)
//             return
//
//         logger.warning(f"{asset} already in list '{self.name}'")
//
//     # }}}
//     def remove(self, asset: Asset) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.remove({asset.ticker})")
//
//         try:
//             self.__assets.remove(asset)
//         except ValueError:
//             logger.exception(
//                 f"AssetList.remove(asset) failed: '{asset}' not in list",
//             )
//
//     # }}}
//     def clear(self) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.clear()")
//
//         self.__assets.clear()
//
//     # }}}
//     def find(  # {{{
//         self, instrument: Instrument | None = None, figi: str | None = None
//     ) -> Asset | None:
//         logger.debug(f"{self.__class__.__name__}.find()")
//         if instrument:
//             return self.__findById(instrument)
//         if figi:
//             asset = self.__findByFigi(figi)
//             return asset
//
//         assert False, "Bad arguments"
//
//     # }}}
//
//     @classmethod  # fromRecord  # {{{
//     async def fromRecord(
//         cls,
//         name: str,
//         record: asyncpg.Record
//     ) -> AssetList:
//         logger.debug(f"{cls.__name__}.fromRecord()")
//
//         alist = cls(name)
//         for i in record:
//             asset = Asset.fromRecord(i)
//             alist.add(asset)
//
//         return alist
//
//     # }}}
//     @classmethod  # save  # {{{
//     async def save(cls, asset_list: AssetList) -> None:
//         logger.debug(f"{cls.__name__}.save()")
//         assert isinstance(asset_list, AssetList)
//
//         await Keeper.delete(asset_list)
//         await Keeper.add(asset_list)
//
//     # }}}
//     @classmethod  # load  # {{{
//     async def load(cls, name: str) -> AssetList | None:
//         logger.debug(f"{cls.__name__}.load()")
//
//         alist = await Keeper.get(cls, name=name)
//         return alist
//
//     # }}}
//     @classmethod  # delete  # {{{
//     async def delete(cls, asset_list: AssetList) -> None:
//         logger.debug(f"{cls.__name__}.delete()")
//         assert isinstance(asset_list, AssetList)
//
//         await Keeper.delete(asset_list)
//
//     # }}}
//     @classmethod  # rename  # {{{
//     async def rename(cls, asset_list: AssetList, new_name: str) -> None:
//         logger.debug(f"{cls.__name__}.rename()")
//         assert isinstance(new_name, str)
//         assert len(new_name) > 0
//
//         await cls.delete(asset_list)
//         asset_list.name = new_name
//         await cls.save(asset_list)
//
//     # }}}
//     @classmethod  # copy  # {{{
//     async def copy(cls, asset_list: AssetList, new_name: str) -> None:
//         logger.debug(f"{cls.__name__}.copy()")
//
//         new_list = AssetList(new_name)
//         new_list.assets = asset_list.assets
//         await cls.save(new_list)
//
//     # }}}
//     @classmethod  # requestAll# {{{
//     async def requestAll(cls) -> list[str]:
//         logger.debug(f"{cls.__name__}.requestAll()")
//
//         names = await Keeper.get(cls, get_only_names=True)
//         return names
//
//     # }}}
//
//     def __findById(self, instrument: Instrument) -> Asset | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.__findById()")
//         for i in self.__assets:
//             if i == instrument:
//                 return i
//
//         return None
//
//     # }}}
//     def __findByFigi(self, figi: str) -> Asset | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.__findByFigi()")
//         for i in self.__assets:
//             if i.figi == figi:
//                 return i
//
//         return None
//
//     # }}}
//
// # }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_chart() {
        let mut asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("1H");
        let begin = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let chart = asset.load_chart_period(&tf, &begin, &end).unwrap();

        assert_eq!(chart.tf(), &tf);
        assert_eq!(
            chart.first().unwrap().dt,
            Utc.with_ymd_and_hms(2025, 1, 3, 6, 0, 0).unwrap()
        );
        assert_eq!(
            chart.last().unwrap().dt,
            Utc.with_ymd_and_hms(2025, 1, 31, 20, 0, 0).unwrap()
        );
    }
    #[test]
    fn load_chart_no_args() {
        let mut asset = Asset::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = asset.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
