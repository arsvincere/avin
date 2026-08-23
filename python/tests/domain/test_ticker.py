# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Ticker


def test_ticker():
    ticker = Ticker("SBER")

    assert str(ticker) == "SBER"


def test_invalid_ticker():
    with pytest.raises(ValueError):
        Ticker("")

    with pytest.raises(ValueError):
        Ticker("SB ER")


def test_ticker_eq():
    assert Ticker("SBER") == Ticker("SBER")
    assert Ticker("SBER") != Ticker("LKOH")
    assert Ticker("SBER") != "SBER"
