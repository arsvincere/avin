# Principle

avin             — idiomatic Rust API
binds/_native    — тупой и минимальный transport/FFI API
python/avin      — idiomatic Python API

avin
    public Rust API
          ↓ полное зеркало
binds / avin._native
    technical Python representation of Rust API
          ↓ выборочно
python/avin
    idiomatic public Python API
