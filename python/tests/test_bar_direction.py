from avin import BarDirection


def test_bar_direction():
    assert BarDirection.Bull == BarDirection.Bull
    assert BarDirection.Bull != BarDirection.Bear

    assert str(BarDirection.Bull) == "Bull"
    assert str(BarDirection.Doji) == "Doji"
    assert str(BarDirection.Bear) == "Bear"

    assert BarDirection.Bull.name == "Bull"
    assert BarDirection.Doji.name == "Doji"
    assert BarDirection.Bear.name == "Bear"

    assert BarDirection.Bull.value == 1
    assert BarDirection.Doji.value == 0
    assert BarDirection.Bear.value == -1
