# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import UTC
from datetime import datetime as DateTime

from avin import Bar, BarDirection, PriceRange


def test_bar():
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

    assert (
        str(bar) == "2026-08-20 14:20:05 UTC O=10 H=11.1 L=9.9 C=10.5 V=5000"
    )


def test_bar_direction():
    bull = Bar(0, 10.0, 11.0, 9.0, 10.5, 100)
    bear = Bar(0, 10.0, 11.0, 9.0, 9.5, 100)
    neutral = Bar(0, 10.0, 11.0, 9.0, 10.0, 100)

    assert bull.is_bull()
    assert bull.direction() == BarDirection.BULL

    assert bear.is_bear()
    assert bear.direction() == BarDirection.BEAR

    assert neutral.is_neutral()
    assert neutral.direction() == BarDirection.NEUTRAL


def test_bar_ranges():
    bar = Bar(0, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar.range() == PriceRange(9.9, 11.1)
    assert bar.body() == PriceRange(10.0, 10.5)
    assert bar.lower() == PriceRange(9.9, 10.0)
    assert bar.upper() == PriceRange(10.5, 11.1)


def test_bar_contains():
    bar = Bar(0, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar.contains(10.3)

    assert 9.9 in bar
    assert 11.1 in bar
    assert 11.2 not in bar


def test_bar_eq():
    bar = Bar(0, 10.0, 11.1, 9.9, 10.5, 5000)

    assert bar == Bar(0, 10.0, 11.1, 9.9, 10.5, 5000)
    assert bar != Bar(0, 10.0, 11.1, 9.9, 10.6, 5000)
    assert bar != "foo"
