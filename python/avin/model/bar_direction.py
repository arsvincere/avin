from enum import Enum

from avin import _native


class BarDirection(Enum):
    """
    Bar direction.

    Indicates whether the bar is bullish, neutral, or bearish.
    """

    Bull = _native.BarDirection.Bull.value()
    Neutral = _native.BarDirection.Neutral.value()
    Bear = _native.BarDirection.Bear.value()

    def __str__(self) -> str:
        return self.name
