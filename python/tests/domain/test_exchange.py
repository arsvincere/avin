# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin.domain.exchange import Exchange


def test_exchange_str():
    assert str(Exchange.MOEX) == "MOEX"
    assert str(Exchange.BYBIT) == "Bybit"


def test_exchange_key():
    assert Exchange.BINANCE.key == "binance"
    assert Exchange.BYBIT.key == "bybit"
    assert Exchange.MOEX.key == "moex"
    assert Exchange.SPB.key == "spb"


def test_exchange_from_str():
    assert Exchange.from_str("BiNaNcE") == Exchange.BINANCE
    assert Exchange.from_str("SPB") is Exchange.SPB

    with pytest.raises(ValueError):
        Exchange.from_str("foo")


def test_exchange_all():
    assert list(Exchange) == [
        Exchange.BINANCE,
        Exchange.BYBIT,
        Exchange.MOEX,
        Exchange.SPB,
    ]
