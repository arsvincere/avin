# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Category, Exchange, InstrumentId, Symbol


def test_instrument_id():
    iid = InstrumentId(
        Exchange.MOEX,
        Category.SHARE,
        Symbol("SBER"),
    )

    assert iid.exchange is Exchange.MOEX
    assert iid.category is Category.SHARE
    assert iid.symbol == Symbol("SBER")

    assert str(iid) == "MOEX.Share.SBER"


def test_instrument_id_from_str():
    iid = InstrumentId.from_str("moex.share.SBER")

    assert iid.exchange is Exchange.MOEX
    assert iid.category is Category.SHARE
    assert iid.symbol == Symbol("SBER")

    iid = InstrumentId.from_str("moex.share.BRK.B")
    assert iid.symbol == Symbol("BRK.B")


def test_invalid_instrument_id():
    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.Share")

    with pytest.raises(ValueError):
        InstrumentId.from_str("foo.Share.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.foo.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.Share.")


def test_instrument_id_eq():
    iid = InstrumentId.from_str("MOEX.Share.SBER")

    assert iid == InstrumentId.from_str("MOEX.Share.SBER")
    assert iid != InstrumentId.from_str("MOEX.Share.LKOH")
    assert iid != "MOEX.Share.SBER"
