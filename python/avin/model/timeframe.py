# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from datetime import timedelta as TimeDelta
from enum import Enum

from avin._native import PyTimeFrame


class TimeFrame(Enum):
    S1 = PyTimeFrame.S1
    S5 = PyTimeFrame.S5
    S10 = PyTimeFrame.S10
    S15 = PyTimeFrame.S15

    M1 = PyTimeFrame.M1
    M5 = PyTimeFrame.M5
    M10 = PyTimeFrame.M10
    M15 = PyTimeFrame.M15

    H1 = PyTimeFrame.H1
    H4 = PyTimeFrame.H4

    DAY = PyTimeFrame.Day
    WEEK = PyTimeFrame.Week
    MONTH = PyTimeFrame.Month

    def __str__(self) -> str:
        return self.value.str()

    @classmethod
    def from_str(cls, s: str) -> TimeFrame:
        native = PyTimeFrame.from_str(s)

        for tf in cls:
            if native.eq(tf.value):
                return tf

        # Reached only if the native and public Python timeframe definitions
        # are out of sync, e.g. a new native variant is missing from this Enum.
        raise RuntimeError("native timeframe is missing from TimeFrame")

    def nanos(self) -> int | None:
        return self.value.nanos()

    def seconds(self) -> int | None:
        return self.value.seconds()

    def timedelta(self) -> TimeDelta | None:
        return self.value.timedelta()

    def begin_frame_ts(self, ts: int) -> int:
        return self.value.begin_frame_ts(ts)

    def end_frame_ts(self, ts: int) -> int:
        return self.value.end_frame_ts(ts)
