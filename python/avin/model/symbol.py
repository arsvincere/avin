# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PySymbol


class Symbol:
    """
    Trading instrument symbol.

    A symbol must be non-empty and must not contain whitespace.

    Parameters
    ----------
    value : str
        Instrument symbol.

    Raises
    ------
    ValueError
        If the symbol is empty or contains whitespace.

    Examples
    --------
    >>> symbol = Symbol("SBER")
    >>> str(symbol)
    'SBER'

    >>> Symbol("SiU6") == Symbol("SiU6")
    True
    """

    __slots__ = ("_inner",)

    def __init__(self, value: str) -> None:
        self._inner = PySymbol(value)

    def __str__(self) -> str:
        return self._inner.display()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Symbol):
            return NotImplemented

        return self._inner.eq(other._inner)

    @classmethod
    def _from_native(cls, inner: PySymbol) -> Symbol:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
