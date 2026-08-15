from avin import BarDirection


def test_bar_direction():
    assert BarDirection.Bull == BarDirection.Bull
    assert BarDirection.Bull != BarDirection.Bear

    assert str(BarDirection.Bull) == "Bull"
    assert str(BarDirection.Neutral) == "Neutral"
    assert str(BarDirection.Bear) == "Bear"

    assert BarDirection.Bull.name == "Bull"
    assert BarDirection.Neutral.name == "Neutral"
    assert BarDirection.Bear.name == "Bear"

    assert BarDirection.Bull.value == 1
    assert BarDirection.Neutral.value == 0
    assert BarDirection.Bear.value == -1
