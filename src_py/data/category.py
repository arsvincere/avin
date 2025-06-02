#!/usr/bin/env  python3
# ============================================================================
# URL:          http://arsvincere.com
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum

from src_py.utils import log


class Category(enum.Enum):
    CASH = 1
    INDEX = 2
    SHARE = 3
    BOND = 4
    FUTURE = 5
    OPTION = 6
    CURRENCY = 7
    ETF = 8

    @classmethod
    def from_str(cls, string: str) -> Category:
        categories = {
            "CASH": Category.CASH,
            "INDEX": Category.INDEX,
            "SHARE": Category.SHARE,
            "BOND": Category.BOND,
            "FUTURE": Category.FUTURE,
            "OPTION": Category.OPTION,
            "CURRENCY": Category.CURRENCY,
            "ETF": Category.ETF,
        }

        category = categories.get(string)

        if category is None:
            log.error(f"Invalid category: {string}")
            exit(1)

        return category


if __name__ == "__main__":
    ...
