# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Category, Exchange, InstrumentId, Ticker


def test_instrument_id():
    iid = InstrumentId(
        Exchange.MOEX,
        Category.SHARE,
        Ticker("SBER"),
    )

    assert iid.exchange is Exchange.MOEX
    assert iid.category is Category.SHARE
    assert iid.ticker == Ticker("SBER")

    assert str(iid) == "MOEX.SHARE.SBER"


def test_instrument_id_from_str():
    iid = InstrumentId.from_str("moex.SHARE.SBER")

    assert iid.exchange is Exchange.MOEX
    assert iid.category is Category.SHARE
    assert iid.ticker == Ticker("SBER")

    iid = InstrumentId.from_str("moex.SHARE.BRK.B")
    assert iid.ticker == Ticker("BRK.B")


def test_invalid_instrument_id():
    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.SHARE")

    with pytest.raises(ValueError):
        InstrumentId.from_str("foo.SHARE.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.foo.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.SHARE.")


def test_instrument_id_eq():
    iid = InstrumentId.from_str("MOEX.SHARE.SBER")

    assert iid == InstrumentId.from_str("MOEX.SHARE.SBER")
    assert iid != InstrumentId.from_str("MOEX.SHARE.LKOH")
    assert iid != "MOEX.SHARE.SBER"
