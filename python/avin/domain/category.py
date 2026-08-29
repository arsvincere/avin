# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from __future__ import annotations

from enum import Enum

from avin._native import PyCategory


class Category(Enum):
    """
    Financial category.

    Examples
    --------
    >>> for category in Category:
    ...     print(category.key)
    index
    share
    future
    bond
    option
    etf
    currency_pair
    >>> category = Category.FUTURE
    >>> str(category)
    'Future'
    >>> Category.from_str("bond") == Category.BOND
    True
    >>> Category.from_str("share") is Category.SHARE
    True
    """

    INDEX = PyCategory.Index
    SHARE = PyCategory.Share
    FUTURE = PyCategory.Future
    BOND = PyCategory.Bond
    OPTION = PyCategory.Option
    ETF = PyCategory.Etf
    CURRENCY_PAIR = PyCategory.CurrencyPair

    _inner: PyCategory

    def __new__(cls, inner: PyCategory):
        obj = object.__new__(cls)
        obj._value_ = inner.key()
        obj._inner = inner

        return obj

    def __str__(self) -> str:
        return self._inner.display()

    @property
    def key(self) -> str:
        """
        Return the stable machine-readable category key.

        The key is intended for persistence, configuration, and serialization.
        """
        return self._inner.key()

    @classmethod
    def from_str(cls, s: str) -> Category:
        """
        Parse a category key.

        Parsing is case-insensitive.

        Parameters
        ----------
        s : str
            Category key.

        Raises
        ------
        ValueError
            If the category key is unknown.
        RuntimeError
            If the native and public Python category definitions are
            out of sync.

        Examples
        --------
        >>> Category.from_str("FuTuRe") is Category.FUTURE
        True
        >>> Category.from_str("ETF") is Category.ETF
        True
        """
        return Category._from_native(PyCategory.from_str(s))

    @classmethod
    def _from_native(cls, inner: PyCategory) -> Category:
        for category in cls:
            if category._inner.eq(inner):
                return category

        raise RuntimeError("native category is missing from Category")
