#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

import sys
from datetime import datetime as DateTime

sys.path.append("/home/alex/avin/avin_data")
from avin_data import *


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
