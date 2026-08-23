# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PyInstrumentId
from avin.domain.category import Category
from avin.domain.exchange import Exchange
from avin.domain.ticker import Ticker


class InstrumentId:
    """
    Canonical instrument identifier used by AVIN.

    An instrument ID combines an exchange, category, and ticker into
    a compact, human-readable form such as ``MOEX.SHARE.SBER``.

    Parameters
    ----------
    exchange : Exchange
        Instrument exchange.
    category : Category
        Category.
    ticker : Ticker
        Instrument ticker.

    Examples
    --------
    >>> iid = InstrumentId(
    ...     Exchange.MOEX,
    ...     Category.SHARE,
    ...     Ticker("SBER"),
    ... )
    >>> str(iid)
    'MOEX.SHARE.SBER'

    >>> iid = InstrumentId.from_str("moex.SHARE.SBER")
    >>> iid.exchange is Exchange.MOEX
    True
    >>> iid.category is Category.SHARE
    True
    >>> str(iid.ticker)
    'SBER'
    """

    __slots__ = ("_inner",)

    def __init__(
        self,
        exchange: Exchange,
        category: Category,
        ticker: Ticker,
    ) -> None:
        self._inner = PyInstrumentId(
            exchange._inner,
            category._inner,
            ticker._inner,
        )

    def __str__(self) -> str:
        return self._inner.display()

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, InstrumentId):
            return NotImplemented

        return self._inner.eq(other._inner)

    @property
    def exchange(self) -> Exchange:
        """
        Instrument exchange.
        """
        return Exchange._from_native(self._inner.exchange())

    @property
    def category(self) -> Category:
        """
        Category.
        """
        return Category._from_native(self._inner.category())

    @property
    def ticker(self) -> Ticker:
        """
        Instrument ticker.
        """
        return Ticker._from_native(self._inner.ticker())

    @classmethod
    def from_str(cls, s: str) -> InstrumentId:
        """
        Parse an instrument ID.

        Parameters
        ----------
        s : str
            Instrument ID in ``EXCHANGE.CATEGORY.TICKER`` format.

        Raises
        ------
        ValueError
            If the instrument ID is invalid.

        Examples
        --------
        >>> iid = InstrumentId.from_str("MOEX.SHARE.SBER")
        >>> iid.exchange is Exchange.MOEX
        True
        >>> iid.category is Category.SHARE
        True
        >>> str(iid.ticker)
        'SBER'
        """
        return cls._from_native(PyInstrumentId.from_str(s))

    @classmethod
    def _from_native(cls, inner: PyInstrumentId) -> InstrumentId:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
