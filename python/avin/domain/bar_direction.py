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
    BULL
        The bar closes above its opening price.
    NEUTRAL
        The bar closes at its opening price.
    BEAR
        The bar closes below its opening price.

    Examples
    --------
    >>> direction = BarDirection.BULL
    >>> direction.name
    'BULL'
    >>> direction.value
    1
    >>> str(direction)
    'Bull'
    """

    BULL = PyBarDirection.Bull
    NEUTRAL = PyBarDirection.Neutral
    BEAR = PyBarDirection.Bear

    _inner: PyBarDirection

    def __new__(cls, inner: PyBarDirection):
        obj = object.__new__(cls)
        obj._value_ = inner.value()
        obj._inner = inner

        return obj

    def __str__(self) -> str:
        return self._inner.display()

    @classmethod
    def _from_native(cls, inner: PyBarDirection) -> BarDirection:
        for direction in cls:
            if direction._inner.eq(inner):
                return direction

        raise RuntimeError(
            "native bar direction is missing from BarDirection"
        )
