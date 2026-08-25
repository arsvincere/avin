# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from collections.abc import Iterator

from avin._native import PyInstrumentList
from avin.domain.instrument_id import InstrumentId
from avin.domain.instrument_info import InstrumentInfo


class InstrumentList:
    """
    List of instrument reference data.

    Each `InstrumentId` is unique within the list. Instruments with the same
    `InstrumentId` cannot be added even if their remaining metadata differs.

    Instrument order is not guaranteed.
    """

    __slots__ = ("_inner",)
    _inner: PyInstrumentList

    def __init__(self) -> None:
        self._inner = PyInstrumentList()

    def __len__(self) -> int:
        return self._inner.len()

    def __iter__(self) -> Iterator[InstrumentInfo]:
        return (
            InstrumentInfo._from_native(info) for info in self._inner.iter()
        )

    @property
    def is_empty(self) -> bool:
        """
        Whether the list contains no instruments.
        """
        return self._inner.is_empty()

    def add(self, instrument: InstrumentInfo) -> None:
        """
        Add an instrument to the list.

        Raises
        ------
        ValueError
            If the list already contains the same `InstrumentId`.
        """
        self._inner.add(instrument._inner)

    def find(self, iid: InstrumentId) -> InstrumentInfo | None:
        """
        Find instrument reference data by `InstrumentId`.

        Parameters
        ----------
        iid : InstrumentId
            Instrument ID.

        Returns
        -------
        InstrumentInfo | None
            Instrument reference data, or `None` if not found.
        """
        info = self._inner.find(iid._inner)

        if info is None:
            return None

        return InstrumentInfo._from_native(info)

    @classmethod
    def _from_native(cls, inner: PyInstrumentList) -> InstrumentList:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
