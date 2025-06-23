#!/usr/bin/env  python3
# ============================================================================
# URL:          http://avin.info
# AUTHOR:       Alex Avin
# E-MAIL:       mr.alexavin@gmail.com
# LICENSE:      MIT
# ============================================================================

import sys
from datetime import datetime as DateTime

sys.path.append("/home/alex/avin/avin_data")
from avin_data import *


def test_exchange():
    moex = Exchange.MOEX
    assert moex.name == "MOEX"

    from_str = Exchange.from_str("MOEX")
    assert from_str == Exchange.MOEX


def test_market_data():
    md = MarketData.BAR_1M
    assert md.name == "BAR_1M"
    assert str(md) == "1M"

    from_str = MarketData.from_str("BAR_10M")
    assert from_str == MarketData.BAR_10M

    # test next/prev
    dt = DateTime(2025, 6, 5, 12, 57, 12)
    assert MarketData.BAR_1M.prev_dt(dt) == DateTime(2025, 6, 5, 12, 57, 0)
    assert MarketData.BAR_1M.next_dt(dt) == DateTime(2025, 6, 5, 12, 58, 0)

    assert MarketData.BAR_5M.prev_dt(dt) == DateTime(2025, 6, 5, 12, 55, 0)
    assert MarketData.BAR_5M.next_dt(dt) == DateTime(2025, 6, 5, 13, 0, 0)

    assert MarketData.BAR_10M.prev_dt(dt) == DateTime(2025, 6, 5, 12, 50, 0)
    assert MarketData.BAR_10M.next_dt(dt) == DateTime(2025, 6, 5, 13, 0, 0)

    assert MarketData.BAR_1H.prev_dt(dt) == DateTime(2025, 6, 5, 12, 0, 0)
    assert MarketData.BAR_1H.next_dt(dt) == DateTime(2025, 6, 5, 13, 0, 0)

    assert MarketData.BAR_D.prev_dt(dt) == DateTime(2025, 6, 5, 0, 0, 0)
    assert MarketData.BAR_D.next_dt(dt) == DateTime(2025, 6, 6, 0, 0, 0)

    assert MarketData.BAR_W.prev_dt(dt) == DateTime(2025, 6, 2, 0, 0, 0)
    assert MarketData.BAR_W.next_dt(dt) == DateTime(2025, 6, 9, 0, 0, 0)

    assert MarketData.BAR_M.prev_dt(dt) == DateTime(2025, 6, 1, 0, 0, 0)
    assert MarketData.BAR_M.next_dt(dt) == DateTime(2025, 7, 1, 0, 0, 0)


def test_source():
    src = Source.TINKOFF
    assert src.name == "TINKOFF"

    src = Source.MOEX
    assert src.name == "MOEX"

    from_str = Source.from_str("MOEX")
    assert from_str == Source.MOEX

    from_str = Source.from_str("TINKOFF")
    assert from_str == Source.TINKOFF


def test_iid():
    info = {
        "exchange": "MOEX",
        "category": "SHARE",
        "ticker": "SBER",
        "figi": "BBG004730N88",
        "name": "Сбер Банк",
        "lot": "10",
        "step": "0.01",
    }

    iid = Iid(info)
    assert str(iid) == "MOEX_SHARE_SBER"
    assert iid.exchange() == Exchange.MOEX
    assert iid.category() == Category.SHARE
    assert iid.ticker() == "SBER"
    assert iid.figi() == "BBG004730N88"
    assert iid.name() == "Сбер Банк"
    assert iid.lot() == 10
    assert iid.step() == 0.01
    assert iid.path() == "/home/alex/trading/data/MOEX/SHARE/SBER"
