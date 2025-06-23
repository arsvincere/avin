#!/usr/bin/env  python3
# ============================================================================
# URL:          http://arsvincjre.com
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import polars as pl
import tinkoff.invest as ti

from src.manager.category import Category
from src.manager.iid_cache import IidCache
from src.manager.source import Source
from src.utils import Cmd, cfg, dt_to_ts, log

SOURCE = Source.TINKOFF
TARGET = ti.constants.INVEST_GRPC_API
SCHEMA = pl.Schema(
    {
        "exchange": pl.String,
        "exchange_specific": pl.String,
        "category": pl.String,
        "ticker": pl.String,
        "figi": pl.String,
        "country": pl.String,
        "currency": pl.String,
        "sector": pl.String,
        "class_code": pl.String,
        "isin": pl.String,
        "uid": pl.String,
        "name": pl.String,
        "lot": pl.String,
        "step": pl.String,
        "long": pl.String,
        "short": pl.String,
        "long_qual": pl.String,
        "short_qual": pl.String,
        "first_1m": pl.String,
        "first_d": pl.String,
    }
)


class SourceTinkoff:
    __token = None

    # public
    @classmethod
    def cache_instruments_info(cls) -> None:
        log.info("Caching instruments info from Tinkoff")

        # Without authorization - not work
        cls.__ensure_auth()

        # tinkoff_categories = ["shares", "bonds", "futures", "currencies"]
        tinkoff_categories = ["shares"]
        for i in tinkoff_categories:
            df = cls.__request_instruments(i)
            category = cls.__to_avin_category(i)
            cache = IidCache(SOURCE, category, df)

            IidCache.save(cache)

    # private
    @classmethod
    def __ensure_auth(cls) -> None:
        # if auth true -> return
        if cls.__token:
            return

        # check file with token
        token_path = cfg.tinkoff_token
        if not Cmd.is_exist(token_path):
            log.error(
                "Tinkoff not exist token file, operations with market data "
                "and orders unavailible. Make a token and put it in a "
                f"'{token_path}'. Read more about token: "
                "https://developer.tinkoff.ru/docs/intro/"
                "manuals/self-service-auth"
            )
            exit(1)

        # read token and try connect
        token = Cmd.read(token_path).strip()
        try:
            with ti.Client(token) as client:
                response = client.users.get_accounts()
                if response:
                    cls.__token = token
                    log.info("Tinkoff Authorization successful")
                    return
        except ti.exceptions.UnauthenticatedError as err:
            log.exception(err)
            log.error(
                "Tinkoff authorization fault, check your token. "
                "Operations with market data unavailible. "
                f"Token='{token}'"
            )
            exit(1)

    @classmethod
    def __request_instruments(cls, tinkoff_category: str) -> pl.DataFrame:
        assert cls.__token is not None

        with ti.Client(cls.__token) as client:
            method = getattr(client.instruments, tinkoff_category)
            response: list[ti.Instrument] = method().instruments

        df_info = pl.DataFrame(schema=SCHEMA)
        for i in response:
            # skip unknown exchange
            info = cls.__extract_info(i)
            if info["exchange"] == "":
                continue

            row = pl.DataFrame(info)
            df_info.extend(row)

        return df_info

    @classmethod
    def __extract_info(cls, i: ti.Instrument) -> dict:
        # define short alias
        dec = ti.utils.quotation_to_decimal

        info = {
            "exchange": cls.__to_avin_exchange(i.exchange),
            "exchange_specific": i.exchange,  # original exchange name
            "category": "",  # seting below
            "ticker": i.ticker,
            "figi": i.figi,
            "country": i.country_of_risk,
            "currency": i.currency,
            "sector": "",  # seting below
            "class_code": i.class_code,
            "isin": "",  # seting below
            "uid": i.uid,
            "name": i.name,
            "lot": str(i.lot),
            "step": str(float(dec(i.min_price_increment))),
            "long": str(float(dec(i.dlong))),
            "short": str(float(dec(i.dshort))),
            "long_qual": str(float(dec(i.dlong_min))),
            "short_qual": str(float(dec(i.dshort_min))),
            "first_1m": str(dt_to_ts(i.first_1min_candle_date)),
            "first_d": str(dt_to_ts(i.first_1day_candle_date)),
        }

        # save attributes "isin" & "sector", if availible
        if hasattr(i, "isin"):
            info["isin"] = i.isin
        if hasattr(i, "sector"):
            info["sector"] = i.sector

        # # set standart instrument category
        if isinstance(i, ti.Share):
            info["category"] = Category.SHARE.name
        elif isinstance(i, ti.Bond):
            info["category"] = Category.BOND.name
        elif isinstance(i, ti.Future):
            info["category"] = Category.FUTURE.name
        elif isinstance(i, ti.Currency):
            info["category"] = Category.CURRENCY.name
        else:
            log.error(f"Unknown instrument category: {i}")
            exit(1)

        return info

    @classmethod
    def __to_avin_exchange(cls, name: str) -> str:
        if "MOEX" in name.upper():
            # values as "MOEX_PLUS", "MOEX_WEEKEND".. set "echange"="MOEX"
            standart_exchange_name = "MOEX"
        elif "SPB" in name.upper():
            # values as "SPB_RU_MORNING"... set "echange"="SPB"
            standart_exchange_name = "SPB"
        elif "FORTS" in name.upper():
            # NOTE:
            # FUTURE - у них биржа указана FORTS_EVENING, но похеру
            # пока для простоты ставлю им тоже биржу MOEX
            standart_exchange_name = "MOEX"
        elif name == "FX":
            # NOTE:
            # CURRENCY - у них биржа указана FX, но похеру
            # пока для простоты ставлю им тоже биржу MOEX
            standart_exchange_name = "MOEX"
        else:
            # NOTE:
            # там всякая странная хуйня еще есть в биржах
            # "otc_ncc", "LSE_MORNING", "moex_close", "Issuance",
            # "unknown"...
            # Часть из них по факту американские биржи, по которым сейчас
            # один хрен торги не доступны, другие хз, внебирживые еще, я всем
            # этим не торгую, поэтому сейчас ставим всем непонятным активам
            # биржу "", а потом перед сохранением делаем фильтр
            # если биржа "" - отбрасываем этот ассет из кэша
            standart_exchange_name = ""

        return standart_exchange_name

    @classmethod
    def __to_avin_category(cls, name: str) -> Category:
        names = {
            "shares": Category.SHARE,
            "bonds": Category.BOND,
            "futures": Category.FUTURE,
            "currencies": Category.CURRENCY,
        }

        return names[name]


if __name__ == "__main__":
    ...
