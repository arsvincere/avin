import pytest

from avin import PriceRange


def test_price_range():
    r = PriceRange(100.0, 110.0)

    assert r.low == 100.0
    assert r.high == 110.0
    assert r.width() == 10.0
    assert r.middle() == 105.0
    assert 105.0 in r


def test_invalid_price_range():
    with pytest.raises(ValueError):
        PriceRange(110.0, 100.0)


def test_price_range_eq():
    assert PriceRange(100.0, 110.0) == PriceRange(100.0, 110.0)
    assert PriceRange(100.0, 110.0) != PriceRange(100.0, 120.0)
    assert PriceRange(100.0, 110.0) != None
    assert PriceRange(100.0, 110.0) != "foo"
    assert PriceRange(100.0, 110.0) != 123
