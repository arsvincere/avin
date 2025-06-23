#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

from datetime import UTC
from datetime import datetime as Date
from datetime import datetime as DateTime

from src.connect import SourceMoex, SourceTinkoff
from src.manager.category import Category
from src.manager.data_file_bar import DataFileBar
from src.manager.data_file_tic import DataFileTic
from src.manager.exchange import Exchange
from src.manager.iid import Iid
from src.manager.market_data import MarketData
from src.manager.source import Source
from src.utils import Cmd, cfg, log, now, ts_to_dt


class Manager:
    # public
    @classmethod
    def cache(cls, source: Source) -> None:
        """Make cache of instruments info"""

        log.info(":: Caching instruments info")
        match source:
            case Source.MOEX:
                SourceMoex.cache_instruments_info()
            case Source.TINKOFF:
                SourceTinkoff.cache_instruments_info()
            case _:
                log.error("Not implemented")
                exit(1)

    @classmethod
    def find(cls, s: str) -> Iid | None:
        """Find instrument id"""

        iid_opt = SourceMoex.find(s)
        return iid_opt

    @classmethod
    def download(
        cls,
        source: Source,
        iid: Iid,
        market_data: MarketData,
        *,
        year: int | None = None,
    ) -> None:
        assert isinstance(source, Source)
        assert isinstance(iid, Iid)
        assert isinstance(market_data, MarketData)
        log.info(f":: Download {iid.ticker()} {market_data.name}")

        match market_data:
            case MarketData.TIC:
                cls.__download_tics(source, iid, market_data)
            case _:  # bars
                cls.__download_bars(source, iid, market_data, year)

    @classmethod
    def update(
        cls,
        source: Source,
        iid: Iid,
        market_data: MarketData,
    ) -> None:
        assert isinstance(source, Source)
        assert isinstance(iid, Iid)
        assert isinstance(market_data, MarketData)
        assert source == Source.MOEX
        log.info(f":: Update {iid.ticker()} {market_data.name}")

        match market_data:
            case MarketData.TIC:
                cls.__update_tics(source, iid, market_data)
            case _:  # bars
                cls.__update_bars(source, iid, market_data)

    @classmethod
    def update_all(
        cls,
    ) -> None:
        log.info(":: Update all market data")

        # check data dir
        data_dir = cfg.data
        if not Cmd.is_exist(data_dir):
            log.error(f"Data dir not found: {data_dir}")
            exit(1)

        # for each exchange & for each category
        for e in Exchange:
            for c in Category:
                instrument_path = Cmd.path(data_dir, e.name, c.name)
                if not Cmd.is_exist(instrument_path):
                    continue

                # dir name == ticker
                dir_names = sorted(Cmd.get_dirs(instrument_path))
                for ticker in dir_names:
                    iid = cls.find(f"{e.name}_{c.name}_{ticker}")
                    assert iid is not None

                    # for each market data if exist
                    for md in MarketData:
                        path = Cmd.path(instrument_path, ticker, md.name)
                        if not Cmd.is_exist(path):
                            continue

                        cls.update(Source.MOEX, iid, md)

    # private
    @classmethod
    def __download_bars(
        cls, source: Source, iid: Iid, market_data: MarketData, year
    ):
        if year is None:
            cls.__download_bars_all_availible(source, iid, market_data)
        else:
            cls.__download_bars_one_year(source, iid, market_data, year)

    @classmethod
    def __download_bars_all_availible(
        cls, source: Source, iid: Iid, market_data: MarketData
    ) -> None:
        year = 1990
        end = now().year
        while year <= end:
            cls.__download_bars_one_year(source, iid, market_data, year)
            year += 1

    @classmethod
    def __download_bars_one_year(
        cls, source: Source, iid: Iid, market_data: MarketData, year: int
    ) -> None:
        assert source == Source.MOEX

        b = DateTime(year, 1, 1, tzinfo=UTC)
        e = DateTime(year + 1, 1, 1, tzinfo=UTC)

        df = SourceMoex.get_market_data(iid, market_data, begin=b, end=e)
        if df.is_empty():
            log.info(f"{year} no data")
            return

        file = DataFileBar(iid, market_data, df)
        DataFileBar.save(file)

    @classmethod
    def __download_tics(
        cls, source: Source, iid: Iid, market_data: MarketData
    ) -> None:
        assert source == Source.MOEX
        assert market_data == MarketData.TIC

        df = SourceMoex.get_market_data(iid, market_data)
        if df.is_empty():
            log.info(f"{Date.today()} no data")
            return

        file = DataFileTic(iid, market_data, df)
        DataFileTic.save(file)

    @classmethod
    def __update_bars(
        cls, source: Source, iid: Iid, market_data: MarketData
    ) -> None:
        # load last
        last_data = DataFileBar.load_last(iid, market_data)

        # get last datetime
        ts = last_data.df().item(-1, "ts_nanos")
        dt = ts_to_dt(ts)

        # request [last, now()]
        df = SourceMoex.get_market_data(iid, market_data, begin=dt, end=now())
        df = df[1:]  # remove first duplicate item

        if df.is_empty():
            log.info("no new bars")
        else:
            log.info(f"receved {len(df)} bars")
            last_data.add(df)
            DataFileBar.save(last_data)

    @classmethod
    def __update_tics(
        cls, source: Source, iid: Iid, market_data: MarketData
    ) -> None:
        # check today tics
        last_data = DataFileTic.load(iid, market_data, now().date())
        if last_data is None:
            cls.download(source, iid, market_data)
            return

        # get last tradeno
        n = last_data.df().item(-1, "tradeno")

        # request from trade n
        df = SourceMoex.get_market_data(iid, market_data, tradeno=n)
        df = df[1:]  # remove first duplicate item

        if df.is_empty():
            log.info("no new tics")
        else:
            log.info(f"receved {len(df)} tics")
            last_data.add(df)
            DataFileTic.save(last_data)


if __name__ == "__main__":
    ...
