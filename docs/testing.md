# General

* Обычные unit-tests должны быть deterministic и не зависеть от network, external API или environment конкретной developer machine.
* Tests проверяют meaningful observable behavior, а не детали implementation.
* Tests по возможности используют real AVIN components и explicit test data. Дополнительная test machinery используется только когда она действительно упрощает test.

# Rust tests

* Exhaustive correctness.
* All variants, all branches, edge cases.

# Python tests

* Don't repeat all rust tests.
* Compact public API / end-to-end contract.
* Python/FFI-specific поведение.

AVIN использует `pytest`.
