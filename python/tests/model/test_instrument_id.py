# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import Exchange, InstrumentId, InstrumentKind, Symbol


def test_instrument_id():
    iid = InstrumentId(
        Exchange.MOEX,
        InstrumentKind.STOCK,
        Symbol("SBER"),
    )

    assert iid.exchange is Exchange.MOEX
    assert iid.kind is InstrumentKind.STOCK
    assert iid.symbol == Symbol("SBER")

    assert str(iid) == "MOEX.Stock.SBER"


def test_instrument_id_from_str():
    iid = InstrumentId.from_str("moex.stock.SBER")

    assert iid.exchange is Exchange.MOEX
    assert iid.kind is InstrumentKind.STOCK
    assert iid.symbol == Symbol("SBER")

    iid = InstrumentId.from_str("moex.stock.BRK.B")
    assert iid.symbol == Symbol("BRK.B")


def test_invalid_instrument_id():
    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.Stock")

    with pytest.raises(ValueError):
        InstrumentId.from_str("foo.Stock.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.foo.SBER")

    with pytest.raises(ValueError):
        InstrumentId.from_str("MOEX.Stock.")


def test_instrument_id_eq():
    iid = InstrumentId.from_str("MOEX.Stock.SBER")

    assert iid == InstrumentId.from_str("MOEX.Stock.SBER")
    assert iid != InstrumentId.from_str("MOEX.Stock.LKOH")
    assert iid != "MOEX.Stock.SBER"
