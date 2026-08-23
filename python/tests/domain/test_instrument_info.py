# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

import avin
from avin.domain.instrument_info import InstrumentInfo


def test_not_public():
    assert "InstrumentInfo" not in avin.__all__
    assert not hasattr(avin, "InstrumentInfo")


def test_cannot_instantiate():
    with pytest.raises(TypeError):
        InstrumentInfo()

    with pytest.raises(TypeError):
        InstrumentInfo({"exchange": "MOEX"})


def test_raw_info_not_public():
    assert not hasattr(InstrumentInfo, "raw_info")
