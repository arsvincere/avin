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


def ts(year, month, day, hour, minute, second, microsecond, tz) -> int:
    dt = DateTime(year, month, day, hour, minute, second, microsecond, tz)
    ts = int(dt.timestamp()) * 1_000_000_000 + microsecond * 1_000
    return ts


def test_timeframe_key():
    assert TimeFrame.M5.key == "5m"
    assert TimeFrame.DAY.key == "d"


def test_timeframe_duration():
    tf = TimeFrame.M5

    assert tf.nanos() == 300_000_000_000
    assert tf.seconds() == 300
    assert tf.timedelta() == TimeDelta(minutes=5)

    assert TimeFrame.MONTH.nanos() is None
    assert TimeFrame.MONTH.seconds() is None
    assert TimeFrame.MONTH.timedelta() is None


def test_timeframe_bounds():
    tf = TimeFrame.M5
    current = ts(2026, 8, 21, 12, 51, 45, 123456, UTC)

    expected_begin = ts(2026, 8, 21, 12, 50, 0, 0, UTC)
    expected_end = ts(2026, 8, 21, 12, 55, 0, 0, UTC)

    begin = tf.begin_frame_ts(current)
    end = tf.end_frame_ts(current)

    assert begin == expected_begin
    assert end == expected_end


def test_timeframe_all():
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


def test_timeframe_str():
    assert str(TimeFrame.M5) == "5M"
    assert str(TimeFrame.DAY) == "D"


def test_timeframe_from_str():
    assert TimeFrame.from_str("15M") is TimeFrame.M15
    assert TimeFrame.from_str("d") is TimeFrame.DAY

    with pytest.raises(ValueError):
        TimeFrame.from_str("Week")

    with pytest.raises(ValueError):
        TimeFrame.from_str("foo")
