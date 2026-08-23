# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from avin._native import PyInstrumentInfo
from avin.domain.category import Category
from avin.domain.exchange import Exchange
from avin.domain.instrument_id import InstrumentId
from avin.domain.symbol import Symbol


class InstrumentInfo:
    """
    Instrument reference data.

    Represents a locally stored instrument description used for instrument
    lookup, asset creation, and offline market research.

    `InstrumentInfo` is not intended to be instantiated directly. Instances are
    created by AVIN as part of concrete instrument objects such as futures,
    shares, bonds, and options.

    Reference data may be slightly outdated and must not be treated as
    authoritative for live trading validation.
    """

    __slots__ = ("_inner",)
    _inner: PyInstrumentInfo

    def __init__(self, *args: object, **kwargs: object) -> None:
        raise TypeError("InstrumentInfo cannot be instantiated directly")

    @property
    def iid(self) -> InstrumentId:
        """
        Canonical instrument ID.
        """
        return InstrumentId._from_native(self._inner.iid())

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
    def symbol(self) -> Symbol:
        """
        Instrument symbol.
        """
        return Symbol._from_native(self._inner.symbol())

    @property
    def figi(self) -> str:
        """
        FIGI - Financial Instrument Global Identifier.
        """
        return self._inner.figi()

    @property
    def name(self) -> str:
        """
        Instrument name.
        """
        return self._inner.name()

    @property
    def lot(self) -> int:
        """
        Lot size.
        """
        return self._inner.lot()

    @property
    def step(self) -> float:
        """
        Minimum price step.
        """
        return self._inner.step()

    @classmethod
    def _from_native(cls, inner: PyInstrumentInfo) -> InstrumentInfo:
        obj = object.__new__(cls)
        obj._inner = inner

        return obj
