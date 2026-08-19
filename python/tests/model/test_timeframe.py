# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import timedelta as TimeDelta

import pytest

from avin import TimeFrame


# helper - returns timestamp nanos of datetime
def ts(
    year: int,
    month: int,
    day: int,
    hour: int = 0,
    minute: int = 0,
    second: int = 0,
) -> int:
    from datetime import UTC, datetime

    dt = datetime(year, month, day, hour, minute, second, tzinfo=UTC)
    return int(dt.timestamp()) * 1_000_000_000


def test_members():
    assert list(TimeFrame) == [
        TimeFrame.S1,
        TimeFrame.S5,
        TimeFrame.S10,
        TimeFrame.S15,
        TimeFrame.M1,
        TimeFrame.M5,
        TimeFrame.M10,
        TimeFrame.M15,
        TimeFrame.H1,
        TimeFrame.H4,
        TimeFrame.DAY,
        TimeFrame.WEEK,
        TimeFrame.MONTH,
    ]


def test_str():
    cases = [
        (TimeFrame.S1, "1S"),
        (TimeFrame.S5, "5S"),
        (TimeFrame.S10, "10S"),
        (TimeFrame.S15, "15S"),
        (TimeFrame.M1, "1M"),
        (TimeFrame.M5, "5M"),
        (TimeFrame.M10, "10M"),
        (TimeFrame.M15, "15M"),
        (TimeFrame.H1, "1H"),
        (TimeFrame.H4, "4H"),
        (TimeFrame.DAY, "D"),
        (TimeFrame.WEEK, "W"),
        (TimeFrame.MONTH, "M"),
    ]

    for timeframe, expected in cases:
        assert str(timeframe) == expected


def test_from_str():
    assert TimeFrame.from_str("1M") is TimeFrame.M1
    assert TimeFrame.from_str("5m") is TimeFrame.M5
    assert TimeFrame.from_str("4H") is TimeFrame.H4
    assert TimeFrame.from_str("d") is TimeFrame.DAY
    assert TimeFrame.from_str("W") is TimeFrame.WEEK
    assert TimeFrame.from_str("m") is TimeFrame.MONTH

    with pytest.raises(ValueError):
        TimeFrame.from_str("M1")

    with pytest.raises(ValueError):
        TimeFrame.from_str("foo")


def test_duration():
    assert TimeFrame.M15.nanos() == 900_000_000_000
    assert TimeFrame.M15.seconds() == 900
    assert TimeFrame.M15.timedelta() == TimeDelta(minutes=15)

    assert TimeFrame.MONTH.nanos() is None
    assert TimeFrame.MONTH.seconds() is None
    assert TimeFrame.MONTH.timedelta() is None


def test_frame_boundaries():
    value = ts(2026, 8, 18, 10, 13, 42)

    assert TimeFrame.M5.begin_frame_ts(value) == ts(2026, 8, 18, 10, 10)
    assert TimeFrame.M5.end_frame_ts(value) == ts(2026, 8, 18, 10, 15)

    assert TimeFrame.MONTH.begin_frame_ts(value) == ts(2026, 8, 1)
    assert TimeFrame.MONTH.end_frame_ts(value) == ts(2026, 9, 1)
