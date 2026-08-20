# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from avin import BarDirection


def test_bar_direction():
    assert str(BarDirection.BULL) == "Bull"
    assert str(BarDirection.NEUTRAL) == "Neutral"
    assert str(BarDirection.BEAR) == "Bear"

    assert BarDirection.BULL.value == 1
    assert BarDirection.NEUTRAL.value == 0
    assert BarDirection.BEAR.value == -1
