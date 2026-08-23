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
    ...     print(category)
    Currency
    Index
    Stock
    Future
    Bond
    Option
    ETF
    >>> category = Category.FUTURE
    >>> str(category)
    'Future'
    >>> Category.from_str("bond") == Category.BOND
    True
    >>> Category.from_str("stock") is Category.STOCK
    True
    """

    CURRENCY = PyCategory.Currency
    INDEX = PyCategory.Index
    STOCK = PyCategory.Stock
    FUTURE = PyCategory.Future
    BOND = PyCategory.Bond
    OPTION = PyCategory.Option
    ETF = PyCategory.ETF

    _inner: PyCategory

    def __new__(cls, inner: PyCategory):
        obj = object.__new__(cls)
        obj._value_ = inner.display()
        obj._inner = inner

        return obj

    def __str__(self) -> str:
        return self._inner.display()

    @classmethod
    def from_str(cls, s: str) -> Category:
        """
        Parse a category.

        Parsing is case-insensitive.

        Parameters
        ----------
        s : str
            Category.

        Raises
        ------
        ValueError
            If the category is unknown.
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
