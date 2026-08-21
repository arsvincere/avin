# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Symbol


def test_symbol():
    symbol = Symbol("SBER")

    assert str(symbol) == "SBER"


def test_invalid_symbol():
    with pytest.raises(ValueError):
        Symbol("")

    with pytest.raises(ValueError):
        Symbol("SB ER")


def test_symbol_eq():
    assert Symbol("SBER") == Symbol("SBER")
    assert Symbol("SBER") != Symbol("LKOH")
    assert Symbol("SBER") != "SBER"
