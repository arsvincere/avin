# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin.domain.category import Category


def test_category_key():
    assert Category.FUTURE.key == "future"
    assert Category.CURRENCY_PAIR.key == "currency_pair"


def test_category_str():
    assert str(Category.FUTURE) == "Future"
    assert str(Category.CURRENCY_PAIR) == "Currency pair"


def test_category_from_str():
    assert Category.from_str("FuTuRe") is Category.FUTURE
    assert Category.from_str("CURRENCY_PAIR") is Category.CURRENCY_PAIR

    with pytest.raises(ValueError):
        Category.from_str("foo")


def test_category_all():
    assert list(Category) == [
        Category.INDEX,
        Category.SHARE,
        Category.FUTURE,
        Category.BOND,
        Category.OPTION,
        Category.ETF,
        Category.CURRENCY_PAIR,
    ]
