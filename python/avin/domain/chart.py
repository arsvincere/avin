# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from collections.abc import Iterator

from avin._native import PyChart
from avin.domain.bar import Bar
from avin.domain.instrument_id import InstrumentId
from avin.domain.ticker import Ticker
from avin.domain.timeframe import TimeFrame


class Chart:
    """
    Mutable candlestick chart for one instrument and one timeframe.

    Bars are ordered by increasing timestamp. The last bar is the most recent
    bar and may be unfinished in realtime.

    Parameters
    ----------
    iid : InstrumentId
        Instrument ID.
    tf : TimeFrame
        Chart timeframe.
    bars : list[Bar]
        Bars ordered from oldest to newest.

    Notes
    -----
    The constructor accepts trusted bars. Historical validation belongs to
    storage/service, not to ``Chart``.
    """

    __slots__ = ("_inner",)

    def __init__(
        self,
        iid: InstrumentId,
        tf: TimeFrame,
        bars: list[Bar],
    ) -> None:
        self._inner = PyChart(
            iid._inner,
            tf._inner,
            [bar._inner for bar in bars],
        )

    def __len__(self) -> int:
        return self._inner.len()

    def __iter__(self) -> Iterator[Bar]:
        return iter(self.bars)

    def __getitem__(self, index: int | slice) -> Bar | list[Bar]:
        if isinstance(index, int):
            bar = self._inner.bar(index)

            if bar is None:
                raise IndexError("Chart index out of range")

            return Bar._from_native(bar)

        if isinstance(index, slice):
            return self.bars[index]

        raise TypeError(
            f"Chart indices must be integers or slices, not "
            f"{type(index).__name__}"
        )

    @property
    def iid(self) -> InstrumentId:
        """
        Instrument ID.
        """
        return InstrumentId._from_native(self._inner.iid())

    @property
    def ticker(self) -> Ticker:
        """
        Instrument ticker.
        """
        return Ticker._from_native(self._inner.ticker())

    @property
    def tf(self) -> TimeFrame:
        """
        Chart timeframe.
        """
        return TimeFrame._from_native(self._inner.tf())

    # PERF: `bars` currently materializes all Rust bars as Python objects.
    # This also makes slicing and iteration expensive for large charts.
    # Consider replacing `list[Bar]` with a lazy/native view or adding
    # efficient native slicing when performance becomes relevant.
    # Kept simple for now.
    @property
    def bars(self) -> list[Bar]:
        """
        Chart bars ordered from oldest to newest.
        """
        return [Bar._from_native(bar) for bar in self._inner.bars()]

    @property
    def first(self) -> Bar | None:
        """
        First bar.
        """
        bar = self._inner.first()

        if bar is None:
            return None

        return Bar._from_native(bar)

    @property
    def last(self) -> Bar | None:
        """
        Last bar.

        In realtime the last bar may be unfinished.
        """
        bar = self._inner.last()

        if bar is None:
            return None

        return Bar._from_native(bar)

    @property
    def last_price(self) -> float | None:
        """
        Close price of the last bar.
        """
        return self._inner.last_price()

    @property
    def is_empty(self) -> bool:
        """
        Whether the chart contains no bars.
        """
        return self._inner.is_empty()

    def select(self, from_ts: int, till_ts: int) -> list[Bar]:
        """
        Select bars in the closed interval ``[from_ts, till_ts]``.

        Parameters
        ----------
        from_ts : int
            Inclusive start timestamp in Unix nanoseconds.
        till_ts : int
            Inclusive end timestamp in Unix nanoseconds.

        Returns
        -------
        list[Bar]
            Selected bars.

        Raises
        ------
        ValueError
            If ``from_ts > till_ts``.
        """
        bars = self._inner.select(from_ts, till_ts)

        return [Bar._from_native(bar) for bar in bars]

    def upsert(self, bar: Bar) -> None:
        """
        Insert a bar or replace an existing bar with the same timestamp.

        Bars remain ordered by increasing timestamp.
        """
        self._inner.upsert(bar._inner)

    @classmethod
    def empty(cls, iid: InstrumentId, tf: TimeFrame) -> Chart:
        """
        Create an empty chart.
        """
        return cls._from_native(PyChart.empty(iid._inner, tf._inner))

    @classmethod
    def _from_native(cls, inner: PyChart) -> Chart:
        obj = cls.__new__(cls)
        obj._inner = inner

        return obj
