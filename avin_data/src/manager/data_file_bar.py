#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

from datetime import UTC
from datetime import datetime as DateTime

import polars as pl

from src.manager.iid import Iid
from src.manager.market_data import MarketData
from src.utils import Cmd, dt_to_ts, log, ts_to_dt


class DataFileBar:
    def __init__(self, iid: Iid, market_data: MarketData, df: pl.DataFrame):
        assert isinstance(iid, Iid)
        assert isinstance(market_data, MarketData)
        assert isinstance(df, pl.DataFrame)

        b = ts_to_dt(df.item(0, "ts_nanos")).year
        e = ts_to_dt(df.item(-1, "ts_nanos")).year
        assert b == e

        self.__iid = iid
        self.__market_data = market_data
        self.__df = df

    def iid(self) -> Iid:
        return self.__iid

    def market_data(self) -> MarketData:
        return self.__market_data

    def df(self) -> pl.DataFrame:
        return self.__df

    def add(self, df: pl.DataFrame) -> None:
        self.__df.extend(df)

    @classmethod
    def save(cls, data: DataFileBar) -> None:
        assert isinstance(data, DataFileBar)

        iid = data.iid()
        market_data = data.market_data()

        # NOTE:
        # после data.add(df) в датафрейме может быть данные за
        # два разных года. Например при обновление в первых
        # числах января. Перед сохранением нужно проверить состав
        # датафрейма и сохранить кусками по годам.
        year = ts_to_dt(data.__df.item(0, "ts_nanos")).year
        end = ts_to_dt(data.__df.item(-1, "ts_nanos")).year
        while year <= end:
            begin_ts = dt_to_ts(DateTime(year, 1, 1, tzinfo=UTC))
            end_ts = dt_to_ts(DateTime(year + 1, 1, 1, tzinfo=UTC))

            year_df = data.__df.filter(
                pl.col("ts_nanos") >= begin_ts,
                pl.col("ts_nanos") < end_ts,
            )

            path = cls.__create_file_path(iid, market_data, year)
            Cmd.write_pqt(year_df, path)
            log.info(f"Save bars: {path}")

            year += 1

    @classmethod
    def load(
        cls, iid: Iid, market_data: MarketData, year: int
    ) -> DataFileBar:
        path = cls.__create_file_path(iid, market_data, year)
        df = Cmd.read_pqt(path)
        data = DataFileBar(iid, market_data, df)

        return data

    @classmethod
    def load_last(cls, iid: Iid, market_data: MarketData) -> DataFileBar:
        dir_path = cls.__create_dir_path(iid, market_data)

        if not Cmd.is_exist(dir_path):
            log.error(f"Data not found: {iid} {market_data} ({dir_path})")
            exit(1)

        files = sorted(Cmd.get_files(dir_path, full_path=True))
        if len(files) == 0:
            log.error(f"Data not found: {iid} {market_data} ({dir_path})")
            exit(1)

        last_file = files[-1]

        df = Cmd.read_pqt(last_file)
        data = DataFileBar(iid, market_data, df)

        return data

    @classmethod
    def __create_dir_path(cls, iid: Iid, market_data: MarketData) -> str:
        dir_path = Cmd.path(
            iid.path(),
            market_data.name,
        )

        return dir_path

    @classmethod
    def __create_file_path(
        cls, iid: Iid, market_data: MarketData, year: int
    ) -> str:
        dir_path = cls.__create_dir_path(iid, market_data)
        file_path = Cmd.path(
            dir_path,
            f"{year}.pqt",
        )

        return file_path
