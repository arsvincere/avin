from avin import BarKind


def test_bar_kind():
    assert BarKind.Bull == BarKind.Bull
    assert BarKind.Bull != BarKind.Bear
