# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import avin
from avin.domain.category import Category
from avin.domain.exchange import Exchange
from avin.domain.instrument_id import InstrumentId
from avin.domain.instrument_list import InstrumentList
from avin.domain.ticker import Ticker


def test_not_public():
    assert "InstrumentList" not in avin.__all__
    assert not hasattr(avin, "InstrumentList")


def iid() -> InstrumentId:
    return InstrumentId(
        Exchange.MOEX,
        Category.SHARE,
        Ticker("SBER"),
    )


def test_empty():
    instruments = InstrumentList()

    assert len(instruments) == 0
    assert instruments.is_empty
    assert list(instruments) == []


def test_find_missing():
    instruments = InstrumentList()

    assert instruments.find(iid()) is None
