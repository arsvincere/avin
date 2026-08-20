# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from enum import Enum

from avin._native import PyBarDirection


class BarDirection(Enum):
    """
    Bar direction.

    Indicates whether a bar is bullish, neutral, or bearish.

    Attributes
    ----------
    Bull
        The bar closes above its opening price.
    Neutral
        The bar closes at its opening price.
    Bear
        The bar closes below its opening price.

    Examples
    --------
    >>> direction = BarDirection.Bull
    >>> direction.name
    'Bull'
    >>> direction.value
    1
    >>> str(direction)
    'Bull'
    """

    # TODO: сделать ли значения PyBarDirection.Variant как в Exchange?
    Bull = PyBarDirection.Bull.value()
    Neutral = PyBarDirection.Neutral.value()
    Bear = PyBarDirection.Bear.value()

    def __str__(self) -> str:
        return self.name

    @classmethod
    def _from_native(cls, inner: PyBarDirection) -> BarDirection:
        return cls(inner.value())
