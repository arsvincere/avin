#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

from datetime import date as Date

import polars as pl

from src.manager.iid import Iid
from src.manager.market_data import MarketData
from src.utils import Cmd, log, ts_to_dt


class DataFileTic:
    def __init__(self, iid: Iid, market_data: MarketData, df: pl.DataFrame):
        assert isinstance(iid, Iid)
        assert isinstance(market_data, MarketData)
        assert isinstance(df, pl.DataFrame)

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
    def save(cls, data: DataFileTic) -> None:
        assert isinstance(data, DataFileTic)

        iid = data.iid()
        market_data = data.market_data()
        df = data.df()
        date = ts_to_dt(df.item(0, "ts_nanos")).date()

        path = cls.__create_file_path(iid, market_data, date)
        Cmd.write_pqt(df, path)

        log.info(f"Save tics: {path}")

    @classmethod
    def load(
        cls, iid: Iid, market_data: MarketData, date: Date
    ) -> DataFileTic | None:
        assert isinstance(iid, Iid)
        assert isinstance(market_data, MarketData)
        assert isinstance(date, Date)

        path = cls.__create_file_path(iid, market_data, date)
        if not Cmd.is_exist(path):
            return None

        df = Cmd.read_pqt(path)
        data = DataFileTic(iid, market_data, df)

        return data

    @classmethod
    def __create_file_path(
        cls, iid: Iid, market_data: MarketData, date: Date
    ) -> str:
        file_path = Cmd.path(
            iid.path(),
            market_data.name,
            f"{date.year}",
            f"{date}.pqt",
        )

        return file_path
