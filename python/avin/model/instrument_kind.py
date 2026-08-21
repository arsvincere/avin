# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from enum import Enum

from avin._native import PyInstrumentKind


class InstrumentKind(Enum):
    """
    Financial instrument kind.

    Examples
    --------
    >>> for kind in InstrumentKind:
    ...     print(kind)
    Currency
    Index
    Stock
    Future
    Bond
    Option
    ETF
    >>> kind = InstrumentKind.FUTURE
    >>> str(kind)
    'Future'
    >>> InstrumentKind.from_str("bond") == InstrumentKind.STOCK
    True
    >>> InstrumentKind.from_str("stock") is InstrumentKind.STOCK
    True
    """

    CURRENCY = PyInstrumentKind.Currency
    INDEX = PyInstrumentKind.Index
    STOCK = PyInstrumentKind.Stock
    FUTURE = PyInstrumentKind.Future
    BOND = PyInstrumentKind.Bond
    OPTION = PyInstrumentKind.Option
    ETF = PyInstrumentKind.ETF

    _inner: PyInstrumentKind

    def __new__(cls, inner: PyInstrumentKind):
        obj = object.__new__(cls)
        obj._value_ = inner.display()
        obj._inner = inner

        return obj

    def __str__(self) -> str:
        return self._inner.display()

    @classmethod
    def from_str(cls, s: str) -> InstrumentKind:
        """
        Parse an instrument kind.

        Parsing is case-insensitive.

        Parameters
        ----------
        s : str
            Instrument kind.

        Raises
        ------
        ValueError
            If the instrument kind is unknown.
        RuntimeError
            If the native and public Python instrument kind definitions are
            out of sync.

        Examples
        --------
        >>> InstrumentKind.from_str("FuTuRe") is InstrumentKind.FUTURE
        True
        >>> InstrumentKind.from_str("ETF") is InstrumentKind.ETF
        True
        """
        return InstrumentKind._from_native(PyInstrumentKind.from_str(s))

    @classmethod
    def _from_native(cls, inner: PyInstrumentKind) -> InstrumentKind:
        for kind in cls:
            if kind._inner.eq(inner):
                return kind

        raise RuntimeError(
            "native instrument kind is missing from InstrumentKind"
        )
