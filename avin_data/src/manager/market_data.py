#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum
from datetime import datetime as DateTime
from datetime import timedelta as TimeDelta

from src.utils import log


class MarketData(enum.Enum):
    """Enum for selet what data type to download."""

    BAR_1M = "1M"
    BAR_5M = "5M"
    BAR_10M = "10M"
    BAR_1H = "1H"
    BAR_D = "D"
    BAR_W = "W"
    BAR_M = "M"
    TIC = "TIC"
    BOOK = "BOOK"
    TRADE_STATS = "TRADE_STATS"
    ORDER_STATS = "ORDER_STATS"
    OB_STATS = "OB_STATS"

    def __str__(self) -> str:
        return self.value

    def __hash__(self):
        return hash(self.name)

    def timedelta(self) -> TimeDelta:
        periods = {
            "1M": TimeDelta(minutes=1),
            "5M": TimeDelta(minutes=5),
            "10M": TimeDelta(minutes=10),
            "1H": TimeDelta(hours=1),
            "D": TimeDelta(days=1),
            "W": TimeDelta(weeks=1),
            # "M": TimeDelta(days=30),  # don't use it! it dangerous
            "TRADE_STATS": TimeDelta(minutes=5),
            "ORDER_STATS": TimeDelta(minutes=5),
            "OB_STATS": TimeDelta(minutes=5),
        }
        return periods[self.value]

    def prev_dt(self, dt: DateTime) -> DateTime:
        match self:
            case MarketData.BAR_1M:
                prev = dt.replace(second=0, microsecond=0)

            case MarketData.BAR_5M:
                prev = dt.replace(second=0, microsecond=0)
                past = dt.minute % 5
                prev -= TimeDelta(minutes=past)
            case MarketData.TRADE_STATS:
                prev = dt.replace(second=0, microsecond=0)
                past = dt.minute % 5
                prev -= TimeDelta(minutes=past)
            case MarketData.ORDER_STATS:
                prev = dt.replace(second=0, microsecond=0)
                past = dt.minute % 5
                prev -= TimeDelta(minutes=past)
            case MarketData.OB_STATS:
                prev = dt.replace(second=0, microsecond=0)
                past = dt.minute % 5
                prev -= TimeDelta(minutes=past)

            case MarketData.BAR_10M:
                prev = dt.replace(second=0, microsecond=0)
                past = dt.minute % 10
                prev -= TimeDelta(minutes=past)

            case MarketData.BAR_1H:
                prev = dt.replace(minute=0, second=0, microsecond=0)

            case MarketData.BAR_D:
                prev = dt.replace(hour=0, minute=0, second=0, microsecond=0)

            case MarketData.BAR_W:
                prev = dt.replace(hour=0, minute=0, second=0, microsecond=0)
                past = dt.weekday()
                prev -= TimeDelta(days=past)

            case MarketData.BAR_M:
                prev = dt.replace(
                    day=1, hour=0, minute=0, second=0, microsecond=0
                )

            case _:
                log.error(f"Not implemented prev_dt: {self}")
                exit(1)

        return prev

    def next_dt(self, dt: DateTime) -> DateTime:
        match self:
            case MarketData.BAR_1M:
                next = dt.replace(second=0, microsecond=0)
                next += TimeDelta(minutes=1)

            case MarketData.BAR_5M:
                next = dt.replace(second=0, microsecond=0)
                need_minutes = 5 - (dt.minute % 5)
                next += TimeDelta(minutes=need_minutes)
            case MarketData.TRADE_STATS:
                next = dt.replace(second=0, microsecond=0)
                need_minutes = 5 - (dt.minute % 5)
                next += TimeDelta(minutes=need_minutes)
            case MarketData.ORDER_STATS:
                next = dt.replace(second=0, microsecond=0)
                need_minutes = 5 - (dt.minute % 5)
                next += TimeDelta(minutes=need_minutes)
            case MarketData.OB_STATS:
                next = dt.replace(second=0, microsecond=0)
                need_minutes = 5 - (dt.minute % 5)
                next += TimeDelta(minutes=need_minutes)

            case MarketData.BAR_10M:
                next = dt.replace(second=0, microsecond=0)
                need_minutes = 10 - (dt.minute % 10)
                next += TimeDelta(minutes=need_minutes)

            case MarketData.BAR_1H:
                next = dt.replace(minute=0, second=0, microsecond=0)
                next += TimeDelta(hours=1)

            case MarketData.BAR_D:
                next = dt.replace(hour=0, minute=0, second=0, microsecond=0)
                next += TimeDelta(days=1)

            case MarketData.BAR_W:
                next = dt.replace(hour=0, minute=0, second=0, microsecond=0)
                need_days = 7 - dt.weekday()
                next += TimeDelta(days=need_days)

            case MarketData.BAR_M:
                if dt.month == 12:
                    next = dt.replace(
                        year=dt.year + 1,
                        month=1,
                        day=1,
                        hour=0,
                        minute=0,
                        second=0,
                        microsecond=0,
                    )
                else:
                    next = dt.replace(
                        month=dt.month + 1,
                        day=1,
                        hour=0,
                        minute=0,
                        second=0,
                        microsecond=0,
                    )

            case _:
                log.error(f"Not implemented next_dt: {self}")
                exit(1)

        return next

    @classmethod
    def from_str(cls, string: str) -> MarketData:
        if not isinstance(string, str):
            log.error(f"Invalid argument for market data: {string}")
            exit(1)

        types = {
            "1M": MarketData.BAR_1M,
            "5M": MarketData.BAR_5M,
            "10M": MarketData.BAR_10M,
            "1H": MarketData.BAR_1H,
            "D": MarketData.BAR_D,
            "W": MarketData.BAR_W,
            "M": MarketData.BAR_M,
            "BAR_1M": MarketData.BAR_1M,
            "BAR_5M": MarketData.BAR_5M,
            "BAR_10M": MarketData.BAR_10M,
            "BAR_1H": MarketData.BAR_1H,
            "BAR_D": MarketData.BAR_D,
            "BAR_W": MarketData.BAR_W,
            "BAR_M": MarketData.BAR_M,
            "TIC": MarketData.TIC,
            "BOOK": MarketData.BOOK,
            "TRADE_STATS": MarketData.TRADE_STATS,
            "ORDER_STATS": MarketData.ORDER_STATS,
            "OB_STATS": MarketData.OB_STATS,
        }

        result = types.get(string.upper())
        if result is None:
            log.error(f"Invalid market data name: {string}")
            exit(1)

        return result


if __name__ == "__main__":
    ...
