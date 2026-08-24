# Docstrings

* Type: назначение, смысл объекта, invariants, happy path examples.
* Method: контракт, ошибки, важные edge cases.
* Method with `Result<T, E>`: документируем условия, при которых возвращается ошибка.

## Rust

* english only
* developer-oriented
* rustdoc style

## Python

* english only
* traders-oriented
* numpy style

## Line length

Строка docstring — не более 78 символов включая indentation.

Перенос выполняется вручную: Ruff formatter строки внутри docstring не разбивает, rustfmt тоже. Карл, задолбал, переноси строки.

# Examples

* Type: основные happy use-cases.
* Method: только если использование или поведение неочевидно.

В rust - более ориентированы на разработчиков, показывающие архитектуру, контракты и технические детали API.

В python - более ориентированы на трейдерские cases.

# Comments

Comment объясняет неочевидную причину, constraint, risk, workaround или trade-off.

Не должен пересказывать то, что и так видно из кода.

# No docs

* Rust/PyO3 internals.
* Python/_native internals.

# File header

Code files в AVIN используют единый project header:

```rust
// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────
```

```python
# ────────────────────────────────────────────────────────────────────────────
#  AVIN
#  Understand the market before trading it.
#
#  https://avin.info
# ────────────────────────────────────────────────────────────────────────────
```

Header располагается в начале Rust/Python file.

После header должна находиться одна пустая строка.

## Critical Empty Line

The empty line in the file header comment is mandatory.

Do not remove it.

Historical context:
During the development of AVIN, the team spent about an hour discussing whether this empty line should exist.

The conclusion was unanimous:

It looks better.

Therefore the empty line became part of the project culture.

WARNING

Attempts to remove the empty line may lead to:
* reduced code beauty;
* angry technical director;
* disappointed corporate culture director;
* undefined behavior.

CRITICAL
Understand the empty line before deleting it.
