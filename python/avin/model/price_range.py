from __future__ import annotations

from avin import _native


class PriceRange:
    """
    Closed price interval [low, high].

    Represents a price range including both boundary values.

    Parameters
    ----------
    low
        Lower bound of the range.
    high
        Upper bound of the range.

    Raises
    ------
    ValueError
        If either bound is non-finite or if `low > high`.
    """

    __slots__ = ("_inner",)

    def __init__(self, low: float, high: float) -> None:
        self._inner = _native.PriceRange(low, high)

    def __str__(self) -> str:
        return str(self._inner)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, PriceRange):
            return NotImplemented

        return self._inner == other._inner

    def __contains__(self, value: float) -> bool:
        return self._inner.contains(value)

    def __repr__(self) -> str:
        return f"PriceRange({self.low!r}, {self.high!r})"

    @property
    def low(self) -> float:
        """
        Return the lower bound of the range.
        """
        return self._inner.low()

    @property
    def high(self) -> float:
        """
        Return the upper bound of the range.
        """
        return self._inner.high()

    def contains(self, value: float) -> bool:
        """
        Check whether the given value is within the range.

        Both boundary values are included.
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
