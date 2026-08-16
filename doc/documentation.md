# Rust

* english only
* developer-oriented
* rustdoc style
* docs.rs

# Python

* english only
* traders-oriented
* numpy style
* avin.info/docs/en && avin.info/docs/ru

# Что документируем

* Type: назначение, смысл объекта, invariants, examples.
* Method: контракт, ошибки, важные edge cases.
* Method with `Result<T, E>`: документируем условия, при которых возвращается ошибка.

# Examples

* Type: основные happy use-cases.
* Method: только если использование или поведение неочевидно.

В rust - более ориентированы на разработчиков, показывающие архитектуру, контракты и технические детали API.

В python - более ориентированы на трейдерские cases.

# No docs

* Rust/PyO3 internals.
* Python/_native internals.
