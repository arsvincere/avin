from avin import BarKind


def test_bar_kind():
    assert BarKind.BULL == BarKind.BULL
    assert BarKind.BULL != BarKind.BEAR
