/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode, Clone, strum::Display)]
pub enum Category {
    SHARE,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        let value = value.to_uppercase();

        match value.as_str() {
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
        let c = Category::SHARE;
        assert_eq!(c.to_string(), String::from("SHARE"));
    }
}
