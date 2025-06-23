#!/usr/bin/env python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from datetime import UTC
from datetime import datetime as DateTime
from datetime import timedelta as TimeDelta
from datetime import timezone as TimeZone

from src.utils.conf import cfg


def dt_to_ts(dt: DateTime) -> int:
    assert isinstance(dt, DateTime)
    assert dt.tzinfo == UTC

    ts = dt.timestamp() * 1_000_000_000
    ts = int(ts)

    return ts


def next_month(dt: DateTime) -> DateTime:
    """Возвращает datetime первое число следующего месяца от полученного dt"""

    if dt.month == 12:
        next = dt.replace(
            year=dt.year + 1,
            month=1,
            day=1,
            hour=0,
            minute=0,
            second=0,
        )
    else:
        next = dt.replace(
            month=dt.month + 1,
            day=1,
            hour=0,
            minute=0,
            second=0,
        )

    return next


def now():
    return DateTime.now(UTC)


def now_local():
    return DateTime.now()


def prev_month(dt: DateTime) -> DateTime:
    """Возвращает datetime первое число предыдущего месяца от dt"""

    if dt.month == 1:
        next = dt.replace(
            year=dt.year - 1,
            month=12,
            day=1,
            hour=0,
            minute=0,
            second=0,
        )
    else:
        next = dt.replace(
            month=dt.month - 1,
            day=1,
            hour=0,
            minute=0,
            second=0,
        )

    return next


def str_to_utc(s: str) -> DateTime:
    """Get UTC datetime from naive string with Moscow date/datetime"""

    tz = TimeZone(TimeDelta(hours=3), "MSK")

    dt = DateTime.fromisoformat(s)
    dt = dt.replace(tzinfo=tz)
    dt = dt.astimezone(UTC)

    return dt


def ts_to_dt(ts_nanos: int) -> DateTime:
    ts_sec = ts_nanos / 1_000_000_000
    dt = DateTime.fromtimestamp(ts_sec, UTC)

    return dt


def utc_to_local(dt: DateTime) -> str:
    return (dt + cfg.offset).strftime(cfg.dt_fmt)


if __name__ == "__main__":
    ...
