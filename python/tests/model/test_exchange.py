# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Exchange


def test_exchange():
    assert str(Exchange.MOEX) == "MOEX"
    assert str(Exchange.Bybit) == "Bybit"

    assert Exchange.from_str("BiNaNcE") is Exchange.Binance
    assert Exchange.from_str("SPB") is Exchange.SPB

    with pytest.raises(ValueError):
        Exchange.from_str("foo")
