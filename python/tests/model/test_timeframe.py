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

NON_DELEGATED_NATIVE_METHODS = {
    "all",
    "eq",
}

DELEGATED_METHODS = {
    "__str__": ("display", ()),
    "nanos": ("nanos", ()),
    "seconds": ("seconds", ()),
    "timedelta": ("timedelta", ()),
    "begin_frame_ts": (
        "begin_frame_ts",
        (1_691_000_123_456_789_000,),
    ),
    "end_frame_ts": (
        "end_frame_ts",
        (1_691_000_123_456_789_000,),
    ),
}

WRAPPED_DELEGATED_METHODS = {
    "from_str": "from_str",
}


def test_enum_variants_complete():
    public = list(TimeFrame)
    native = PyTimeFrame.all()
    assert len(public) == len(native)

    for native_tf in native:
        matches = [tf for tf in public if native_tf.eq(tf.value)]
        assert len(matches) == 1


def test_native_methods_complete():
    native_methods = {
        name
        for name in PyTimeFrame.__dict__
        if not name.startswith("_") and callable(getattr(PyTimeFrame, name))
    }

    delegated_native_methods = {
        native_method for native_method, _ in DELEGATED_METHODS.values()
    }

    covered_methods = (
        NON_DELEGATED_NATIVE_METHODS
        | delegated_native_methods
        | set(WRAPPED_DELEGATED_METHODS.values())
    )

    assert native_methods == covered_methods


def test_delegation():
    for tf in TimeFrame:
        for public_method, (native_method, args) in DELEGATED_METHODS.items():
            public_result = getattr(tf, public_method)(*args)
            native_result = getattr(tf.value, native_method)(*args)

            assert public_result == native_result


def test_wrapped_delegation():
    for tf in TimeFrame:
        text = tf.value.display()

        for public_method, native_method in WRAPPED_DELEGATED_METHODS.items():
            public_result = getattr(TimeFrame, public_method)(text)
            native_result = getattr(PyTimeFrame, native_method)(text)

            assert native_result.eq(public_result.value)


def test_type_conversions():
    assert isinstance(TimeFrame.M15.nanos(), int)
    assert isinstance(TimeFrame.M15.seconds(), int)
    assert isinstance(TimeFrame.M15.timedelta(), TimeDelta)

    assert TimeFrame.MONTH.nanos() is None
    assert TimeFrame.MONTH.seconds() is None
    assert TimeFrame.MONTH.timedelta() is None


def test_error_mapping():
    with pytest.raises(ValueError):
        TimeFrame.from_str("foo")
