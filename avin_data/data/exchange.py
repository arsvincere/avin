#!/usr/bin/env  python3
# ============================================================================
# URL:          http://arsvincere.com
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

    @classmethod
    def from_str(cls, string: str) -> Exchange:
        types = {
            "MOEX": Exchange.MOEX,
        }
        return types[string]

    def morning(self) -> [Time, Time]:
        match self:
            case Exchange.MOEX:
                return [Time(2, 59, tzinfo=UTC), Time(7, 0, tzinfo=UTC)]

        assert False, "TODO_ME"

    def day(self) -> [Time, Time]:
        match self:
            case Exchange.MOEX:
                return [Time(7, 0, tzinfo=UTC), Time(15, 39, tzinfo=UTC)]

        assert False, "TODO_ME"

    def evening(self) -> [Time, Time]:
        match self:
            case Exchange.MOEX:
                return [Time(16, 5, tzinfo=UTC), Time(20, 49, tzinfo=UTC)]

        assert False, "TODO_ME"


if __name__ == "__main__":
    ...
