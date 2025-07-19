#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum
from datetime import UTC
from datetime import time as Time


class Exchange(enum.Enum):
    MOEX = 1
    SPB = 2

    @classmethod
    def from_str(cls, string: str) -> Exchange:
        types = {
            "MOEX": Exchange.MOEX,
            "SPB": Exchange.SPB,
        }
        return types[string]

    def morning(self) -> tuple[Time, Time]:
        match self:
            case Exchange.MOEX:
                return (Time(2, 59, tzinfo=UTC), Time(7, 0, tzinfo=UTC))
            case _:
                assert False, "TODO"

    def day(self) -> tuple[Time, Time]:
        match self:
            case Exchange.MOEX:
                return (Time(7, 0, tzinfo=UTC), Time(15, 39, tzinfo=UTC))
            case _:
                assert False, "TODO"

    def evening(self) -> tuple[Time, Time]:
        match self:
            case Exchange.MOEX:
                return (Time(16, 5, tzinfo=UTC), Time(20, 49, tzinfo=UTC))
            case _:
                assert False, "TODO"


if __name__ == "__main__":
    ...
