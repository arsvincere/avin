from enum import Enum

from avin import _native


class BarDirection(Enum):
    """
    Bar direction.

    Indicates whether a bar is bullish, neutral, or bearish.

    Attributes
    ----------
    Bull
        The bar closes above its opening price.
    Neutral
        The bar closes at its opening price.
    Bear
        The bar closes below its opening price.

    Examples
    --------
    >>> direction = BarDirection.Bull
    >>> direction.name
    'Bull'
    >>> direction.value
    1
    >>> str(direction)
    'Bull'
    """

    Bull = _native.BarDirection.Bull.value()
    Neutral = _native.BarDirection.Neutral.value()
    Bear = _native.BarDirection.Bear.value()

    def __str__(self) -> str:
        return self.name
