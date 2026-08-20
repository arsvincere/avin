# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────

from datetime import datetime as DateTime

from avin import Bar, BarDirection, PriceRange
from avin._native import PyBar

SPECIAL_NATIVE_METHODS = {
    "eq",
}

DELEGATED_PROPERTIES = {
    "ts": "ts",
    "o": "o",
    "h": "h",
    "l": "l",
    "c": "c",
    "v": "v",
}

DELEGATED_METHODS = {
    "__str__": ("display", ()),
    "__contains__": ("contains", (10.3,)),
    "dt": ("dt", ()),
    "is_bear": ("is_bear", ()),
    "is_bull": ("is_bull", ()),
    "is_neutral": ("is_neutral", ()),
    "contains": ("contains", (10.3,)),
}

WRAPPED_DELEGATED_METHODS = {
    "direction": "direction",
    "range": "range",
    "body": "body",
    "lower": "lower",
    "upper": "upper",
}


def make_bar() -> Bar:
    return Bar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)


def test_new_delegation():
    public = Bar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)
    native = PyBar(123_456_789, 10.0, 11.1, 9.9, 10.5, 5000)

    assert public._inner.eq(native)


def test_native_methods_complete():
    native_methods = {
        name
        for name in PyBar.__dict__
        if not name.startswith("_") and callable(getattr(PyBar, name))
    }

    covered_methods = (
        set(DELEGATED_PROPERTIES.values())
        | {native_method for native_method, _ in DELEGATED_METHODS.values()}
        | set(WRAPPED_DELEGATED_METHODS.values())
        | SPECIAL_NATIVE_METHODS
    )

    assert native_methods == covered_methods


def test_properties_delegation():
    bar = make_bar()

    for public_property, native_method in DELEGATED_PROPERTIES.items():
        public_result = getattr(bar, public_property)
        native_result = getattr(bar._inner, native_method)()

        assert public_result == native_result


def test_delegation():
    bar = make_bar()

    for public_method, (native_method, args) in DELEGATED_METHODS.items():
        public_result = getattr(bar, public_method)(*args)
        native_result = getattr(bar._inner, native_method)(*args)

        assert public_result == native_result


def test_wrapped_delegation():
    bar = make_bar()

    direction = bar.direction()
    native_direction = bar._inner.direction()
    assert direction.value == native_direction.value()

    for public_method, native_method in WRAPPED_DELEGATED_METHODS.items():
        # TODO: переделать сначала BarDirection по общей схеме тогда тут
        # будет нормальный тест а не исключение...
        if public_method == "direction":
            continue

        public_result = getattr(bar, public_method)()
        native_result = getattr(bar._inner, native_method)()

        assert native_result.eq(public_result._inner)


def test_direction_wrapped_delegation():
    bar = make_bar()

    public = bar.direction()
    native = bar._inner.direction()

    assert public.value == native.value()


def test_eq_delegation():
    a = make_bar()
    b = make_bar()
    c = Bar(987_654_321, 10.0, 11.1, 9.9, 10.5, 50)

    assert (a == b) == a._inner.eq(b._inner)
    assert (a == c) == a._inner.eq(c._inner)


def test_type_conversions():
    bar = make_bar()

    assert isinstance(bar.dt(), DateTime)
    assert isinstance(bar.direction(), BarDirection)

    assert isinstance(bar.range(), PriceRange)
    assert isinstance(bar.body(), PriceRange)
    assert isinstance(bar.lower(), PriceRange)
    assert isinstance(bar.upper(), PriceRange)
