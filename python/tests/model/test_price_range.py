import pytest

from avin import PriceRange


def test_price_range():
    r = PriceRange(100.0, 110.0)

    assert r.low == 100.0
    assert r.high == 110.0

    assert r.width() == 10.0
    assert r.middle() == 105.0
    assert r.contains(105.1)


def test_invalid_price_range():
    with pytest.raises(ValueError):
        PriceRange(110.0, 100.0)


def test_price_range_str():
    r = PriceRange(123.5, 234.5)

    assert str(r) == "[123.5, 234.5]"


def test_price_range_eq():
    assert PriceRange(100.0, 110.0) == PriceRange(100.0, 110.0)
    assert PriceRange(100.0, 110.0) != PriceRange(100.0, 120.0)
    assert PriceRange(100.0, 110.0) != None
    assert PriceRange(100.0, 110.0) != "foo"
    assert PriceRange(100.0, 110.0) != 123


def test_price_range_contains():
    r = PriceRange(100.0, 110.0)

    assert 100.0 in r
    assert 106.6 in r
    assert 110.0 in r

    assert 99.9 not in r
    assert 110.1 not in r
