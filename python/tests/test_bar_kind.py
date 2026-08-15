from avin import BarKind


def test_bar_kind():
    assert BarKind.Bull == BarKind.Bull
    assert BarKind.Bull != BarKind.Bear

    assert BarKind.Bull.name == "Bull"
    assert BarKind.Doji.name == "Doji"
    assert BarKind.Bear.name == "Bear"

    assert BarKind.Bull.value == 1
    assert BarKind.Doji.value == 0
    assert BarKind.Bear.value == -1
