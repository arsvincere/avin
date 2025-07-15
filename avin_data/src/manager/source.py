#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum

from src.exceptions import NotImplemetedCategory


class Source(enum.Enum):
    """Stock exchange."""
    MOEX = 1
    TINKOFF = 2

    @classmethod
    def from_str(cls, string: str) -> Source:
        """Get enum from str.

        Args:
            string: category name.

        Returns:
            Category Enum.

        Raises:
            NotImplemetedCategory if category not exists.
        """
        if attr := getattr(cls, string.upper(), None):
            return attr
        raise NotImplemetedCategory(f"Category not implemented. Choice from {cls.__members__.keys()}")


if __name__ == "__main__":
    ...
