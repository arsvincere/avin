# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PyInstrumentId
from avin.domain.exchange import Exchange
from avin.domain.instrument_kind import InstrumentKind
from avin.domain.symbol import Symbol


class InstrumentId:
    """
    Canonical instrument identifier used by AVIN.

    An instrument ID combines an exchange, instrument kind, and symbol into
    a compact, human-readable form such as ``MOEX.Stock.SBER``.

    Parameters
    ----------
    exchange : Exchange
        Instrument exchange.
    kind : InstrumentKind
        Instrument kind.
    symbol : Symbol
        Instrument symbol.

    Examples
    --------
    >>> iid = InstrumentId(
    ...     Exchange.MOEX,
    ...     InstrumentKind.STOCK,
    ...     Symbol("SBER"),
    ... )
    >>> str(iid)
    'MOEX.Stock.SBER'

    >>> iid = InstrumentId.from_str("moex.stock.SBER")
    >>> iid.exchange is Exchange.MOEX
    True
    >>> iid.kind is InstrumentKind.STOCK
    True
    >>> str(iid.symbol)
    'SBER'
    """

    __slots__ = ("_inner",)

    def __init__(
        self,
        exchange: Exchange,
        kind: InstrumentKind,
        symbol: Symbol,
    ) -> None:
        self._inner = PyInstrumentId(
            exchange._inner,
            kind._inner,
            symbol._inner,
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
    def kind(self) -> InstrumentKind:
        """
        Instrument kind.
        """
        return InstrumentKind._from_native(self._inner.kind())

    @property
    def symbol(self) -> Symbol:
        """
        Instrument symbol.
        """
        return Symbol._from_native(self._inner.symbol())

    @classmethod
    def from_str(cls, s: str) -> InstrumentId:
        """
        Parse an instrument ID.

        Parameters
        ----------
        s : str
            Instrument ID in ``EXCHANGE.KIND.SYMBOL`` format.

        Raises
        ------
        ValueError
            If the instrument ID is invalid.

        Examples
        --------
        >>> iid = InstrumentId.from_str("MOEX.Stock.SBER")
        >>> iid.exchange is Exchange.MOEX
        True
        >>> iid.kind is InstrumentKind.STOCK
        True
        >>> str(iid.symbol)
        'SBER'
        """
        return cls._from_native(PyInstrumentId.from_str(s))

    @classmethod
    def _from_native(cls, inner: PyInstrumentId) -> InstrumentId:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
