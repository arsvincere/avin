#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

import sys
from datetime import datetime as DateTime
from datetime import timedelta as TimeDelta
from pathlib import Path

sys.path.append("/home/alex/avin/avin_data_py")
from avin_data import *
from avin_data.utils.conf import Configuration


def test_utc_dt():
    dt = str_to_utc("2024-01-02")
    assert str(dt) == "2024-01-01 21:00:00+00:00"

    dt = str_to_utc("2025-01-01 10:00")
    assert str(dt) == "2025-01-01 07:00:00+00:00"


def test_dt_ts():
    ts_nanos = 1_000_000_000

    dt = ts_to_dt(ts_nanos)
    assert str(dt) == "1970-01-01 00:00:01+00:00"

    ts = dt_to_ts(dt)
    assert ts == 1_000_000_000


def test_prev_month():
    dt = DateTime(2023, 10, 30, 12, 20)
    dt = prev_month(dt)
    assert dt == DateTime(2023, 9, 1, 0, 0)
    dt = prev_month(dt)
    assert dt == DateTime(2023, 8, 1, 0, 0)
    dt = prev_month(dt)
    assert dt == DateTime(2023, 7, 1, 0, 0)

    dt = DateTime(2023, 1, 30, 11, 16, 15)
    dt = prev_month(dt)
    assert dt == DateTime(2022, 12, 1, 0, 0)


def test_next_month():
    dt = DateTime(2023, 1, 30, 12, 20)
    dt = next_month(dt)
    assert dt == DateTime(2023, 2, 1, 0, 0)
    dt = next_month(dt)
    assert dt == DateTime(2023, 3, 1, 0, 0)
    dt = next_month(dt)
    assert dt == DateTime(2023, 4, 1, 0, 0)

    dt = DateTime(2023, 12, 30, 11, 16)
    dt = next_month(dt)
    assert dt == DateTime(2024, 1, 1)


def test_configuration():
    cfg = Configuration.read_config()
    assert cfg.root == Path("/home/alex/trading")
    assert cfg.data == Path("/home/alex/trading/data")
    assert cfg.tinkoff_token == Path(
        "/home/alex/trading/connect/tinkoff/token.txt"
    )
    assert cfg.moex_account == Path(
        "/home/alex/trading/connect/moex/account.txt"
    )
    assert cfg.log == Path("/home/alex/trading/log")
    assert cfg.res == Path("/home/alex/trading/res")
    assert cfg.tmp == Path("/home/alex/trading/tmp")
    assert cfg.connect == Path("/home/alex/trading/connect")
    assert cfg.cache == Path("/home/alex/trading/data/cache")
    assert cfg.log_history == 5
    assert cfg.log_debug
    assert cfg.log_info
    assert cfg.offset == TimeDelta(hours=3)
    assert cfg.dt_fmt == "%Y-%m-%d %H:%M:%S"
