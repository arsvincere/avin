use pyo3::prelude::*;

use avin::BarKind;

#[pyclass(
    name = "BarKind",
    eq,
    from_py_object,
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyBarKind {
    Bull,
    Doji,
    Bear,
}

impl From<BarKind> for PyBarKind {
    fn from(value: BarKind) -> Self {
        match value {
            BarKind::Bull => Self::Bull,
            BarKind::Doji => Self::Doji,
            BarKind::Bear => Self::Bear,
        }
    }
}

impl From<PyBarKind> for BarKind {
    fn from(value: PyBarKind) -> Self {
        match value {
            PyBarKind::Bull => Self::Bull,
            PyBarKind::Doji => Self::Doji,
            PyBarKind::Bear => Self::Bear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(BarKind::Bull.to_string(), "Bull");
        assert_eq!(BarKind::Doji.to_string(), "Doji");
        assert_eq!(BarKind::Bear.to_string(), "Bear");
    }
}
