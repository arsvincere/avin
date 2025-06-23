#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum

from src.utils import log


class Category(enum.Enum):
    CURRENCY = 1
    INDEX = 2
    SHARE = 3
    BOND = 4
    FUTURE = 5
    OPTION = 6
    ETF = 7

    @classmethod
    def from_str(cls, string: str) -> Category:
        categories = {
            "CURRENCY": Category.CURRENCY,
            "INDEX": Category.INDEX,
            "SHARE": Category.SHARE,
            "BOND": Category.BOND,
            "FUTURE": Category.FUTURE,
            "OPTION": Category.OPTION,
            "ETF": Category.ETF,
        }

        category = categories.get(string)

        if category is None:
            log.error(f"Invalid category: {string}")
            exit(1)

        return category


if __name__ == "__main__":
    ...
