# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from enum import Enum

from avin import _native


class Exchange(Enum):
    """
    Exchange supported by AVIN.

    Examples
    --------
    >>> exchange = Exchange.MOEX
    >>> exchange.name
    'MOEX'
    >>> str(exchange)
    'MOEX'

    >>> Exchange.from_str("MOEX") is Exchange.MOEX
    True
    >>> Exchange.from_str("BiNaNcE") is Exchange.Binance
    True

    >>> for exchange in Exchange:
    ...     print(exchange)
    Binance
    Bybit
    MOEX
    SPB
    """

    Binance = _native.Exchange.Binance
    Bybit = _native.Exchange.Bybit
    MOEX = _native.Exchange.MOEX
    SPB = _native.Exchange.SPB

    def __str__(self) -> str:
        return self.value.name()

    @classmethod
    def from_str(cls, s: str) -> Exchange:
        """
        Parse an exchange name.

        Parsing is case-insensitive.

        Parameters
        ----------
        s : str
            Exchange name.

        Raises
        ------
        ValueError
            If the exchange name is unknown.
        """
        native = _native.Exchange.from_str(s)
        return cls[native.name()]
