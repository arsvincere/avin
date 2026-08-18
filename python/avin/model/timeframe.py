# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from datetime import timedelta as TimeDelta
from enum import Enum

from avin._native import PyTimeFrame


class TimeFrame(Enum):
    """
    Trading timeframe used by AVIN charts and footprints.

    Fixed timeframes range from one second to one week. ``MONTH`` is a
    calendar-based timeframe and therefore has no fixed duration.

    Examples
    --------
    >>> timeframe = TimeFrame.M1
    >>> str(timeframe)
    '1M'

    >>> TimeFrame.from_str("5m") is TimeFrame.M5
    True

    >>> TimeFrame.M15.seconds()
    900

    >>> TimeFrame.MONTH.seconds() is None
    True
    """

    S1 = PyTimeFrame.S1
    S5 = PyTimeFrame.S5
    S10 = PyTimeFrame.S10
    S15 = PyTimeFrame.S15

    M1 = PyTimeFrame.M1
    M5 = PyTimeFrame.M5
    M10 = PyTimeFrame.M10
    M15 = PyTimeFrame.M15

    H1 = PyTimeFrame.H1
    H4 = PyTimeFrame.H4

    DAY = PyTimeFrame.Day
    WEEK = PyTimeFrame.Week
    MONTH = PyTimeFrame.Month

    def __str__(self) -> str:
        return self.value.str()

    @classmethod
    def from_str(cls, s: str) -> TimeFrame:
        """
        Parse a timeframe from its canonical text representation.

        Parsing is case-insensitive. Valid representations include ``"1S"``,
        ``"15M"``, ``"4H"``, ``"D"``, ``"W"``, and ``"M"``.

        Parameters
        ----------
        s : str
            Timeframe representation.

        Raises
        ------
        ValueError
            If the timeframe is unknown.
        RuntimeError
            If the native and public Python timeframe definitions are out of
            sync.

        Examples
        --------
        >>> TimeFrame.from_str("1m") is TimeFrame.M1
        True
        >>> TimeFrame.from_str("D") is TimeFrame.DAY
        True
        """
        native = PyTimeFrame.from_str(s)

        for tf in cls:
            if native.eq(tf.value):
                return tf

        # Reached only if the native and public Python timeframe definitions
        # are out of sync, e.g. a new native variant is missing from this Enum.
        raise RuntimeError("native timeframe is missing from TimeFrame")

    def nanos(self) -> int | None:
        """
        Return the fixed timeframe duration in nanoseconds.

        Returns ``None`` for ``MONTH`` because calendar months do not have a
        fixed duration.
        """
        return self.value.nanos()

    def seconds(self) -> int | None:
        """
        Return the fixed timeframe duration in seconds.

        Returns ``None`` for ``MONTH`` because calendar months do not have a
        fixed duration.
        """
        return self.value.seconds()

    def timedelta(self) -> TimeDelta | None:
        """
        Return the fixed timeframe duration as ``datetime.timedelta``.

        Returns ``None`` for ``MONTH`` because calendar months do not have a
        fixed duration.
        """
        return self.value.timedelta()

    def begin_frame_ts(self, ts: int) -> int:
        """
        Return the beginning of the frame containing ``ts``.

        Parameters
        ----------
        ts : int
            Unix timestamp in nanoseconds.

        Notes
        -----
        The returned boundary is inclusive. Frame boundaries are calculated
        in UTC. Weeks begin on Monday and months begin on the first day of the
        month.

        Examples
        --------
        A timestamp inside a 5-minute frame is aligned to the beginning of
        that frame.

        >>> from datetime import datetime, UTC

        >>> dt = datetime(2026, 8, 18, 10, 13, 42, tzinfo=UTC)
        >>> print(dt)
        2026-08-18 10:13:42+00:00

        >>> ts = int(dt.timestamp()) * 1_000_000_000
        >>> begin_ts = TimeFrame.M5.begin_frame_ts(ts)
        >>> begin_dt = datetime.fromtimestamp(begin_ts / 1_000_000_000, UTC)

        >>> print(begin_dt)
        2026-08-18 10:10:00+00:00
        """
        return self.value.begin_frame_ts(ts)

    def end_frame_ts(self, ts: int) -> int:
        """
        Return the end of the frame containing ``ts``.

        Parameters
        ----------
        ts : int
            Unix timestamp in nanoseconds.

        Notes
        -----
        The returned boundary is exclusive. Together with
        :meth:`begin_frame_ts`, a frame is represented as the half-open
        interval ``[begin, end)``.
        """
        return self.value.end_frame_ts(ts)
