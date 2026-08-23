# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from datetime import datetime as DateTime

from avin._native import PyBar
from avin.domain.bar_direction import BarDirection
from avin.domain.price_range import PriceRange


class Bar:
    """
    OHLCV market bar.

    The timestamp identifies the start of the bar.

    Bar values are assumed to contain valid market data and are not
    validated during construction.

    Parameters
    ----------
    ts : int
        Bar start timestamp in Unix nanoseconds.
    o : float
        Open price.
    h : float
        High price.
    l : float
        Low price.
    c : float
        Close price.
    v : int
        Volume.

    Attributes
    ----------
    ts : int
        Bar start timestamp in Unix nanoseconds.
    o : float
        Open price.
    h : float
        High price.
    l : float
        Low price.
    c : float
        Close price.
    v : int
        Volume.

    Examples
    --------
    >>> bar = Bar(
    ...     1787235600000000000,
    ...     100.1,
    ...     105.5,
    ...     98.8,
    ...     103.3,
    ...     5000,
    ... )
    >>> bar.o
    100.1
    >>> bar.c
    103.3
    >>> bar.is_bull()
    True
    >>> print(bar.dt())
    2026-08-20 14:20:00+00:00
    >>> 101.0 in bar
    True
    >>> r = bar.range()
    >>> print(r)
    [98.8, 105.5]
    """

    __slots__ = ("_inner",)

    def __init__(
        self, ts: int, o: float, h: float, l: float, c: float, v: int
    ) -> None:
        self._inner = PyBar(ts, o, h, l, c, v)

    def __str__(self) -> str:
        return self._inner.display()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Bar):
            return NotImplemented

        return self._inner.eq(other._inner)

    def __contains__(self, price: float) -> bool:
        return self._inner.contains(price)

    @property
    def ts(self) -> int:
        """
        Bar start timestamp in Unix nanoseconds.
        """
        return self._inner.ts()

    @property
    def o(self) -> float:
        """
        Open price.
        """
        return self._inner.o()

    @property
    def h(self) -> float:
        """
        High price.
        """
        return self._inner.h()

    @property
    def l(self) -> float:
        """
        Low price.
        """
        return self._inner.l()

    @property
    def c(self) -> float:
        """
        Close price.
        """
        return self._inner.c()

    @property
    def v(self) -> int:
        """
        Volume.
        """
        return self._inner.v()

    def dt(self) -> DateTime:
        """
        Return the bar start timestamp as a UTC datetime.
        """
        return self._inner.dt()

    def direction(self) -> BarDirection:
        """
        Return the bar direction.
        """
        return BarDirection._from_native(self._inner.direction())

    def is_bear(self) -> bool:
        """
        Return whether the bar is bearish.
        """
        return self._inner.is_bear()

    def is_bull(self) -> bool:
        """
        Return whether the bar is bullish.
        """
        return self._inner.is_bull()

    def is_neutral(self) -> bool:
        """
        Return whether the bar is neutral (open == close).
        """
        return self._inner.is_neutral()

    def range(self) -> PriceRange:
        """
        Return the full price range of the bar, `[L, H]`.
        """
        return PriceRange._from_native(self._inner.range())

    def body(self) -> PriceRange:
        """
        Return the body price range, `[min(O, C), max(O, C)]`.
        """
        return PriceRange._from_native(self._inner.body())

    def lower(self) -> PriceRange:
        """
        Return the lower wick price range, `[L, min(O, C)]`.
        """
        return PriceRange._from_native(self._inner.lower())

    def upper(self) -> PriceRange:
        """
        Return the upper wick price range, `[max(O, C), H]`.
        """
        return PriceRange._from_native(self._inner.upper())

    def contains(self, price: float) -> bool:
        """
        Check whether a price is within the closed bar range `[L, H]`.
        """
        return self._inner.contains(price)

    @classmethod
    def _from_native(cls, native: PyBar) -> Bar:
        obj = cls.__new__(cls)
        obj._inner = native

        return obj
