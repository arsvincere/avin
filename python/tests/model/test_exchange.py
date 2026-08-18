# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Exchange


def test_exchange():
    assert Exchange.SPB.name == "SPB"
    assert Exchange.Binance.name == "Binance"

    assert str(Exchange.MOEX) == "MOEX"
    assert str(Exchange.Bybit) == "Bybit"

    assert Exchange.from_str("BiNaNcE") is Exchange.Binance
    assert Exchange.from_str("SPB") is Exchange.SPB

    with pytest.raises(ValueError):
        Exchange.from_str("foo")


def test_exchange_all():
    assert list(Exchange) == [
        Exchange.Binance,
        Exchange.Bybit,
        Exchange.MOEX,
        Exchange.SPB,
    ]
