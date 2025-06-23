/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the instrument category.
///
/// # ru
/// Перечисление для выбора категории инструмента.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, strum::Display)]
pub enum Category {
    SHARE,
}
impl Category {
    pub fn name(&self) -> &str {
        match self {
            Self::SHARE => "SHARE",
        }
    }
}
impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "SHARE" => Category::SHARE,
            _ => panic!("Invalid category: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Category::SHARE.to_string(), "SHARE");
    }
}
