"""Custom exceptions."""


class ConfigNotFound(Exception):
    """Config not found exception."""


class SourceNotFound(Exception):
    """Source not found exception."""


class CategoryNotFound(Exception):
    """Category not found exception."""


class TickerNotFound(Exception):
    """Ticker not found exception."""


class InvalidMarketData(Exception):
    """Invalid market data name exception."""
