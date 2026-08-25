# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from avin.domain.bar_direction import BarDirection


def test_bar_direction():
    assert BarDirection.BULL.value == 1
    assert BarDirection.NEUTRAL.value == 0
    assert BarDirection.BEAR.value == -1

    assert str(BarDirection.BULL) == "Bull"
    assert str(BarDirection.NEUTRAL) == "Neutral"
    assert str(BarDirection.BEAR) == "Bear"
