# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

import pytest

from avin import (
    Bar,
    Category,
    Chart,
    Exchange,
    InstrumentId,
    Ticker,
    TimeFrame,
)

SECOND = 1_000_000_000


def iid() -> InstrumentId:
    return InstrumentId(
        Exchange.MOEX,
        Category.SHARE,
        Ticker("SBER"),
    )


def bar(n: int) -> Bar:
    price = float(n + 1)

    return Bar(
        n * SECOND,
        price,
        price,
        price,
        price,
        (n + 1) * 100,
    )


def chart() -> Chart:
    return Chart(
        iid(),
        TimeFrame.S1,
        [bar(n) for n in range(5)],
    )


def test_chart():
    c = chart()

    assert c.iid == iid()
    assert c.ticker == Ticker("SBER")
    assert c.tf is TimeFrame.S1

    assert len(c) == 5
    assert not c.is_empty

    assert c.bars == [bar(0), bar(1), bar(2), bar(3), bar(4)]
    assert c.first == bar(0)
    assert c.last == bar(4)
    assert c.last_price == 5.0


def test_chart_empty():
    c = Chart.empty(iid(), TimeFrame.S1)

    assert len(c) == 0
    assert c.is_empty
    assert c.bars == []
    assert c.first is None
    assert c.last is None
    assert c.last_price is None


def test_chart_index():
    c = chart()

    assert c[0] == bar(0)
    assert c[1] == bar(1)
    assert c[4] == bar(4)

    assert c[-1] == bar(4)
    assert c[-2] == bar(3)
    assert c[-5] == bar(0)

    with pytest.raises(IndexError):
        _ = c[5]

    with pytest.raises(IndexError):
        _ = c[-6]


def test_chart_slice():
    c = chart()

    assert c[1:4] == [bar(1), bar(2), bar(3)]
    assert c[-3:] == [bar(2), bar(3), bar(4)]
    assert c[:2] == [bar(0), bar(1)]
    assert c[::2] == [bar(0), bar(2), bar(4)]


def test_chart_iter():
    c = chart()

    assert list(c) == [bar(0), bar(1), bar(2), bar(3), bar(4)]


def test_chart_select():
    c = chart()

    assert c.select(SECOND, 3 * SECOND) == [
        bar(1),
        bar(2),
        bar(3),
    ]

    assert c.select(SECOND + 1, 4 * SECOND - 1) == [
        bar(2),
        bar(3),
    ]

    assert c.select(5 * SECOND, 6 * SECOND) == []


def test_chart_select_invalid_range():
    c = chart()

    with pytest.raises(ValueError):
        c.select(SECOND, 0)


def test_chart_upsert():
    c = Chart.empty(iid(), TimeFrame.S1)

    # Empty chart.
    c.upsert(bar(2))
    assert c.bars == [bar(2)]

    # Append.
    c.upsert(bar(4))
    assert c.bars == [bar(2), bar(4)]

    # Insert in the middle.
    c.upsert(bar(3))
    assert c.bars == [bar(2), bar(3), bar(4)]

    # Insert at the beginning.
    c.upsert(bar(1))
    assert c.bars == [bar(1), bar(2), bar(3), bar(4)]

    # Replace.
    replacement = Bar(
        3 * SECOND,
        30.0,
        31.0,
        29.0,
        30.5,
        999,
    )

    c.upsert(replacement)

    assert len(c) == 4
    assert c[2] == replacement
