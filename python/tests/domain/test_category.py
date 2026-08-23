# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Category


def test_category_text():
    assert Category.FUTURE.value == "Future"
    assert str(Category.FUTURE) == "Future"

    assert Category.ETF.value == "ETF"
    assert str(Category.ETF) == "ETF"


def test_category_from_str():
    assert Category.from_str("ShArE") is Category.SHARE
    assert Category.from_str("future") is Category.FUTURE
    assert Category.from_str("ETF") is Category.ETF

    with pytest.raises(ValueError):
        Category.from_str("foo")


def test_category_all():
    assert list(Category) == [
        Category.CURRENCY,
        Category.INDEX,
        Category.SHARE,
        Category.FUTURE,
        Category.BOND,
        Category.OPTION,
        Category.ETF,
    ]
