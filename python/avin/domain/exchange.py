# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from enum import Enum

from avin._native import PyExchange


class Exchange(Enum):
    """
    Exchange supported by AVIN.

    Examples
    --------
    >>> for exchange in Exchange:
    ...     print(exchange)
    Binance
    Bybit
    MOEX
    SPB
    >>> exchange = Exchange.MOEX
    >>> str(exchange)
    'MOEX'
    >>> Exchange.from_str("MOEX") == Exchange.MOEX
    True
    >>> Exchange.from_str("BiNaNcE") is Exchange.BINANCE
    True
    """

    BINANCE = PyExchange.Binance
    BYBIT = PyExchange.Bybit
    MOEX = PyExchange.Moex
    SPB = PyExchange.Spb

    _inner: PyExchange

    def __new__(cls, inner: PyExchange):
        obj = object.__new__(cls)
        obj._value_ = inner.key()
        obj._inner = inner

        return obj

    def __str__(self) -> str:
        return self._inner.display()

    @property
    def key(self) -> str:
        """
        Return the stable machine-readable exchange key.

        The key is intended for persistence, configuration, and serialization.
        """
        return self._inner.key()

    @classmethod
    def from_str(cls, s: str) -> Exchange:
        """
        Parse an exchange key.

        Parsing is case-insensitive.

        Parameters
        ----------
        s : str
            Exchange key.

        Raises
        ------
        ValueError
            If the exchange key is unknown.
        """
        return Exchange._from_native(PyExchange.from_str(s))

    @classmethod
    def _from_native(cls, inner: PyExchange) -> Exchange:
        for exchange in cls:
            if exchange._inner.eq(inner):
                return exchange

        raise RuntimeError("native exchange is missing from Exchange")
