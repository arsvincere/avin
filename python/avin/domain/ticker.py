# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PyTicker


class Ticker:
    """
    Trading instrument ticker.

    A ticker must be non-empty and must not contain whitespace.

    Parameters
    ----------
    value : str
        Instrument ticker.

    Raises
    ------
    ValueError
        If the ticker is empty or contains whitespace.

    Examples
    --------
    >>> ticker = Ticker("SBER")
    >>> str(ticker)
    'SBER'

    >>> Ticker("SiU6") == Ticker("SiU6")
    True
    """

    __slots__ = ("_inner",)

    def __init__(self, value: str) -> None:
        self._inner = PyTicker(value)

    def __str__(self) -> str:
        return self._inner.display()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Ticker):
            return NotImplemented

        return self._inner.eq(other._inner)

    @classmethod
    def _from_native(cls, inner: PyTicker) -> Ticker:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
