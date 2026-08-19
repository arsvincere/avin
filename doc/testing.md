avin / Rust проверяют:
- логику
- корректность алгоритмов
- пограничные случаи
- display / parsing

binds / Rust проверяют:
- только корректность делегирования rust wrapper: PyXxx.method() == Xxx.method()
- никакой domain logic заново.

python / pytest проверяют:
- native enum variants полностью представлены в public Enum
- весь native method surface учтен
- public methods -> native methods
- Rust/PyO3 -> Python type conversions
- Rust error -> Python exception
- python str(obj) -> native display obj
- public from_str -> правильный public member


Rust domain tests
    exhaustive correctness

Rust bind tests
    Rust domain → PyO3 wrapper без искажений

Python binding tests
    PyO3 wrapper → public Python без искажений

Python doctests
    несколько реальных public API happy paths, end-to-end smoke tests
