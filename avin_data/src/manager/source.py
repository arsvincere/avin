#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

from __future__ import annotations

import enum

from src.exceptions import NotImplemetedSource


class Source(enum.Enum):
    """Market data source."""

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
            NotImplemetedSource if category not exists.
        """
        if attr := getattr(cls, string.upper(), None):
            return attr
        raise NotImplemetedSource(
            f"Source not implemented. Choice from {cls.__members__.keys()}"
        )


if __name__ == "__main__":
    ...
