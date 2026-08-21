# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import UTC
from datetime import datetime as DateTime
from datetime import timedelta as TimeDelta

import pytest

from avin import TimeFrame


# helper - returns timestamp nanos of datetime
def ts(
    year: int,
    month: int,
    day: int,
    hour: int,
    minute: int,
    second: int,
    nanos: int,
) -> int:
    dt = DateTime(year, month, day, hour, minute, second, tzinfo=UTC)

    return int(dt.timestamp()) * 1_000_000_000 + nanos


def test_all():
    expected = [
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

    assert list(TimeFrame) == expected


def test_duration():
    cases = [
        (TimeFrame.S1, 1),
        (TimeFrame.S5, 5),
        (TimeFrame.S10, 10),
        (TimeFrame.S15, 15),
        (TimeFrame.M1, 60),
        (TimeFrame.M5, 5 * 60),
        (TimeFrame.M10, 10 * 60),
        (TimeFrame.M15, 15 * 60),
        (TimeFrame.H1, 60 * 60),
        (TimeFrame.H4, 4 * 60 * 60),
        (TimeFrame.DAY, 24 * 60 * 60),
        (TimeFrame.WEEK, 7 * 24 * 60 * 60),
    ]

    for timeframe, seconds in cases:
        assert timeframe.seconds() == seconds
        assert timeframe.nanos() == seconds * 1_000_000_000
        assert timeframe.timedelta() == TimeDelta(seconds=seconds)

    assert TimeFrame.MONTH.nanos() is None
    assert TimeFrame.MONTH.seconds() is None
    assert TimeFrame.MONTH.timedelta() is None


def test_begin_frame_ts():
    input_ts = ts(2023, 8, 2, 10, 13, 42, 123_456_789)

    cases = [
        (TimeFrame.S1, ts(2023, 8, 2, 10, 13, 42, 0)),
        (TimeFrame.S5, ts(2023, 8, 2, 10, 13, 40, 0)),
        (TimeFrame.S10, ts(2023, 8, 2, 10, 13, 40, 0)),
        (TimeFrame.S15, ts(2023, 8, 2, 10, 13, 30, 0)),
        (TimeFrame.M1, ts(2023, 8, 2, 10, 13, 0, 0)),
        (TimeFrame.M5, ts(2023, 8, 2, 10, 10, 0, 0)),
        (TimeFrame.M10, ts(2023, 8, 2, 10, 10, 0, 0)),
        (TimeFrame.M15, ts(2023, 8, 2, 10, 0, 0, 0)),
        (TimeFrame.H1, ts(2023, 8, 2, 10, 0, 0, 0)),
        (TimeFrame.H4, ts(2023, 8, 2, 8, 0, 0, 0)),
        (TimeFrame.DAY, ts(2023, 8, 2, 0, 0, 0, 0)),
        (TimeFrame.WEEK, ts(2023, 7, 31, 0, 0, 0, 0)),
        (TimeFrame.MONTH, ts(2023, 8, 1, 0, 0, 0, 0)),
    ]

    for timeframe, expected in cases:
        assert timeframe.begin_frame_ts(input_ts) == expected


def test_end_frame_ts():
    input_ts = ts(2023, 8, 2, 10, 13, 42, 123_456_789)

    cases = [
        (TimeFrame.S1, ts(2023, 8, 2, 10, 13, 43, 0)),
        (TimeFrame.S5, ts(2023, 8, 2, 10, 13, 45, 0)),
        (TimeFrame.S10, ts(2023, 8, 2, 10, 13, 50, 0)),
        (TimeFrame.S15, ts(2023, 8, 2, 10, 13, 45, 0)),
        (TimeFrame.M1, ts(2023, 8, 2, 10, 14, 0, 0)),
        (TimeFrame.M5, ts(2023, 8, 2, 10, 15, 0, 0)),
        (TimeFrame.M10, ts(2023, 8, 2, 10, 20, 0, 0)),
        (TimeFrame.M15, ts(2023, 8, 2, 10, 15, 0, 0)),
        (TimeFrame.H1, ts(2023, 8, 2, 11, 0, 0, 0)),
        (TimeFrame.H4, ts(2023, 8, 2, 12, 0, 0, 0)),
        (TimeFrame.DAY, ts(2023, 8, 3, 0, 0, 0, 0)),
        (TimeFrame.WEEK, ts(2023, 8, 7, 0, 0, 0, 0)),
        (TimeFrame.MONTH, ts(2023, 9, 1, 0, 0, 0, 0)),
    ]

    for timeframe, expected in cases:
        assert timeframe.end_frame_ts(input_ts) == expected


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
    for timeframe in TimeFrame:
        canonical = str(timeframe)

        assert TimeFrame.from_str(canonical) is timeframe
        assert TimeFrame.from_str(canonical.lower()) is timeframe

    with pytest.raises(ValueError):
        TimeFrame.from_str("M1")

    with pytest.raises(ValueError):
        TimeFrame.from_str("Day")

    with pytest.raises(ValueError):
        TimeFrame.from_str("foo")
