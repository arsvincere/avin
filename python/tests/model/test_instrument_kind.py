# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import InstrumentKind


def test_instrument_kind_text():
    assert InstrumentKind.FUTURE.value == "Future"
    assert str(InstrumentKind.FUTURE) == "Future"

    assert InstrumentKind.ETF.value == "ETF"
    assert str(InstrumentKind.ETF) == "ETF"


def test_instrument_kind_from_str():
    assert InstrumentKind.from_str("StOcK") is InstrumentKind.STOCK
    assert InstrumentKind.from_str("future") is InstrumentKind.FUTURE
    assert InstrumentKind.from_str("ETF") is InstrumentKind.ETF

    with pytest.raises(ValueError):
        InstrumentKind.from_str("foo")


def test_instrument_kind_all():
    assert list(InstrumentKind) == [
        InstrumentKind.CURRENCY,
        InstrumentKind.INDEX,
        InstrumentKind.STOCK,
        InstrumentKind.FUTURE,
        InstrumentKind.BOND,
        InstrumentKind.OPTION,
        InstrumentKind.ETF,
    ]
