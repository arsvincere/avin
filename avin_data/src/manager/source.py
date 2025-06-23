#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum


class Source(enum.Enum):
    MOEX = 1
    TINKOFF = 2

    @classmethod
    def from_str(cls, string: str) -> Source:
        sources = {
            "MOEX": Source.MOEX,
            "TINKOFF": Source.TINKOFF,
        }
        return sources[string.upper()]


if __name__ == "__main__":
    ...
