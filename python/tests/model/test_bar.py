# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import UTC
from datetime import datetime as DateTime

from avin import Bar, BarDirection, PriceRange


def test_ohlcv_ts_dt():
    dt = DateTime(2026, 8, 20, 14, 20, 5, tzinfo=UTC)
    ts = int(dt.timestamp()) * 1_000_000_000
    bar = Bar(ts, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar.ts == ts
    assert bar.o == 10.0
    assert bar.h == 11.1
    assert bar.l == 9.9
    assert bar.c == 10.5
    assert bar.v == 5000
    assert bar.dt() == dt


def test_direction():
    ts = 123_456_789
    vol = 5000

    bull_bar = Bar(ts, 10.0, 11.1, 9.9, 10.5, vol)
    assert bull_bar.is_bull()
    assert not bull_bar.is_bear()
    assert not bull_bar.is_neutral()
    assert bull_bar.direction() is BarDirection.BULL

    bear_bar = Bar(ts, 10.0, 11.1, 9.9, 9.5, vol)
    assert not bear_bar.is_bull()
    assert bear_bar.is_bear()
    assert not bear_bar.is_neutral()
    assert bear_bar.direction() is BarDirection.BEAR

    neutral_bar = Bar(ts, 10.0, 11.1, 9.9, 10.0, vol)
    assert not neutral_bar.is_bull()
    assert not neutral_bar.is_bear()
    assert neutral_bar.is_neutral()
    assert neutral_bar.direction() is BarDirection.NEUTRAL


def test_ranges():
    ts = 123_456_789
    vol = 5000

    bull = Bar(ts, 10.0, 11.1, 9.9, 10.5, vol)
    assert bull.range() == PriceRange(9.9, 11.1)
    assert bull.body() == PriceRange(10.0, 10.5)
    assert bull.lower() == PriceRange(9.9, 10.0)
    assert bull.upper() == PriceRange(10.5, 11.1)

    bear = Bar(ts, 10.0, 11.1, 9.4, 9.5, vol)
    assert bear.range() == PriceRange(9.4, 11.1)
    assert bear.body() == PriceRange(9.5, 10.0)
    assert bear.lower() == PriceRange(9.4, 9.5)
    assert bear.upper() == PriceRange(10.0, 11.1)

    neutral = Bar(ts, 10.0, 11.1, 9.9, 10.0, vol)
    assert neutral.range() == PriceRange(9.9, 11.1)
    assert neutral.body() == PriceRange(10.0, 10.0)
    assert neutral.lower() == PriceRange(9.9, 10.0)
    assert neutral.upper() == PriceRange(10.0, 11.1)


def test_contains():
    bar = Bar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar.contains(10.3)
    assert bar.contains(9.9)
    assert bar.contains(11.1)

    assert not bar.contains(11.11)
    assert not bar.contains(9.89)


def test_display():
    dt = DateTime(2026, 8, 20, 14, 20, 5, tzinfo=UTC)
    ts = int(dt.timestamp()) * 1_000_000_000
    bar = Bar(ts, 10.0, 11.1, 9.9, 10.5, 5000)

    assert (
        str(bar) == "2026-08-20 14:20:05 UTC O=10 H=11.1 L=9.9 C=10.5 V=5000"
    )


def test_python_protocols():
    bar = Bar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)

    assert 10.3 in bar
    assert 9.9 in bar
    assert 11.1 in bar

    assert 11.11 not in bar
    assert 9.89 not in bar

    assert bar == Bar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar != Bar(123_456_789, 10.0, 11.1, 9.9, 10.6, 5000)
    assert bar != None
    assert bar != "foo"
    assert bar != 123
