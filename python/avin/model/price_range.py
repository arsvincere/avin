# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PyPriceRange


class PriceRange:
    """
    Closed price range [low, high].

    Represents a price range that includes both boundary prices.

    Parameters
    ----------
    low : float
        Lower price of the range.
    high : float
        Upper price of the range.

    Raises
    ------
    ValueError
        If either price is NaN or infinite, or if `low > high`.

    Examples
    --------
    >>> price_range = PriceRange(100.0, 105.0)
    >>> price_range.low
    100.0
    >>> price_range.high
    105.0
    >>> 103.0 in price_range
    True
    >>> 105.1 in price_range
    False
    >>> price_range.middle()
    102.5
    >>> price_range.width()
    5.0
    """

    __slots__ = ("_inner",)

    def __init__(self, low: float, high: float) -> None:
        self._inner = PyPriceRange(low, high)

    def __str__(self) -> str:
        return self._inner.str()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, PriceRange):
            return NotImplemented

        return self._inner.eq(other._inner)

    def __contains__(self, value: float) -> bool:
        return self._inner.contains(value)

    def __repr__(self) -> str:
        return f"PriceRange({self.low!r}, {self.high!r})"

    @property
    def low(self) -> float:
        """
        Lower price of the range.
        """
        return self._inner.low()

    @property
    def high(self) -> float:
        """
        Upper price of the range.
        """
        return self._inner.high()

    def contains(self, value: float) -> bool:
        """
        Check whether a price is within the range.
        """
        return self._inner.contains(value)

    def middle(self) -> float:
        """
        Return the midpoint of the range.
        """
        return self._inner.middle()

    def width(self) -> float:
        """
        Return the width of the range.
        """
        return self._inner.width()
