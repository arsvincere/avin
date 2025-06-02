#!/usr/bin/env  python3
# ============================================================================
# URL:          http://arsvincere.com
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum


class Source(enum.Enum):
    CONVERT = 0
    MOEX = 1
    TINKOFF = 2

    @classmethod
    def from_str(cls, string: str) -> Source:
        sources = {
            "CONVERT": Source.CONVERT,
            "MOEX": Source.MOEX,
            "TINKOFF": Source.TINKOFF,
        }
        return sources[string]


if __name__ == "__main__":
    ...
