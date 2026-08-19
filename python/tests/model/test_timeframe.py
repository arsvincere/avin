# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import timedelta as TimeDelta

import pytest

from avin import TimeFrame
from avin._native import PyTimeFrame

SPECIAL_METHODS = {
    "all",
    "from_str",
    "display",
    "eq",
}

DELEGATED_METHODS = {
    "nanos": (),
    "seconds": (),
    "timedelta": (),
    "begin_frame_ts": (1_691_000_123_456_789_000,),
    "end_frame_ts": (1_691_000_123_456_789_000,),
}


def test_binding_complete():
    public = list(TimeFrame)
    native = PyTimeFrame.all()
    assert len(public) == len(native)

    for native_tf in native:
        matches = [tf for tf in public if native_tf.eq(tf.value)]
        assert len(matches) == 1


def test_binding_methods_complete():
    native_methods = {
        name
        for name in PyTimeFrame.__dict__
        if not name.startswith("_") and callable(getattr(PyTimeFrame, name))
    }

    covered_methods = SPECIAL_METHODS | set(DELEGATED_METHODS)
    assert native_methods == covered_methods


def test_binding_delegation():
    for tf in TimeFrame:
        for method, args in DELEGATED_METHODS.items():
            public_result = getattr(tf, method)(*args)
            native_result = getattr(tf.value, method)(*args)

            assert public_result == native_result


def test_binding_type_conversions():
    assert type(TimeFrame.M15.nanos()) is int
    assert type(TimeFrame.M15.seconds()) is int
    assert isinstance(TimeFrame.M15.timedelta(), TimeDelta)

    assert TimeFrame.MONTH.nanos() is None
    assert TimeFrame.MONTH.seconds() is None
    assert TimeFrame.MONTH.timedelta() is None


def test_binding_error_mapping():
    with pytest.raises(ValueError):
        TimeFrame.from_str("foo")


def test_binding_str():
    for tf in TimeFrame:
        assert str(tf) == tf.value.display()


def test_binding_from_str():
    for tf in TimeFrame:
        display = tf.value.display()
        assert TimeFrame.from_str(display) is tf
