#!/usr/bin/env  python3
# ============================================================================
# URL:          http://arsvincere.com
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

import enum
import os
from datetime import UTC
from datetime import time as Time
from datetime import timedelta as TimeDelta

__all__ = (
    "WeekDays",
    "ONE_SECOND",
    "ONE_MINUTE",
    "FIVE_MINUTE",
    "TEN_MINUTE",
    "ONE_HOUR",
    "ONE_DAY",
    "ONE_WEEK",
    "ONE_MONTH",
    "ONE_YEAR",
    "DAY_BEGIN",
    "DAY_END",
)


class WeekDays(enum.Enum):  # {{{
    Mon = 0
    Tue = 1
    Wed = 2
    Thu = 3
    Fri = 4
    Sat = 5
    Sun = 6

    @staticmethod
    def is_workday(day_number: int):
        return day_number < 5

    @staticmethod
    def is_holiday(day_number):
        return day_number in (5, 6)


ONE_SECOND = TimeDelta(seconds=1)
ONE_MINUTE = TimeDelta(minutes=1)
FIVE_MINUTE = TimeDelta(minutes=5)
TEN_MINUTE = TimeDelta(minutes=10)
ONE_HOUR = TimeDelta(hours=1)
ONE_DAY = TimeDelta(days=1)
ONE_WEEK = TimeDelta(weeks=1)
ONE_MONTH = TimeDelta(days=30)
ONE_YEAR = TimeDelta(days=365)
DAY_BEGIN = Time(0, 0, tzinfo=UTC)
DAY_END = Time(23, 59, tzinfo=UTC)
